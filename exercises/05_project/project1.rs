//! 练习 project1：小项目（3D 模型查看器 + 轨道相机）
//!
//! 阶段：5（小项目）
//! 前置：阶段 1~4 全部
//! 依赖：wgpu + winit + glam
//!
//! 步骤
//! 1. 复用一个最小框架（window / surface / device / queue）。
//! 2. 封装 buffer / texture / pipeline 的创建与持有。
//! 3. 实现轨道相机（俯仰 / 偏航 / 缩放）。
//! 4. 每帧写 MVP，渲染带深度 + 纹理的网格。
//! 5. 整理参数（FOV、物体位置、光照）便于修改。
//!
//! 资料
//! - learn-wgpu（整体串讲）: https://sotrh.github.io/learn-wgpu/
//! - 轨道相机（Arcball）: https://asliceofrendering.com/camera/2019/11/30/ArcballCamera/
//!
//! 验收
//! - 组合前面概念成可交互项目
//! - 封装 GPU 资源
//! - 能交互、能改参数、不崩
//!
//! 总结
//! 把阶段 1~4 串成可交互项目，重点在资源封装与参数化。
//!
//! 可选方向：3D 模型查看器 / 2D 精灵渲染器 / 粒子 demo。

fn main() {
    // TODO: 组合阶段 1~4，做一个带轨道相机的 3D 查看器。
    todo!("3D 模型查看器 + 轨道相机");
}
