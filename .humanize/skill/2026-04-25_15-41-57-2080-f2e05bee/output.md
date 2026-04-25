Based on my analysis of the Go TCPing implementation, here's a comprehensive breakdown of the key functionality that needs to be preserved in the Rust rewrite:

## **Core TCP Connection Logic and RTT Measurement**

**Essential Components:**
- **TCP Connection Establishment**: Uses `net.DialTimeout` with configurable timeout
- **RTT Measurement**: Measures time from connection start to successful establishment
- **Interface Binding**: Supports binding to specific network interfaces via `-I` flag
- **IPv4/IPv6 Support**: Dual-stack support with `-4` and `-6` flags for protocol selection
- **Connection Reuse**: Single connection per probe (no connection pooling)

**Measurement Details:**
- RTT calculated in milliseconds with nanosecond precision
- Timeout handling with configurable duration (default: 1 second)
- Interval between probes configurable (default: 1 second)

## **Command-Line Interface Structure**

**Required Flags:**
- `-4` / `-6`: IPv4/IPv6 only mode
- `-r <n>`: Retry hostname resolution after N failed probes
- `-c <n>`: Stop after N probes
- `-i <seconds>`: Interval between probes (float)
- `-t <seconds>`: Timeout duration (float)
- `-I <interface>`: Bind to specific interface
- `-j`: JSON output format
- `-pretty`: Pretty JSON formatting
- `-no-color`: Disable colored output
- `-D`: Show timestamps
- `-csv <file>`: Save to CSV
- `-db <file>`: Save to SQLite database
- `-show-source-address`: Display source address
- `-show-failures-only`: Only show failed probes
- `-v`: Version information
- `-u`: Check for updates
- `-h`: Help message

## **Output Formatting and Statistics Calculation**

**Statistics Tracked:**
- Total successful/unsuccessful probes
- Packet loss percentage
- Uptime/downtime duration
- Longest uptime/downtime periods
- Min/Avg/Max RTT values
- Last successful/unsuccessful probe timestamps

**Output Formats:**
- **Colored Console Output**: Real-time probe status with color coding
- **JSON Output**: Structured data with optional pretty printing
- **CSV Export**: Tabular data storage
- **SQLite Database**: Persistent storage option

**Real-time Features:**
- Enter key press triggers statistics display
- Signal handling (SIGINT/SIGTERM) for graceful shutdown
- Continuous operation until stopped or probe limit reached

## **Error Handling Patterns**

**Key Error Scenarios:**
- **Connection Timeouts**: Handled via configurable timeout
- **Hostname Resolution**: Automatic retry after N failures
- **Interface Binding Failures**: Fallback to default interface
- **File Output Errors**: Graceful degradation to console output

**Retry Mechanisms:**
- Hostname resolution retry with exponential backoff
- Connection failure tracking with streak counters
- State management for uptime/downtime transitions

## **Platform-Specific Considerations**

**Cross-Platform Compatibility:**
- Uses standard Go net package (platform-agnostic)
- Signal handling works across Unix/Linux/macOS/Windows
- File path handling with OS-specific separators

**Dependencies:**
- **net/netip**: Modern IP address handling
- **bufio**: Standard input monitoring
- **time**: Precise timing and duration calculations
- **flag**: Command-line argument parsing
- **github.com/gookit/color**: Cross-platform colored output
- **encoding/json**: JSON serialization

## **Critical Architecture Decisions for Rust Implementation**

**1. Core Architecture:**
- Maintain single-threaded, event-driven design
- Use async/await for non-blocking I/O
- Preserve the ticker-based probe scheduling

**2. Data Structures:**
- Port `tcping` struct with all tracking fields
- Maintain `userInput` configuration structure
- Keep `rttResult` and `longestTime` calculations

**3. Printer Interface:**
- Abstract output formatting behind trait/interface
- Support multiple output formats simultaneously
- Maintain real-time vs. batch output modes

**4. Signal Handling:**
- Implement cross-platform signal handling
- Graceful shutdown with statistics display
- Resource cleanup on termination

**5. Error Propagation:**
- Use Rust's Result type for error handling
- Maintain comprehensive error contexts
- Preserve retry logic with proper backoff

The Go implementation is well-structured with clear separation of concerns, making it an excellent candidate for a Rust rewrite that can leverage Rust's type safety, performance, and modern async capabilities while preserving all existing functionality.
