//! 练习 clear2：wgpu 初始化与 Surface
//!
//! 阶段：1（窗口 + 清屏）
//! 前置：clear1
//! 依赖：wgpu + winit
//!
//! 学习思路
//! 理解 Instance→Adapter→Device/Queue 各自的职责；Surface 依附于窗口；
//! `SurfaceConfiguration` 决定交换链的格式/尺寸/present 模式。
//!
//! 资料
//! - wgpu Instance/Surface 文档: https://docs.rs/wgpu/
//! - learn-wgpu（初始化章节）: https://sotrh.github.io/learn-wgpu/
//!
//! 验收
//! - Instance → Adapter → Device/Queue
//! - 创建 Surface 与 SurfaceConfiguration
//!
//! 注意：wgpu 30 中 `Instance::new` 按值接收 `InstanceDescriptor`，
//! `SurfaceConfiguration` 需包含 `color_space` 字段。

fn main() {
    // TODO: 完成 wgpu 初始化并 configure surface。
    todo!("Instance → Adapter → Device/Queue → Surface");
}
