use anyhow::Context;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::handler::AppHandler;

// 使用 env_logger 设置日志。
pub fn set_log() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}

// 使用事件循环运行应用。
pub fn run() -> anyhow::Result<()> {
    set_log();

    let event_loop = EventLoop::new().context("创建eventLoop失败！")?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app_handler = AppHandler::default();
    event_loop
        .run_app(&mut app_handler)
        .context("启动程序失败！")?;

    Ok(())
}
