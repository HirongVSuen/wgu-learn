//! 练习 depth_linear：深度非线性（数学 M1）
//!
//! 阶段：4（纹理 + 深度）
//! 前置：03_camera（已有 NDC 概念）
//! 依赖：无（纯 Rust）
//!
//! 步骤
//! 1. 推导/回忆透视投影的深度映射（NDC z 与 1/z 近似线性）。
//! 2. 实现 `perspective_depth_ndc`。
//! 3. 采样多个深度打印，观察近处变化快、远处慢。
//! 4. 画“顶点 → 裁剪空间 → NDC → 屏幕”流程图。
//!
//! 资料
//! - LearnOpenGL（深度测试）: https://learnopengl.com/Advanced-OpenGL/Depth-testing
//! - Scratchapixel（投影/NDC）: https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/
//!
//! 验收
//! - 能解释齐次坐标、NDC、深度非线性
//! - 能画出“顶点 → 裁剪空间 → NDC → 屏幕”流程图
//!
//! 总结
//! 理解深度非线性与近平面对精度的影响，为后续 z-fighting 建立直觉。

/// 把相机空间的深度 `z`（负数，-near..=-far）映射到 NDC [0,1]。
fn perspective_depth_ndc(z: f32, near: f32, far: f32) -> f32 {
    // TODO: 用标准透视投影的深度映射公式实现。
    todo!("透视投影深度映射")
}

/// 自检：验证端点映射（directx 约定：near→0，far→1）。
fn check() {
    let (near, far) = (0.1, 100.0);
    let ndc_near = perspective_depth_ndc(-near, near, far);
    let ndc_far = perspective_depth_ndc(-far, near, far);

    let ok = (ndc_near - 0.0).abs() < 1e-3 && (ndc_far - 1.0).abs() < 1e-3;
    if ok {
        println!("✅ 通过：near→0，far→1，深度映射正确。");
    } else {
        println!("❌ 未通过：near={ndc_near:.3}，far={ndc_far:.3}。");
        println!("   提示：directx 约定 near 映射到 0、far 映射到 1。");
    }
}

fn main() {
    check();
}
