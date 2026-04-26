# TCPing Rust Rewrite Completion and Release Automation

## Goal Description
Complete the Rust rewrite of the TCPing tool by finalizing cross-compilation for Ubuntu, creating tar package releases, and implementing GitHub release automation. The Rust version must maintain identical CLI interface and behavior as the original Go version, with support for Ubuntu-only releases using cross-compilation toolchain.

## Acceptance Criteria

Following TDD philosophy, each criterion includes positive and negative tests for deterministic verification.

- AC-1: Rust implementation maintains 100% CLI compatibility with original Go version
  - Positive Tests (expected to PASS):
    - All CLI flags and arguments from Go version work identically in Rust version
    - Output format and structure matches Go version for equivalent commands
    - Error messages and exit codes are consistent with Go version
  - Negative Tests (expected to FAIL):
    - Rust version introduces new CLI flags not present in Go version
    - Output format differs from Go version for same input parameters
    - Error handling behavior diverges from Go version

- AC-2: Cross-compilation produces working Ubuntu binaries
  - Positive Tests (expected to PASS):
    - Binary compiles successfully for x86_64-unknown-linux-gnu target
    - Compiled binary runs on Ubuntu 20.04+ without dependencies
    - Binary passes basic functionality tests (help, version, connectivity)
  - Negative Tests (expected to FAIL):
    - Cross-compilation fails due to missing dependencies or toolchain
    - Binary fails to execute on target Ubuntu system
    - Binary requires additional runtime dependencies not present in Go version

- AC-3: Tar package creation and distribution
  - Positive Tests (expected to PASS):
    - tar.gz package contains binary and basic documentation
    - Package can be extracted and binary runs without installation
    - Package naming follows convention: tcping_{version}_{arch}.tar.gz
  - Negative Tests (expected to FAIL):
    - Package contains unnecessary files or directories
    - Binary requires special installation steps beyond extraction
    - Package naming convention is inconsistent

- AC-4: GitHub release automation
  - Positive Tests (expected to PASS):
    - Release process automatically creates GitHub release with correct version
    - Release includes Ubuntu tar package as asset
    - Release notes include basic changelog and installation instructions
  - Negative Tests (expected to FAIL):
    - Release process requires manual intervention beyond triggering
    - Release assets are missing or incorrectly packaged
    - Release notes are incomplete or inaccurate

- AC-5: Functional parity with Go version
  - Positive Tests (expected to PASS):
    - All core TCP ping functionality works identically to Go version
    - RTT measurement accuracy matches Go version within acceptable margin
    - Statistics calculation (min/avg/max, packet loss) is consistent
  - Negative Tests (expected to FAIL):
    - TCP connectivity behavior differs from Go version
    - RTT measurements show systematic deviation from Go version
    - Statistics calculations produce different results for same input

## Path Boundaries

Path boundaries define the acceptable range of implementation quality and choices.

### Upper Bound (Maximum Acceptable Scope)
The implementation includes full cross-compilation setup for Ubuntu, automated tar package creation, GitHub release automation with proper changelog generation, comprehensive testing against the original Go version, and performance benchmarking to validate improvements. All build scripts are optimized and documented, with CI/CD integration for automated releases.

### Lower Bound (Minimum Acceptable Scope)
The implementation includes basic cross-compilation for Ubuntu that produces a working binary, manual tar package creation, and manual GitHub release creation. Core functionality is validated to match the Go version, but advanced features like automated CI/CD and performance benchmarking are deferred.

### Allowed Choices
- Can use: Existing Rust toolchain and build system, GitHub Actions for CI/CD, standard tar packaging format, cross-compilation toolchains available in Rust ecosystem
- Cannot use: macOS-specific packaging, deb package format, non-standard CLI interfaces, breaking changes to existing Rust codebase structure

> **Note on Deterministic Designs**: The draft specifies highly deterministic requirements including Ubuntu-only support, tar packaging, and cross-compilation. The path boundaries reflect these fixed constraints while allowing flexibility in implementation details.

## Feasibility Hints and Suggestions

> **Note**: This section is for reference and understanding only. These are conceptual suggestions, not prescriptive requirements.

### Conceptual Approach
1. **Validate Current Implementation**: Test existing Rust code against Go version to identify any functional gaps
2. **Cross-Compilation Setup**: Configure Cargo for Ubuntu target (x86_64-unknown-linux-gnu) with proper toolchain
3. **Package Creation**: Create tar.gz packages containing binary and basic documentation
4. **Release Automation**: Implement GitHub Actions workflow for automated releases
5. **Validation**: Comprehensive testing to ensure parity with Go version

### Relevant References
- `src/` - Rust source code implementation
- `Makefile.rust` - Existing cross-compilation and packaging setup
- `Cargo.toml` - Rust project configuration and dependencies
- `.github/workflows/` - GitHub Actions configuration (to be created)
- `README-RUST-REWRITE.md` - Documentation for Rust version

## Dependencies and Sequence

### Milestones
1. **Milestone 1**: Validate Rust Implementation Completeness
   - Phase A: Functional testing against Go version
   - Phase B: CLI interface compatibility verification
   - Phase C: Performance and accuracy benchmarking

