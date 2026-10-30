//! 练习 depth_linear：深度非线性（数学 M1）
//!
//! 对应阶段 4 + 数学 M1：
//! - 齐次坐标为什么存在
//! - 透视投影与 NDC
//! - 深度为什么非线性、近平面如何影响精度
//! - 画出“顶点 → 裁剪空间 → NDC → 屏幕”流程图
//!
//! 提示：透视投影后 depth 用 1/z 近似，所以近处精度高、远处精度低。

/// 把相机空间的深度 `z`（负数，-near..=-far）映射到 NDC [0,1]。
fn perspective_depth_ndc(z: f32, near: f32, far: f32) -> f32 {
    // TODO: 用标准透视投影的深度映射公式实现。
    todo!("透视投影深度映射")
}

fn main() {
    let near = 0.1;
    let far = 100.0;
    // 采样几个深度，观察 NDC 值在近处变化快、远处变化慢。
    for z in [0.11f32, 0.5, 1.0, 10.0, 50.0, 99.0] {
        let _ndc = perspective_depth_ndc(-z, near, far);
        // TODO: 打印并观察非线性。
    }
}
