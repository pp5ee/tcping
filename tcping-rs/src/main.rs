//! TCPing CLI tool - main entry point

use clap::{Arg, Command};
use std::process;
use std::time::Duration;

use tcping_rs::{Tcping, TcpingConfig, ProbeResult, ProbeStatistics};
use chrono;  // For CSV timestamp formatting

#[tokio::main]
async fn main() {
    let matches = Command::new("tcping")
        .version("0.1.0")
        .author("tcping-rs")
        .about("A TCP ping tool rewritten in Rust")
        .arg(
            Arg::new("host")
                .help("Hostname or IP address to probe")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("port")
                .help("Port number to probe")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Stop after sending N probes")
                .value_name("N")
                .default_value("0"),
        )
        .arg(
            Arg::new("interval")
                .short('i')
                .long("interval")
                .help("Wait interval seconds between sending each probe")
                .value_name("SECONDS")
                .default_value("1.0"),
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .help("Timeout in seconds for each probe")
                .value_name("SECONDS")
                .default_value("1.0"),
        )
        .arg(
            Arg::new("ipv4")
                .short('4')
                .long("ipv4")
                .help("Use IPv4 only")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ipv6")
                .short('6')
                .long("ipv6")
                .help("Use IPv6 only")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("Output in JSON format")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("csv")
                .long("csv")
                .help("Output in CSV format to specified file")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .help("Disable color output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Parse host and port
    let host = matches.get_one::<String>("host").unwrap().to_string();
    let port_str = matches.get_one::<String>("port").unwrap();

    let port = match port_str.parse::<u16>() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Error: Invalid port number '{}'", port_str);
            process::exit(1);
        }
    };

    // Parse numeric arguments
    let count = match matches.get_one::<String>("count").unwrap().parse::<usize>() {
        Ok(c) => if c == 0 { None } else { Some(c) },
        Err(_) => {
            eprintln!("Error: Invalid count value");
            process::exit(1);
        }
    };

    let interval_secs = match matches.get_one::<String>("interval").unwrap().parse::<f64>() {
        Ok(i) if i > 0.0 => i,
        _ => {
            eprintln!("Error: Interval must be a positive number");
            process::exit(1);
        }
    };

    let timeout_secs = match matches.get_one::<String>("timeout").unwrap().parse::<f64>() {
        Ok(t) if t >= 0.0 => t,
        _ => {
            eprintln!("Error: Timeout must be a non-negative number");
            process::exit(1);
        }
    };

    // Create configuration
    let config = TcpingConfig {
        host: host.clone(),
        port,
        timeout: Duration::from_secs_f64(timeout_secs),
        use_ipv4: matches.get_flag("ipv4"),
        use_ipv6: matches.get_flag("ipv6"),
    };

    // Validate IP version selection
    if config.use_ipv4 && config.use_ipv6 {
        eprintln!("Error: Cannot use both IPv4 and IPv6 flags simultaneously");
        process::exit(1);
    }

    let tcping = Tcping::new(config);
    let output_json = matches.get_flag("json");
    let output_csv = matches.get_one::<String>("csv").map(|s| s.as_str());
    let no_color = matches.get_flag("no-color");

    // Run the TCPing session
    if let Err(e) = run_tcping_session(tcping, count, Duration::from_secs_f64(interval_secs), output_json, output_csv, no_color, &host, port).await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

async fn run_tcping_session(
    tcping: Tcping,
    count: Option<usize>,
    interval: Duration,
    output_json: bool,
    output_csv: Option<&str>,
    no_color: bool,
    host: &str,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::time::sleep;
    use std::sync::{Arc, Mutex};
    use tokio::signal;
    use tokio::io::{self, AsyncBufReadExt};

    println!("TCPinging {} on port {}", host, port);

    let stats = Arc::new(Mutex::new(ProbeStatistics::default()));
    let mut probe_count = 0;

    // Setup CSV output if requested
    let csv_writer = if let Some(csv_file) = output_csv {
        match std::fs::File::create(csv_file) {
            Ok(file) => {
                let mut wtr = csv::Writer::from_writer(file);
                // Write CSV header
                if let Err(e) = wtr.write_record(&["timestamp", "sequence", "success", "rtt_ms", "error"]) {
                    eprintln!("Warning: Failed to write CSV header: {}", e);
                }
                Some(Arc::new(Mutex::new(wtr)))
            }
            Err(e) => {
                eprintln!("Error: Failed to create CSV file '{}': {}", csv_file, e);
                None
            }
        }
    } else {
        None
    };

    // Handle Ctrl+C for graceful shutdown
    let stats_clone = stats.clone();
    let host_clone = host.to_string();
    let csv_writer_clone = csv_writer.clone();
    tokio::spawn(async move {
        signal::ctrl_c().await.ok();
        print_statistics(&stats_clone.lock().unwrap(), &host_clone, port, output_json, no_color);

        // Close CSV file if open
        if let Some(writer) = csv_writer_clone {
            if let Ok(mut wtr) = writer.lock() {
                let _ = wtr.flush();
            }
        }

        process::exit(0);
    });

    // Handle Enter key for real-time statistics
    let stats_enter = stats.clone();
    let host_enter = host.to_string();
    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            if line.trim().is_empty() {
                // Enter key pressed - show real-time statistics
                print_statistics(&stats_enter.lock().unwrap(), &host_enter, port, output_json, no_color);
            }
        }
    });

    loop {
        let result = tcping.probe().await;
        probe_count += 1;

        {
            let mut stats_guard = stats.lock().unwrap();
            stats_guard.update(&result);
        }

        // Print result
        print_probe_result(&result, probe_count, no_color);

        // Write to CSV if requested
        if let Some(writer) = &csv_writer {
            if let Ok(mut wtr) = writer.lock() {
                let timestamp = chrono::Utc::now().to_rfc3339();
                let success = if result.success { "true" } else { "false" };
                let rtt = result.rtt.map_or("".to_string(), |r| r.to_string());
                let error = result.error.unwrap_or_default();

                if let Err(e) = wtr.write_record(&[&timestamp, &probe_count.to_string(), success, &rtt, &error]) {
                    eprintln!("Warning: Failed to write CSV record: {}", e);
                }
            }
        }

        // Check if we should stop
        if let Some(max_count) = count {
            if probe_count >= max_count {
                break;
            }
        }

        sleep(interval).await;
    }

    // Print final statistics
    print_statistics(&stats.lock().unwrap(), host, port, output_json, no_color);

    // Close CSV file if open
    if let Some(writer) = csv_writer {
        if let Ok(mut wtr) = writer.lock() {
            let _ = wtr.flush();
        }
    }

    Ok(())
}

