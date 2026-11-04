//! 练习 shader1：vertex / fragment shader
//!
//! 阶段：2（三角形 + shader）
//! 前置：01_clear
//! 依赖：wgpu（WGSL，无 CPU 侧额外依赖）
//!
//! 步骤
//! 1. 写 WGSL：`@vertex` 返回 `@builtin(position)`，`@fragment` 返回颜色。
//! 2. 先用硬编码三角形顶点（在 vertex 里直接返回坐标）。
//! 3. 用 `device.create_shader_module` 编译，看是否有语法错误。
//! 4. 改 fragment 颜色，验证修改能生效。
//!
//! 资料
//! - WGSL 规范: https://www.w3.org/TR/WGSL/
//! - learn-wgpu（pipeline 章节）: https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/
//!
//! 验收
//! - 编写 vertex/fragment shader
//! - 改 shader 能改颜色
//!
//! 总结
//! 会用 WGSL 写最小 vertex/fragment，理解 `@builtin(position)` 输出裁剪坐标。

const SHADER: &str = r#"
// WGSL 已由 src/wgsl/shader.wgsl 承载，供 pipeline1 使用。
// 学习者可在此自行覆写验证语法。
"#;

fn main() {
    // WGSL 已由 src/wgsl/shader.wgsl 承载，ShaderModule 创建见 src/app/renderer.rs。
    // 实现完成后运行 `cargo run --bin shader1`，check() 会打印自查要点。
    check();
}

/// 自检：打印验证要点（shader 语法 + 颜色）。
fn check() {
    println!("自查清单（shader1）：");
    println!("  - WGSL 能通过 create_shader_module 编译");
    println!("  - 修改 fragment 颜色后画面颜色随之改变");
}
