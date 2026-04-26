# TCPing Rust Implementation Validation and Refinement

## Goal Description
Validate and refine the existing Rust TCPing implementation to ensure it meets the draft requirements: 100% CLI compatibility with the original tool, Ubuntu-only deployment with cross-compilation, tar package creation, and automated GitHub releases. The implementation already exists and needs verification rather than new development.

## Acceptance Criteria

Following TDD philosophy, each criterion includes positive and negative tests for deterministic verification.

- AC-1: CLI Compatibility Validation
  - Positive Tests (expected to PASS):
    - All existing CLI flags and options work identically to original tool
    - Command-line interface maintains 100% behavioral parity
    - Help output matches original tool's format and content
  - Negative Tests (expected to FAIL):
    - Any CLI flag produces different behavior than original
    - Help output shows missing or incorrect options
    - Argument parsing fails for valid original tool inputs

- AC-2: Cross-Platform Support Configuration
  - Positive Tests (expected to PASS):
    - Cross-compilation targets only x86_64-unknown-linux-gnu
    - Build scripts produce Ubuntu-compatible binaries
    - Release artifacts are tar packages without macOS support
  - Negative Tests (expected to FAIL):
    - Build process includes macOS targets or artifacts
    - Package creation generates deb packages instead of tar
    - Cross-compilation fails for Ubuntu target

- AC-3: Release Automation Validation
  - Positive Tests (expected to PASS):
    - GitHub Actions workflow triggers on tag releases
    - Release process creates and uploads tar packages
    - Automated release notes are generated correctly
  - Negative Tests (expected to FAIL):
    - Release workflow fails to trigger on tag creation
    - Package creation fails during automation
    - Release assets are missing or incorrect

- AC-4: Code Quality and Documentation
  - Positive Tests (expected to PASS):
    - Rust code compiles without warnings or errors
    - README provides clear installation and usage instructions
    - Documentation matches original tool's functionality
  - Negative Tests (expected to FAIL):
    - Code contains unused macOS-specific configurations
    - Documentation is incomplete or inaccurate
    - Build process produces warnings or errors
## Path Boundaries

Path boundaries define the acceptable range of implementation quality and choices.

### Upper Bound (Maximum Acceptable Scope)
The implementation undergoes comprehensive validation including CLI compatibility testing against the original tool, full cross-compilation verification, automated release process testing, and complete documentation review. All macOS support is removed, and the codebase is thoroughly cleaned up.

### Lower Bound (Minimum Acceptable Scope)
Basic validation that the existing Rust implementation meets the core requirements: Ubuntu-only deployment works, tar packages are created, and GitHub release automation functions. Minimal cleanup of obvious macOS configurations.

### Allowed Choices
- Can use: Existing Rust codebase, current GitHub Actions workflow, existing build scripts
- Cannot use: macOS deployment, deb package creation, changes to CLI interface that break compatibility

> **Note on Deterministic Designs**: The draft specifies highly deterministic requirements (Ubuntu-only, tar packages, 100% CLI compatibility), so the path boundaries are narrow and focused on validation rather than new implementation.

## Feasibility Hints and Suggestions

> **Note**: This section is for reference and understanding only. These are conceptual suggestions, not prescriptive requirements.

### Conceptual Approach
The existing implementation is already complete. The approach should focus on validation and refinement:
1. Test CLI compatibility by comparing with original TCPing tool
2. Verify cross-compilation produces Ubuntu-only binaries
3. Validate GitHub release automation workflow
4. Clean up macOS-specific configurations
5. Update documentation to reflect current state

### Relevant References
- `src/main.rs` - Main entry point for CLI parsing
- `src/cli.rs` - CLI argument definitions and parsing logic
- `build-ubuntu-tar.sh` - Ubuntu package creation script
- `.github/workflows/release.yml` - GitHub release automation
- `Cargo.toml` - Project configuration and dependencies

## Dependencies and Sequence

### Milestones
1. **Validation Phase**: Verify existing implementation meets requirements
   - Phase A: CLI compatibility testing
   - Phase B: Cross-compilation validation
   - Phase C: Release automation testing

2. **Refinement Phase**: Clean up and document the implementation
   - Step 1: Remove macOS-specific configurations
   - Step 2: Update documentation and README
   - Step 3: Final validation and testing

Dependencies: Validation must complete before refinement begins, as refinement decisions depend on validation findings.

## Task Breakdown

Each task must include exactly one routing tag:
- `coding`: implemented by Claude
- `analyze`: executed via Codex (`/humanize:ask-codex`)

| Task ID | Description | Target AC | Tag (`coding`/`analyze`) | Depends On |
|---------|-------------|-----------|----------------------------|------------|
| task1 | Analyze existing implementation and validate CLI compatibility | AC-1 | analyze | - |
| task2 | Verify cross-compilation configuration and Ubuntu-only deployment | AC-2 | analyze | task1 |
| task3 | Test GitHub release automation workflow functionality | AC-3 | analyze | task2 |
| task4 | Remove macOS-specific configurations and clean up codebase | AC-2 | coding | task3 |
| task5 | Update documentation and README to reflect current implementation | AC-4 | coding | task4 |
| task6 | Final validation testing and release preparation | AC-1, AC-3 | analyze | task5 |

## Claude-Codex Deliberation

### Agreements
- The repository already contains a complete Rust implementation matching the draft requirements
- This is a validation and refinement task rather than new development
- Focus should be on CLI compatibility testing and cleanup

### Resolved Disagreements
- **Task Nature**: Codex identified this as validation, Claude agreed - resolved to focus on verification rather than implementation
- **Scope**: Both agree the implementation exists and needs refinement rather than creation

### Convergence Status
- Final Status: `converged`

## Pending User Decisions

- DEC-1: What is the original TCPing tool being compared against for CLI compatibility?
  - Claude Position: Need to identify the specific original tool version for accurate comparison
  - Codex Position: N/A - open question
  - Tradeoff Summary: Without knowing the original tool, CLI compatibility validation cannot be comprehensive
  - Decision Status: `PENDING`

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
