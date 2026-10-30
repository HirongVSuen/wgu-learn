//! 练习 clear2：wgpu 初始化与 Surface
//!
//! 阶段：1（窗口 + 清屏）
//! 前置：clear1
//! 依赖：wgpu + winit
//!
//! 步骤
//! 1. `Instance::new(InstanceDescriptor)`（wgpu 30 按值传参）。
//! 2. `instance.create_surface(window)` 得到 Surface。
//! 3. `request_adapter` 选适配器（带 compatible_surface）。
//! 4. `request_device` 得到 Device + Queue。
//! 5. 用 `SurfaceConfiguration`（含 color_space）configure surface。
//!
//! 资料
//! - wgpu Instance/Surface 文档: https://docs.rs/wgpu/
//! - learn-wgpu（初始化章节）: https://sotrh.github.io/learn-wgpu/
//!
//! 验收
//! - Instance → Adapter → Device/Queue
//! - 创建 Surface 与 SurfaceConfiguration
//!
//! 总结
//! 理清 Instance / Adapter / Device / Queue / Surface 各自的职责与创建顺序。
//!
//! 注意：wgpu 30 中 `Instance::new` 按值接收 `InstanceDescriptor`，
//! `SurfaceConfiguration` 需包含 `color_space` 字段。

fn main() {
    // TODO: 完成 wgpu 初始化并 configure surface。
    todo!("Instance → Adapter → Device/Queue → Surface");
}
