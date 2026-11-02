use winit::{
    application::ApplicationHandler,
    event::WindowEvent::{CloseRequested, RedrawRequested, Resized},
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

// 创建event_loop 并配置
pub fn event_loop() -> EventLoop<()> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    event_loop
}

// event_loop 运行 applicationHandler
pub fn event_loop_run_app<App: ApplicationHandler>(app: &mut App) {
    let event_loop = event_loop();
    event_loop.run_app(app);
}

// 定义简单的App，使用event_loop运行
#[derive(Default)]
pub struct SimpleApp {
    pub simple_window: Option<Window>,
}

impl ApplicationHandler for SimpleApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // 创建窗口
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();
        self.simple_window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            CloseRequested => {
                event_loop.exit();
            }
            Resized(size) => {
                if size.width == 0 || size.height == 0 {
                    return;
                }
            }
            RedrawRequested => {}
            _ => {}
        }
    }
}

pub fn log_init() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
}
