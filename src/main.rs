use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::time::{Duration, Instant};
use std::process;
use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize)]
struct PingResult {
    host: String,
    port: u16,
    success: bool,
    duration_ms: Option<u128>,
    error: Option<String>,
}

fn resolve_host(host: &str, port: u16) -> Result<Vec<SocketAddr>, Box<dyn std::error::Error>> {
    let addr_string = format!("{}:{}", host, port);
    match addr_string.to_socket_addrs() {
        Ok(addrs) => Ok(addrs.collect()),
        Err(e) => Err(Box::new(e)),
    }
}

fn tcp_ping(host: &str, port: u16, timeout_secs: u64) -> Result<Duration, Box<dyn std::error::Error>> {
    let addrs = resolve_host(host, port)?;

    // Try each resolved address until one succeeds
    for addr in addrs {
        let start = Instant::now();
        match TcpStream::connect_timeout(&addr, Duration::from_secs(timeout_secs)) {
            Ok(_) => return Ok(start.elapsed()),
            Err(_) => continue, // Try next address
        }
    }

    Err(Box::new(io::Error::new(
        io::ErrorKind::ConnectionRefused,
        "Failed to connect to any resolved address",
    )))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("tcping")
        .version("0.1.0")
        .author("Your Name")
        .about("TCP connectivity testing tool")
        .arg(Arg::new("host")
            .required(true)
            .help("Target hostname or IP address"))
        .arg(Arg::new("port")
            .required(true)
            .help("Target port number"))
        .arg(Arg::new("timeout")
            .short('t')
            .long("timeout")
            .value_name("SECONDS")
            .default_value("5")
            .help("Connection timeout in seconds"))
        .arg(Arg::new("count")
            .short('c')
            .long("count")
            .value_name("COUNT")
            .help("Number of pings to send"))
        .arg(Arg::new("json")
            .long("json")
            .help("Output results in JSON format"))
        .get_matches();

    let host = matches.get_one::<String>("host").unwrap();
    let port: u16 = matches.get_one::<String>("port").unwrap().parse()?;
    let timeout: u64 = matches.get_one::<String>("timeout").unwrap().parse()?;
    let count: Option<usize> = matches.get_one::<String>("count").map(|s| s.parse().unwrap_or(1));
    let json_output = matches.get_flag("json");

    // Validate inputs
    if port == 0 {
        return Err("Port number must be between 1 and 65535".into());
    }

    if timeout == 0 {
        return Err("Timeout must be greater than 0".into());
    }

    let ping_count = count.unwrap_or(1);
    let mut successful_pings = 0;
    let mut total_duration = Duration::new(0, 0);

    for i in 0..ping_count {
        match tcp_ping(host, port, timeout) {
            Ok(duration) => {
                successful_pings += 1;
                total_duration += duration;

                if json_output {
                    let ping_result = PingResult {
                        host: host.clone(),
                        port,
                        success: true,
                        duration_ms: Some(duration.as_millis()),
                        error: None,
                    };
                    println!("{}", serde_json::to_string(&ping_result)?);
                } else {
                    println!("Ping {}: Connected to {}:{} - time={}ms", i + 1, host, port, duration.as_millis());
                }
            }
            Err(e) => {
                if json_output {
                    let ping_result = PingResult {
                        host: host.clone(),
                        port,
                        success: false,
                        duration_ms: None,
                        error: Some(e.to_string()),
                    };
                    println!("{}", serde_json::to_string(&ping_result)?);
                } else {
                    eprintln!("Ping {}: Failed to connect to {}:{} - {}", i + 1, host, port, e);
                    if ping_count == 1 {
                        process::exit(1);
                    }
                }
            }
        }
    }

    // Summary for multiple pings
    if ping_count > 1 && !json_output {
        println!("\n--- {}:{} ping statistics ---", host, port);
        println!("{} packets transmitted, {} successful, {}% packet loss",
                 ping_count, successful_pings,
                 ((ping_count - successful_pings) * 100) / ping_count);

        if successful_pings > 0 {
            let avg_duration = total_duration / successful_pings as u32;
            println!("round-trip min/avg/max = {0}/{1}/{0} ms", avg_duration.as_millis());
        }
    }

    Ok(())
}