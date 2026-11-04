//! 练习 shadow1：阴影
//!
//! 阶段：6c（阴影）
//! 前置：05_project + 04 深度
//! 依赖：wgpu + glam
//!
//! 步骤
//! 1. 从光源视角渲染深度到 shadow map（光照空间矩阵）。
//! 2. 主 pass 里把世界坐标变换到光照空间，采样 shadow map。
//! 3. 用 `SamplerCompare` 做深度比较。
//! 4. 调 bias 消除阴影 acne。
//!
//! 资料
//! - LearnOpenGL（Shadow Mapping）: https://learnopengl.com/Advanced-Lighting/Shadows/Shadow-Mapping
//! - wgpu Sampler: https://docs.rs/wgpu/latest/wgpu/struct.Sampler.html
//!
//! 验收
//! - 深度 pass 生成 shadow map
//! - 光照空间矩阵
//! - SamplerCompare
//! - shadow bias 调参
//!
//! 总结
//! 掌握 shadow mapping 的两趟渲染与深度比较，理解 bias 的来源与作用。

fn main() {
    // TODO: shadow map → SamplerCompare → bias。
    // 实现完成后运行 `cargo run --bin shadow1`，check() 会打印自查要点。
    check();
}

/// 自检：打印验证要点（阴影是否出现）。
fn check() {
    println!("自查清单（shadow1）：");
    println!("  - 物体能投射出正确形状的阴影");
    println!("  - SamplerCompare 深度比较正常");
    println!("  - 调 bias 后阴影 acne 明显减少");
}
