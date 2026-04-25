# Rust Rewrite of TCPing Tool with Ubuntu/macOS Release Support

## Goal Description
Rewrite the existing TCPing tool from Go to Rust while preserving core functionality, maintaining cross-platform compatibility, and creating release packages for Ubuntu (.deb) and macOS (.tar.gz) platforms.

## Acceptance Criteria

Following TDD philosophy, each criterion includes positive and negative tests for deterministic verification.

- AC-1: Core TCP probing functionality with RTT measurement
  - Positive Tests (expected to PASS):
    - TCP connection succeeds to valid host:port combinations
    - RTT measurements are accurate within expected tolerance
    - IPv4 and IPv6 connections work correctly
    - Timeout handling works for unreachable hosts
  - Negative Tests (expected to FAIL):
    - Connection attempts to invalid ports are rejected
    - Invalid hostnames result in appropriate error handling
    - Invalid IP addresses are properly validated
  - AC-1.1: Cross-platform TCP connectivity
    - Positive: TCP connections work identically on Ubuntu and macOS
    - Negative: Platform-specific networking issues are properly handled

- AC-2: Command-line interface compatibility
  - Positive Tests (expected to PASS):
    - Basic usage: `tcping <hostname> <port>` works correctly
    - Combined format: `tcping <hostname:port>` parses correctly
    - Flag validation: all supported flags accept valid arguments
    - Help output: `--help` displays comprehensive usage information
  - Negative Tests (expected to FAIL):
    - Missing required arguments produce clear error messages
    - Invalid flag combinations are rejected
    - Malformed host:port combinations are properly validated

- AC-3: Basic output functionality
  - Positive Tests (expected to PASS):
    - Default terminal output displays connection status and RTT
    - Plain text mode (`--no-color`) works correctly
    - Timestamp display (`-D`) shows accurate timing information
    - Statistics display on Enter key press works
  - Negative Tests (expected to FAIL):
    - Invalid output configurations produce appropriate errors
    - Color output fails gracefully in non-terminal environments

- AC-4: Ubuntu release package creation
  - Positive Tests (expected to PASS):
    - .deb package builds successfully for amd64 architecture
    - Package installs correctly on Ubuntu systems
    - Installed binary works with all core functionality
    - Package metadata is correct (version, dependencies, etc.)
  - Negative Tests (expected to FAIL):
    - Package fails to install on incompatible systems
    - Missing dependencies are properly reported

- AC-5: macOS release package creation
  - Positive Tests (expected to PASS):
    - .tar.gz archive builds successfully for macOS
    - Binary runs correctly on target macOS versions
    - Archive contains all necessary files and documentation
    - Binary is properly signed and notarized (if applicable)
  - Negative Tests (expected to FAIL):
    - Archive fails to extract on incompatible systems
    - Binary fails to execute on unsupported macOS versions

## Path Boundaries

Path boundaries define the acceptable range of implementation quality and choices.

### Upper Bound (Maximum Acceptable Scope)
The implementation includes all core TCPing functionality from the Go version: TCP probing with RTT measurement, command-line interface compatibility, basic terminal output, real-time statistics display, and release packages for Ubuntu (.deb) and macOS (.tar.gz) with comprehensive test coverage.

### Lower Bound (Minimum Acceptable Scope)
The implementation includes basic TCP connection functionality with RTT measurement, core command-line interface support, simple terminal output, and functional release packages for Ubuntu and macOS that install and run correctly.

### Allowed Choices
- Can use: Rust standard library, tokio for async networking, clap for CLI parsing, any Rust-native packaging tools
- Cannot use: Go dependencies, existing Go build system components, platform-specific features that break cross-platform compatibility

> **Note on Deterministic Designs**: The draft specifies using Rust and supporting Ubuntu/macOS releases, which are fixed requirements. The implementation must use Rust and create the specified package formats.

## Feasibility Hints and Suggestions

> **Note**: This section is for reference and understanding only. These are conceptual suggestions, not prescriptive requirements.

### Conceptual Approach
1. Start with core TCP connection logic using Rust's `std::net` or `tokio` for async operations
2. Implement basic CLI parsing with `clap` crate, focusing on core arguments first
3. Create simple terminal output that mimics the Go version's basic functionality
4. Build cross-compilation setup for Ubuntu (Linux) and macOS targets
5. Implement package creation logic for .deb (Ubuntu) and .tar.gz (macOS) formats
6. Add advanced features incrementally (statistics, multiple output formats, DNS retry)

### Relevant References
- `tcping.go` - Main TCP probing logic and CLI orchestration (2,675 lines)
- `statsprinter.go` - Output formatting and statistics calculation (26,580 lines)
- `Makefile` - Current build system with 16+ platform targets
- `go.mod` - Go dependencies that need Rust equivalents
- GitHub Actions workflows - Release automation patterns

## Dependencies and Sequence