2. **Milestone 2**: Cross-Compilation and Packaging
   - Step 1: Configure Ubuntu cross-compilation toolchain
   - Step 2: Create tar package creation scripts
   - Step 3: Test package installation and execution

3. **Milestone 3**: Release Automation
   - Step 1: Implement GitHub Actions release workflow
   - Step 2: Configure automated package upload
   - Step 3: Test end-to-end release process

### Dependencies
- Milestone 2 depends on Milestone 1 completion (validated implementation)
- Milestone 3 depends on Milestone 2 completion (working packages)
- All milestones depend on existing Rust codebase stability

## Task Breakdown

Each task must include exactly one routing tag:
- `coding`: implemented by Claude
- `analyze`: executed via Codex (`/humanize:ask-codex`)

| Task ID | Description | Target AC | Tag (`coding`/`analyze`) | Depends On |
|---------|-------------|-----------|----------------------------|------------|
| task1 | Analyze current Rust implementation for functional gaps | AC-1, AC-5 | analyze | - |
| task2 | Test CLI compatibility between Rust and Go versions | AC-1 | coding | task1 |
| task3 | Configure Ubuntu cross-compilation toolchain | AC-2 | coding | task2 |
| task4 | Create tar package creation and validation scripts | AC-3 | coding | task3 |
| task5 | Implement GitHub Actions release automation | AC-4 | coding | task4 |
| task6 | Validate end-to-end release process | AC-4 | analyze | task5 |
| task7 | Update documentation and README files | - | coding | task6 |

## Claude-Codex Deliberation

### Agreements
- The Rust rewrite is already substantially complete on the tcping-rust branch
- Core TCP ping functionality appears to be implemented and working
- The project structure is well-organized with proper separation of concerns
- Cross-platform support is partially implemented but needs Ubuntu-specific focus

### Resolved Disagreements
- **Scope Interpretation**: Codex initially interpreted the draft as requiring a complete Rust rewrite from scratch, while Claude recognized the work is mostly done and focused on completion tasks. Resolution: Focus on completing release automation and validation rather than reimplementing core functionality.
- **Package Format**: Codex suggested maintaining both deb and tar formats, while Claude followed the draft's explicit requirement for tar-only packaging. Resolution: Implement tar packaging as specified in the draft.

### Convergence Status
- Final Status: `partially_converged` (due to direct mode skipping iterative refinement)

## Pending User Decisions

- DEC-1: <Decision topic>
  - Claude Position: <...>
  - Codex Position: <...>
  - Tradeoff Summary: <...>
  - Decision Status: `PENDING` or `<User's final decision>`

## Implementation Notes

### Code Style Requirements
- Implementation code and comments must NOT contain plan-specific terminology such as "AC-", "Milestone", "Step", "Phase", or similar workflow markers
- These terms are for plan documentation only, not for the resulting codebase
- Use descriptive, domain-appropriate naming in code instead

## Output File Convention

This template is used to produce the main output file (e.g., `plan.md`).

### Translated Language Variant

When `alternative_plan_language` resolves to a supported language name through merged config loading, a translated variant of the output file is also written after the main file. Humanize loads config from merged layers in this order: default config, optional user config, then optional project config; `alternative_plan_language` may be set at any of those layers. The variant filename is constructed by inserting `_<code>` (the ISO 639-1 code from the built-in mapping table) immediately before the file extension:

- `plan.md` becomes `plan_<code>.md` (e.g. `plan_zh.md` for Chinese, `plan_ko.md` for Korean)
- `docs/my-plan.md` becomes `docs/my-plan_<code>.md`
- `output` (no extension) becomes `output_<code>`

The translated variant file contains a full translation of the main plan file's current content in the configured language. All identifiers (`AC-*`, task IDs, file paths, API names, command flags) remain unchanged, as they are language-neutral.

When `alternative_plan_language` is empty, absent, set to `"English"`, or set to an unsupported language, no translated variant is written. Humanize does not auto-create `.humanize/config.json` when no project config file is present.

--- Original Design Draft Start ---

# Requirement

帮我把这个项目重写成 Rust 版本，并且完成后打包发行 release 到这个分支。Rust 版本必须保持与原项目完全相同的命令行接口和行为；只支持 Ubuntu release，不再需要 macOS 版本；使用交叉编译工具链；发布产物使用 tar 包，不需要 deb 包；代码写完并确认没有问题后，自动创建 GitHub release 并发布到对应 repo 的 release。

---

## Implementation Notes

- For any unspecified details (combat formulas, game balance, UI layout, tech choices, etc.), make reasonable decisions yourself and document them in the plan. Do NOT ask the user for clarification — proceed with sensible defaults.
- If referenced image files exist in the workspace, treat them as visual style references.

## Standard Deliverables (mandatory for every project)

- **README.md** — must be included at the project root with: project title & description, prerequisites, installation steps, usage examples with code snippets, configuration options, and project structure overview.
- **Git commits** — use conventional commit prefix `feat:` for all commits.

--- Original Design Draft End ---
