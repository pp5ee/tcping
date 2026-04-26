# TCPing CLI Compatibility Validation Test Plan

## Overview
This test plan validates that the Rust tcping implementation has 100% CLI compatibility with the original Go version.

## Original Go Flags (19 total)
1. `-4` - IPv4 only
2. `-6` - IPv6 only  
3. `-r <n>` - Retry hostname resolution after n failures
4. `-c <n>` - Stop after n probes
5. `-j` - JSON output
6. `--pretty` - Pretty JSON (requires -j)
7. `--no-color` - Disable colors
8. `-D` - Show timestamp
9. `--csv <file>` - Save to CSV
10. `-v` - Show version
11. `-u` - Check updates
12. `-i <secs>` - Probe interval
13. `-t <secs>` - Probe timeout
14. `--db <file>` - Save to SQLite
15. `-I <interface>` - Bind to interface
16. `--show-source-address` - Show source addr/port
17. `--show-failures-only` - Show only failures
18. `-h` - Show help
19. `host:port` format support

## Test Categories

### 1. Basic Flag Presence Tests
```bash
# Test each flag exists and responds
./tcping-go -h
./tcping-rs -h

./tcping-go -v
./tcping-rs -v

./tcping-go -4 google.com 80
./tcping-rs -4 google.com 80

# ... test each flag
```

### 2. Flag Combination Tests
```bash
# Test valid combinations
./tcping-go -4 -c 5 -i 0.5 google.com 80
./tcping-rs -4 -c 5 -i 0.5 google.com 80

# Test invalid combinations
./tcping-go -4 -6 google.com 80  # Should error
./tcping-rs -4 -6 google.com 80  # Should error
```

### 3. Input Format Tests
```bash
# Host:port format
./tcping-go google.com:80
./tcping-rs google.com:80

# Separate host port
./tcping-go google.com 80
./tcping-rs google.com 80
```

### 4. Output Format Tests
```bash
# JSON output
./tcping-go -j -c 3 google.com 80 | jq .
./tcping-rs -j -c 3 google.com 80 | jq .

# Pretty JSON
./tcping-go -j --pretty -c 3 google.com 80
./tcping-rs -j --pretty -c 3 google.com 80

# CSV output
./tcping-go --csv test.csv -c 3 google.com 80
./tcping-rs --csv test.csv -c 3 google.com 80
```

### 5. Behavioral Tests
```bash
# Retry resolution
./tcping-go -r 3 invalid-host 80
./tcping-rs -r 3 invalid-host 80

# Interface binding
./tcping-go -I eth0 google.com 80
./tcping-rs -I eth0 google.com 80

# Show source address
./tcping-go --show-source-address google.com 80
./tcping-rs --show-source-address google.com 80
```

### 6. Error Handling Tests
```bash
# Invalid inputs
./tcping-go invalid-host invalid-port
./tcping-rs invalid-host invalid-port

# Missing required args
./tcping-go
./tcping-rs
```

## Test Execution Commands

Build both implementations:
```bash
# Build Go version
go build -o tcping-go tcping.go

# Build Rust version
cd tcping-rs
cargo build --release
cp target/release/tcping-rs ../tcping-rs
cd ..
```

Run automated comparison:
```bash
#!/bin/bash
# cli_comparison.sh

echo "=== Basic Flag Tests ==="
for flag in "-h" "-v" "-4" "-6"; do
    echo "Testing flag: $flag"
    ./tcping-go $flag google.com 80 2>&1 | head -5
    ./tcping-rs $flag google.com 80 2>&1 | head -5
    echo "---"
done
```

## Missing Implementation Status

**Flags missing in Rust version:**
- [ ] `-r` (retry resolution)
- [ ] `--pretty` (pretty JSON)
- [ ] `-D` (timestamp)
- [ ] `-u` (check updates)
- [ ] `--db` (SQLite database)
- [ ] `-I` (interface binding)
- [ ] `--show-source-address`
- [ ] `--show-failures-only`
- [ ] `host:port` format parsing

**Behavioral gaps:**
- [ ] Error message formatting
- [ ] Color output consistency
- [ ] JSON output field names/format
- [ ] CSV output format
- [ ] Update checking logic

## Validation Checklist

- [ ] All 19 flags exist and work
- [ ] Flag combinations behave identically
- [ ] Output formats match exactly
- [ ] Error messages are consistent
- [ ] Help text is comprehensive
- [ ] Input parsing is compatible
- [ ] Exit codes match for same conditions
