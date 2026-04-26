# Requirement

帮我把这个项目重写成 Rust 版本，并且完成后打包发行 release 到这个分支。Rust 版本必须保持与原项目完全相同的命令行接口和行为；只支持 Ubuntu release，不再需要 macOS 版本；使用交叉编译工具链；发布产物使用 tar 包，不需要 deb 包；代码写完并确认没有问题后，自动创建 GitHub release 并发布到对应 repo 的 release。

---

## Implementation Notes

- For any unspecified details (combat formulas, game balance, UI layout, tech choices, etc.), make reasonable decisions yourself and document them in the plan. Do NOT ask the user for clarification — proceed with sensible defaults.
- If referenced image files exist in the workspace, treat them as visual style references.

## Standard Deliverables (mandatory for every project)

- **README.md** — must be included at the project root with: project title & description, prerequisites, installation steps, usage examples with code snippets, configuration options, and project structure overview.
- **Git commits** — use conventional commit prefix `feat:` for all commits.
