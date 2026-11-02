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

use wgpu_learn::{SimpleApp, event_loop_run_app, log_init};
use winit::event_loop::ActiveEventLoop;

// newType方式对SimpleApp做功能覆盖
struct WgpuSimpleApp {
    simple_app: Option<SimpleApp>,
}

impl winit::application::ApplicationHandler for WgpuSimpleApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.simple_app.as_mut().unwrap().resumed(event_loop);
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..wgpu::InstanceDescriptor::new_without_display_handle()
        });
        let surface = instance
            .create_surface(
                self.simple_app
                    .as_ref()
                    .unwrap()
                    .simple_window
                    .as_ref()
                    .unwrap(),
            )
            .unwrap();
        // let adapter = instance
        //     .request_adapter(&wgpu::RequestAdapterOptions {
        //         power_preference: wgpu::PowerPreference::HighPerformance,
        //         compatible_surface: Some(&surface),
        //         force_fallback_adapter: false,
        //         apply_limit_buckets: false,
        //     })
        //     .await
        //     .unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.simple_app
            .as_mut()
            .unwrap()
            .window_event(event_loop, _window_id, event);
    }
}

fn main() {
    log_init();
    let mut app = WgpuSimpleApp {
        simple_app: Some(SimpleApp::default()),
    };
    event_loop_run_app(&mut app);
    // TODO: 完成 wgpu 初始化并 configure surface。
    // 实现完成后运行 `cargo run --bin clear2`，check() 会打印自查要点。
    check();
}

/// 自检：打印验证要点（初始化成功后应有日志输出）。
fn check() {
    println!("自查清单（clear2）：");
    println!("  - 初始化不 panic，能拿到 Device + Queue");
    println!("  - Surface 成功 configure（含 color_space）");
    println!("  - 后续能打印 adapter 名称与 backend");
}
