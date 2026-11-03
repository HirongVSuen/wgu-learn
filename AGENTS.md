# AGENTS.md

本文件是给在该仓库工作的 AI 编码代理（以及协作者）的项目约定说明。

## 项目是什么

`wgpu_learn` 是一个基于 `wgpu` 的图形学**学习练习项目**。每个练习是
`exercises/` 下带 `TODO` / `todo!()` 的教学骨架，作为独立二进制目标，
用 `cargo run --bin <name>` 直接运行。

## 核心约定：练习骨架机制

- `exercises/<阶段目录>/<练习名>.rs` 是**带 TODO 的骨架**，不是成品。
- AI 代理的职责通常是 **补全 TODO**，而不是推翻重写整个文件。
- 保留每个练习内的 `check()` 自检清单和文档注释（`//!` 步骤/验收/总结）。
- 不要把 `todo!()` 替换成空实现敷衍了事，要真正实现对应概念。
- 未填写的 `todo!()` 运行时会 panic，这是预期行为，不是 bug。
- 分两类练习，骨架形态不同：
  - **纯计算/数学类**（阶段 3/4/7 的 `view_matrix`、`mvp`、`depth_linear`、`brdf_*` 等）：
    练习文件里就是裸露的 `todo!()` 函数，填了即完成。
  - **图形类**（clear/pipeline/texture/depth/compute 等）：练习文件的 `main` 只调共用的
    `run()`，**真正的实现落在 `src/app/`**（`handler.rs` / `renderer.rs` / `runner.rs`）。
- 图形练习的**完成标志**是：其能力已体现在 `src/app/` 的对应模块里，而不是练习文件里
  是否存在 `todo!()`。已完成练习的 `main` 会留一行注释指向 `src/app/<模块>.rs` 的实现位置；
  重构时按该指针定位能力，不要靠猜。

## 运行与验证

```sh
cargo run                # 打印练习索引（阶段 0 验收）
cargo run --bin <练习名>  # 运行某个练习，例如 `cargo run --bin clear1`
cargo check              # 快速编译检查（推荐用 `--bin <name>` 缩小范围）
```

- 修改后请至少 `cargo check`（或按需 `cargo run --bin <name>`）验证能通过编译。
- `wgpu` 是图形程序，运行会打开窗口 / 需要 GPU；CI 或纯编译场景用 `cargo check`。

## 检查与验收

- 收尾前，依据每个练习的 `check()` 自检清单做最终检查总结。
- `check()` 分两类，验收方式不同：
  - **带断言**的（如 `view_matrix`、`mvp`、`depth_linear`、`brdf_*`、`fresnel`、
    `importance_sample`）：`check()` 里已含可运行断言，实现是否填了 `todo!()` 即可判定，
    真跑过一次更能确认通过。
  - **纯打印提示**的（窗口 / 清屏 / 绘制等视觉类）：属视觉/行为验收，静态检查只能判断
    「是否仍是 `todo!()` 骨架」，真正通过需实际 `cargo run --bin <name>` 观察。
- 总结时明确区分「已实现」与「仍为骨架」两部分；没有实际运行时，不要只凭代码就下「通过」结论。

## 目录清单（重要，先看这里）

本仓库是 Obsidian Vault 的一部分，PARA 结构跨**两个 workspace 内外**的层级。
所有归档/进度文件的**绝对路径**如下，agent 请直接按此定位，不要靠猜：

```text
# Obsidian Vault 根
C:\Users\Microsoft\Desktop\valut\wgpu\wgpu\wgpu\

# ——PARA 目录（在 Vault 根下）——
C:\Users\Microsoft\Desktop\valut\wgpu\wgpu\wgpu\00 Inbox\
C:\Users\Microsoft\Desktop\valut\wgpu\wgpu\wgpu\01 Projects\
C:\Users\Microsoft\Desktop\valut\wgpu\wgpu\wgpu\02 Areas\
C:\Users\Microsoft\Desktop\valut\wgpu\wgpu\wgpu\03 Resources\
C:\Users\Microsoft\Desktop\valut\wgpu\wgpu\wgpu\04 Archives\

# ——本仓库（Rust 项目，Zed workspace 根）——
C:\Users\Microsoft\Desktop\valut\wgpu\wgpu\wgpu\01 Projects\exmaple\wgpu_learn\

# ——归档目录（已完成练习的笔记，按练习名一文件）——
C:\Users\Microsoft\Desktop\valut\wgpu\wgpu\wgpu\04 Archives\已完成项目\wgpu_learn\

# ——进度笔记（checkbox 勾选全部练习）——
C:\Users\Microsoft\Desktop\valut\wgpu\wgpu\wgpu\01 Projects\exmaple\wgpu_learn 进度.md
```

关键注意：
- **Zed workspace 根**只有 `...\01 Projects\exmaple\wgpu_learn\` 这层；
  `04 Archives\` 与 `01 Projects\exmaple\` 都在 workspace 根**之外**。
- 因此写归档/进度笔记不能用 Zed 的 `write_file`（够不着），要走终端（见下节）。

## 归档（PARA）

本项目是 Obsidian Vault 的一部分，采用 PARA 目录结构。一个练习验收通过后要归档：

- **归档目录**：`04 Archives/已完成项目/wgpu_learn/`，**按练习名一文件**（如 `clear1.md`），
  内容包含：`check()` 清单逐项对照、实现位置（通常在 `src/lib.rs`）、步骤对照、验收结论。
- **进度笔记**：`01 Projects/exmaple/wgpu_learn 进度.md`，用 checkbox 列出全部练习并勾选已完成项。
- 归档只**写笔记记录**，不移动 Rust 源文件（源文件需留在原地才能继续编译运行后续练习）。
- 归档笔记的文件内容不经过 Zed 的 `write_file`（因为路径在 workspace 根之外），
  需先在 workspace 内写好临时文件，再用终端 `cp` 复制到归档位置后清理临时文件。

## 目录与清单

- 练习按阶段放在 `exercises/` 下的子目录（`00_setup`、`01_clear` …）。
- `exercises/info.toml` 是所有练习的**清单**（id / stage / tags / description）。
- **新增或改名练习时，必须同步三处**：
  1. `Cargo.toml` 里的 `[[bin]]`（name + path）；
  2. `exercises/info.toml` 里的 `[[exercises]]`；
  3. `README.md` 里的练习名表格（如涉及阶段归属）。

## 版本与工具链约定

- 依赖在 `Cargo.toml` 里**锁定精确版本**（`wgpu = "=30.0.1"` 等），不要擅自改版本。
- 改版本时，`README.md` 的「环境与版本」表要**同步更新**保持一致。
- 工具链：Rust `1.95.0`；`edition = "2024"`。
- 支撑依赖注释掉的部分（`log`、`bytemuck`）在真正用到前保持注释状态。

## 代码风格

- 遵循项目内已存在的 Rust 风格与命名，保持改动最小、聚焦当前练习。
- 日志统一用 `env_logger` + `log`（见阶段 1 的 `clear3`），不要引入其他日志方案。
- 数学库统一用 `glam`（矩阵 / 向量 / 四元数）。
- 不要为澄清而添加多余注释；注释只解释非显而易见的意图、约束或权衡。
- 注释一律用**中文**（与仓库现状一致）；文档/注释里保留的专有名词（如 `wgpu`、`winit`）与链接标题不翻译。

## 提交

- 提交信息遵循：标题首字母大写、祈使语气、不超过 50 字符、结尾不加标点；
  正文仅在提供有用信息时添加，并与标题分行。
