# CLI Compatibility Comparison: Rust vs Go TCPing

## Overview
This document provides a comprehensive comparison between the Rust TCPing implementation (v2.7.1) and the original Go implementation to validate 100% CLI compatibility.

## Critical Compatibility Issues Identified

### Major Incompatibilities (Blocking AC-1)

#### 1. `-v` Flag Conflict (CRITICAL)
- **Go Implementation**: `-v` shows version information
- **Rust Implementation**: `-v` controls verbosity level (0-3)
- **Impact**: Complete behavioral mismatch - breaks CLI compatibility
- **Resolution Required**: Rust must use `-v` for version, find alternative for verbosity

#### 2. Version Flag Discrepancy (HIGH)
- **Go Implementation**: Uses `-v` for version display
- **Rust Implementation**: Uses `-V` for version display  
- **Impact**: Different flag for same functionality
- **Resolution Required**: Standardize on `-v` for version across both implementations

#### 3. JSON Output Flag (MEDIUM)
- **Go Implementation**: Uses `-j` for JSON output
- **Rust Implementation**: Uses `--json` (no short flag)
- **Impact**: Breaks script compatibility
- **Resolution Required**: Add `-j` short flag to Rust implementation

#### 4. Retry Resolution Flag (MEDIUM)
- **Go Implementation**: Uses `-r <n>` for retry resolution
- **Rust Implementation**: Uses `--retry-resolution <n>` (no short flag)
- **Impact**: Breaks script compatibility
- **Resolution Required**: Add `-r` short flag to Rust implementation

### Minor Differences (Non-Blocking)

#### 5. Default Timeout (LOW)
- **Go Implementation**: Default timeout = 1 second
- **Rust Implementation**: Default timeout = 5 seconds
- **Impact**: Behavioral difference but not breaking
- **Resolution**: Consider standardizing on 1 second for compatibility

## Comprehensive Feature Comparison Table

| Feature | Go Implementation | Rust Implementation | Status | Priority |
|---------|------------------|---------------------|--------|----------|
| Required Args | `host port` | `HOST PORT` | ✅ Identical | - |
| Timeout | `-t <seconds>` (1s) | `-t, --timeout` (5s) | ⚠️ Different default | Low |
| Interval | `-i <seconds>` (1s) | `-i, --interval` (1s) | ✅ Identical | - |
| Count | `-c <n>` | `-c, --count <n>` | ✅ Identical | - |
| IPv4 Only | `-4` | `-4, --ipv4` | ✅ Identical | - |
| IPv6 Only | `-6` | `-6, --ipv6` | ✅ Identical | - |
| Interface | `-I <interface>` | `-I, --interface` | ✅ Identical | - |
| JSON Output | `-j` | `--json` | ❌ Different flag | Medium |
| Pretty JSON | `--pretty` | `--pretty` | ✅ Identical | - |
| CSV Output | `--csv <file>` | `--csv <file>` | ✅ Identical | - |
| SQLite DB | `--db <file>` | `--db <file>` | ✅ Identical | - |
| No Color | `--no-color` | `--no-color` | ✅ Identical | - |
| Timestamps | `-D` | `-D, --timestamps` | ✅ Identical | - |
| Show Source | `--show-source-address` | `--show-source-address` | ✅ Identical | - |
| Show Failures Only | `--show-failures-only` | `--show-failures-only` | ✅ Identical | - |
| Retry Resolution | `-r <n>` | `--retry-resolution <n>` | ❌ Different flag | Medium |
| Verbose | (Not available) | `-v, --verbose...` | ❌ Conflict | Critical |
| Version | `-v` | `-V, --version` | ❌ Different flag | High |
| Check Updates | `-u` | `-u, --check-updates` | ✅ Identical | - |
| Help | `-h` | `-h, --help` | ✅ Identical | - |

## Validation Test Suite Requirements

### Prerequisites
- Go compiler available for building original implementation
- Both `go-tcping` and Rust `tcping` binaries built
- Network connectivity for functional testing

### Test Categories

#### 1. Basic CLI Functionality
- Help output comparison
- Version information comparison
- Required argument validation
- Invalid argument error handling

#### 2. Flag Compatibility Testing
- All flag combinations tested side-by-side
- Default value validation
- Short flag vs long flag equivalence
- Flag conflict detection

#### 3. Output Format Validation
- JSON output structure comparison
- CSV format compatibility
- SQLite database schema validation
- Text output formatting

#### 4. Behavioral Testing
- Network connectivity results
- Timing and timeout behavior
- Error handling consistency
- Exit code validation

## Implementation Priority

### Phase 1: Critical Fixes (Required for AC-1)
1. Fix `-v` flag conflict (version vs verbosity)
2. Standardize version flag usage (`-v` for both)
3. Add missing short flags (`-j` for JSON, `-r` for retry)

### Phase 2: Validation Framework
1. Create automated comparison scripts
2. Implement side-by-side testing
3. Document any remaining differences

### Phase 3: Comprehensive Testing
1. Run full test suite with both implementations
2. Validate all acceptance criteria
3. Generate compatibility report

## Conclusion

The Rust implementation is largely compatible but requires critical fixes to achieve 100% CLI compatibility. The `-v` flag conflict is the most significant issue that must be resolved before AC-1 can be considered met.