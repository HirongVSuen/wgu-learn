# wgpu_learn

基于 `wgpu` 的图形学学习练习项目。每个阶段都是一个独立的二进制示例，可以直接 `cargo run` 运行。

## 环境与版本

| 依赖 | 锁定版本 | 说明 |
| --- | --- | --- |
| `wgpu` | `30.0.1` | 图形 API（Vulkan / Metal / DX12 / GL / WebGPU） |
| `winit` | `0.30.13` | 窗口与事件（0.30 使用 `ApplicationHandler`） |
| `glam` | `0.34.1` | 3D 数学库（矩阵/向量/四元数） |
| `pollster` | `1.0.1` | 同步阻塞执行异步初始化 |
| `env_logger` | `0.11.11` | 日志输出（配合 `log`） |

支撑依赖：`log = "0.4"`、`bytemuck = { version = "1", features = ["derive"] }`。

工具链：Rust `1.95.0`（`edition = "2024"`）。

## 运行

```sh
# 打印练习索引（阶段 0 验收）
cargo run

# 运行某个练习
cargo run --bin <练习名>
```

练习名与目录对应（`exercises/info.toml` 中可查全部）：

| 阶段 | 练习名 |
| --- | --- |
| 阶段0 | `setup1` `setup2` |
| 阶段1 | `clear1` `clear2` `clear3` |
| 阶段2 | `shader1` `pipeline1` `pipeline_manager` |
| 阶段3 | `view_matrix` `mvp` `uniform1` `bind_group_assets` |
| 阶段4 | `depth_linear` `texture1` `depth1` `asset_store` |
| 阶段5 | `project1` `ecs_world` |
| 阶段6 | `postprocess1` `blend1` `shadow1` `compute1` `render_graph` |
| 阶段7 | `brdf_diffuse` `brdf_ggx` `fresnel` `importance_sample` |
| 阶段8 | `input1` |

> 每个练习是带 TODO 的骨架；未填写的 `todo!()` 运行时会 panic，填完再运行即可。

## 各阶段对应检查项

- **阶段 0**：依赖锁定 + `cargo run` 跑通。
- **阶段 1**：`winit` 窗口（`CloseRequested` / `Resized` / `RedrawRequested`）、`wgpu` 初始化、Surface 配置、清屏、打印 adapter/backend、接入 `env_logger`。
- **阶段 2**：vertex/fragment shader、`ShaderModule`、`RenderPipeline`、`draw` 三角形、改 shader 颜色生效；后续 `pipeline_manager` 把管线创建重构为可复用缓存库。
- **阶段 3**：vertex/index buffer、uniform buffer + `BindGroup`、`glam` 算 MVP、`queue.write_buffer`、相机控制。
- **阶段 4**：纹理 + 采样器 + sRGB、深度纹理 + `DepthStencilState`、多个 3D 物体遮挡。
- **阶段 5**：3D 模型查看器 + 轨道相机，组合前面概念、封装 GPU 资源、可改参数。
- **阶段 6 / 7**：按需扩展（多 Pass / MSAA / 阴影 / Compute / wasm，以及 PBR / 网格 / 光追 / AA 方向），待选方向后补充。

> 阶段 2~6 每个阶段末尾新增一个「重构练习」（`pipeline_manager` / `bind_group_assets` /
> `asset_store` / `ecs_world` / `render_graph`）：把刚学会的能力重构为更工程化的形态，
> 巩固知识 + 练工程能力（对应「引擎化路线」里的方案一/二/三与阶段 C/D/E）。
- **阶段 8**：统一输入抽象（InputAdapter 维护按键状态 → InputMapper 解析组合/时序 → 纯业务 Action）。