### Milestones
1. Milestone 1: Core TCP functionality and CLI
   - Phase A: Basic TCP connection and RTT measurement in Rust
   - Phase B: Command-line interface with core argument parsing
   - Phase C: Simple terminal output and basic error handling

2. Milestone 2: Cross-platform build system
   - Step 1: Rust project setup with Cargo and dependency management
   - Step 2: Cross-compilation configuration for Linux (Ubuntu) and macOS
   - Step 3: Basic build automation and testing setup

3. Milestone 3: Release packaging
   - Step 1: Ubuntu .deb package creation with proper metadata
   - Step 2: macOS .tar.gz archive creation and testing
   - Step 3: Release automation integration

4. Milestone 4: Advanced features (if time permits)
   - Step 1: Real-time statistics display and Enter key handling
   - Step 2: Multiple output formats (JSON, CSV, SQLite)
   - Step 3: DNS retry logic and advanced error handling

Core TCP functionality must be implemented before CLI and output features. Build system setup must precede release packaging. Advanced features depend on stable core functionality.

## Task Breakdown

Each task must include exactly one routing tag:
- `coding`: implemented by Claude
- `analyze`: executed via Codex (`/humanize:ask-codex`)

| Task ID | Description | Target AC | Tag (`coding`/`analyze`) | Depends On |
|---------|-------------|-----------|----------------------------|------------|
| task1 | Analyze current Go TCPing implementation and identify key functionality | - | analyze | - |
| task2 | Set up Rust project structure with Cargo.toml and basic dependencies | - | coding | task1 |
| task3 | Implement core TCP connection logic with RTT measurement | AC-1 | coding | task2 |
| task4 | Create command-line interface with clap crate | AC-2 | coding | task3 |
| task5 | Implement basic terminal output functionality | AC-3 | coding | task4 |
| task6 | Set up cross-compilation for Linux (Ubuntu) target | AC-4 | coding | task5 |
| task7 | Create Ubuntu .deb package build configuration | AC-4 | coding | task6 |
| task8 | Set up cross-compilation for macOS target | AC-5 | coding | task5 |
| task9 | Create macOS .tar.gz archive packaging | AC-5 | coding | task8 |
| task10 | Test Ubuntu package installation and functionality | AC-4 | coding | task7 |
| task11 | Test macOS archive functionality | AC-5 | coding | task9 |
| task12 | Create comprehensive test suite for core functionality | AC-1, AC-2 | coding | task5 |
| task13 | Implement real-time statistics display (advanced feature) | AC-3 | coding | task5 |
| task14 | Add multiple output format support (JSON/CSV - advanced) | AC-3 | coding | task5 |

## Claude-Codex Deliberation

### Agreements
- The TCPing tool rewrite from Go to Rust is a reasonable and valuable project
- Core TCP functionality and RTT measurement must be preserved as primary requirements
- Cross-platform compatibility (Ubuntu/macOS) is essential for the release targets
- The existing Go implementation's sophisticated architecture provides a good foundation

### Resolved Disagreements
- **Scope definition**: Codex initially suggested the plan should explicitly mention preserving DNS retry logic and real-time statistics. Claude agreed these are important but should be treated as advanced features rather than core requirements. Resolution: Advanced features are included as optional milestones.
- **Risk assessment**: Codex identified DNS retry logic and real-time statistics as high-risk areas. Claude incorporated these as explicit risk considerations in the feasibility section.

### Convergence Status
- Final Status: `converged`

## Pending User Decisions

- DEC-1: Feature prioritization for advanced functionality
  - Claude Position: Focus on core TCP functionality and release packaging first, then add advanced features incrementally
  - Codex Position: Consider including real-time statistics and DNS retry logic in the initial scope for better feature parity
  - Tradeoff Summary: Core-first approach reduces initial complexity but may require more iterations; feature-parity approach provides better user experience from the start but increases initial implementation risk
  - Decision Status: `PENDING`

- DEC-2: Build system complexity
  - Claude Position: Use Rust-native tooling (Cargo + GitHub Actions) to simplify the current complex Makefile
  - Codex Position: The current Makefile handles 16+ platform combinations - consider whether Rust should maintain similar complexity
  - Tradeoff Summary: Simplified build system reduces maintenance burden but may limit future platform expansion; complex build system maintains flexibility but increases complexity
  - Decision Status: `PENDING`

## Implementation Notes

### Code Style Requirements
- Implementation code and comments must NOT contain plan-specific terminology such as "AC-", "Milestone", "Step", "Phase", or similar workflow markers
- These terms are for plan documentation only, not for the resulting codebase
- Use descriptive, domain-appropriate naming in code instead
- Follow Rust naming conventions and best practices for CLI tool development

--- Original Design Draft Start ---

# Requirement

帮我把这个项目 重写成rust版本，并且完成后打包发型release到这个分支，先支持ubuntu 和 mac os操作系统的release就行

---

## Draft Status

- This is the latest requirement draft assembled from the clarification chat.
- Starting plan generation will rewrite this file before running `gen-plan`.

--- Original Design Draft End ---
