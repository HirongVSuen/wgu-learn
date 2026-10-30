//! 练习 shader1：vertex / fragment shader
//!
//! 阶段：2（三角形 + shader）
//! 前置：01_clear
//! 依赖：wgpu（WGSL，无 CPU 侧额外依赖）
//!
//! 学习思路
//! WGSL 用 `@vertex`/`@fragment` 标注入口，`@builtin(position)` 输出裁剪坐标。
//! 先改片元颜色验证 shader 能编译，再谈顶点属性。
//!
//! 资料
//! - WGSL 规范: https://www.w3.org/TR/WGSL/
//! - learn-wgpu（pipeline 章节）: https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/
//!
//! 验收
//! - 编写 vertex/fragment shader
//! - 改 shader 能改颜色

const SHADER: &str = r#"
// TODO: 在这里写 WGSL
"#;

fn main() {
    // TODO: 把上面的 WGSL 编译为 ShaderModule 验证语法。
    todo!("编写 WGSL 并验证");
}
