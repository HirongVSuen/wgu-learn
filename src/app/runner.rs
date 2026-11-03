use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::handler::AppHandler;

// 创建并返回一个等待事件的事件循环。
pub fn event_loop() -> EventLoop<()> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop
}

// 使用 env_logger 设置日志。
pub fn set_log() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}

// 使用事件循环运行应用。
pub fn run() -> anyhow::Result<()> {
    set_log();
    let event_loop = event_loop();
    let mut app_handler = AppHandler::default();
    event_loop.run_app(&mut app_handler)?;
    Ok(())
}