fn print_probe_result(result: &ProbeResult, sequence: usize, no_color: bool) {
    use colored::*;

    if no_color {
        if result.success {
            if let Some(rtt) = result.rtt {
                println!("Reply from {} TCP_conn={} time={:.3} ms", sequence, sequence, rtt);
            }
        } else {
            println!("No reply from {} TCP_conn={}", sequence, sequence);
        }
    } else {
        if result.success {
            if let Some(rtt) = result.rtt {
                println!(
                    "{}",
                    format!("Reply from {} TCP_conn={} time={:.3} ms", sequence, sequence, rtt)
                        .green()
                );
            }
        } else {
            println!(
                "{}",
                format!("No reply from {} TCP_conn={}", sequence, sequence).red()
            );
        }
    }
}

fn print_statistics(stats: &ProbeStatistics, host: &str, port: u16, output_json: bool, no_color: bool) {
    use colored::*;
    use serde_json::json;

    if output_json {
        let json_output = json!({
            "host": host,
            "port": port,
            "statistics": {
                "total_probes": stats.total_probes,
                "successful_probes": stats.successful_probes,
                "failed_probes": stats.failed_probes,
                "packet_loss": stats.packet_loss,
                "min_rtt": stats.min_rtt,
                "max_rtt": stats.max_rtt,
                "avg_rtt": stats.avg_rtt
            }
        });
        println!("{}", json_output);
        return;
    }

    if no_color {
        println!("\n--- {} TCPing statistics ---", host);
        println!("{} probes transmitted on port {} | {} received, {:.2}% packet loss",
                stats.total_probes, port, stats.successful_probes, stats.packet_loss);
        println!("successful probes:   {}", stats.successful_probes);
        println!("unsuccessful probes: {}", stats.failed_probes);

        if let Some(min) = stats.min_rtt {
            println!("rtt min/avg/max: {:.3}/{:.3}/{:.3} ms",
                    min, stats.avg_rtt.unwrap_or(0.0), stats.max_rtt.unwrap_or(0.0));
        }
    } else {
        println!("\n{}", format!("--- {} TCPing statistics ---", host).yellow());

        let packet_loss_text = format!("{} probes transmitted on port {} | {} received, ",
                                      stats.total_probes, port, stats.successful_probes);

        let loss_percentage = if stats.packet_loss == 0.0 {
            format!("{:.2}%", stats.packet_loss).green()
        } else if stats.packet_loss <= 30.0 {
            format!("{:.2}%", stats.packet_loss).yellow()
        } else {
            format!("{:.2}%", stats.packet_loss).red()
        };

        println!("{}{} packet loss", packet_loss_text.yellow(), loss_percentage);
        println!("{} {}", "successful probes:".yellow(), stats.successful_probes.to_string().green());
        println!("{} {}", "unsuccessful probes:".yellow(), stats.failed_probes.to_string().red());

        if let Some(min) = stats.min_rtt {
            let rtt_text = format!("rtt min/avg/max: {:.3}/{:.3}/{:.3} ms",
                                  min, stats.avg_rtt.unwrap_or(0.0), stats.max_rtt.unwrap_or(0.0));

            // Colorize different parts of RTT
            let parts: Vec<&str> = rtt_text.split('/').collect();
            if parts.len() == 3 {
                println!("{} {}/{} {}/{} {}",
                        "rtt".yellow(),
                        parts[0].split(' ').last().unwrap_or("").green(),
                        parts[1].cyan(),
                        parts[2].split(' ').next().unwrap_or("").red(),
                        parts[2].split(' ').nth(1).unwrap_or(""),
                        parts[2].split(' ').nth(2).unwrap_or(""));
            }
        }
    }
}
