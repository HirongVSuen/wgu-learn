use anyhow::Context;
use log::error;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{
        ElementState, KeyEvent,
        WindowEvent::{self, CloseRequested, RedrawRequested, Resized},
    },
    keyboard::KeyCode,
    window::{Window, WindowAttributes},
};

use crate::app::renderer::Renderer;

// 应用处理器
#[derive(Default)]
pub struct AppHandler {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl AppHandler {
    // 尝试初始化应用，创建窗口和渲染器。
    fn try_init(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) -> anyhow::Result<()> {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .context("创建窗口失败")?,
        );
        let size = &window.inner_size();
        let renderer = Renderer::new(window.clone(), size.width, size.height);
        self.renderer = Some(pollster::block_on(renderer).context("创建渲染器失败")?);
        self.window = Some(window);
        Ok(())
    }
}

// 应用的窗口事件与应用状态
impl ApplicationHandler for AppHandler {
    // 窗口恢复时调用，尝试初始化应用。
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let init_result = self.try_init(event_loop);
        if let Err(e) = init_result {
            log::error!("初始化失败: {}", e);
            event_loop.exit();
        }
    }

    // 窗口事件处理，包括关闭和调整大小。
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let renderer = match &mut self.renderer {
            Some(renderer) => renderer,
            None => {
                event_loop.exit();
                return;
            }
        };
        match event {
            CloseRequested => {
                event_loop.exit();
            }
            Resized(size) => {
                if size.width == 0 || size.height == 0 {
                    return;
                }
                renderer.resize(size.width, size.height);
            }
            RedrawRequested => {
                renderer.update();
                match renderer.render() {
                    Ok(_) => {}
                    Err(e) => {
                        error!("渲染失败!{e}");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(KeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => {
                renderer.handle_keyboard_input(translate_keycode(code, state));
            }
            _ => {}
        }
    }
}

fn translate_keycode(code: KeyCode, state: ElementState) -> crate::core::action::Action {
    match (code, state) {
        _ => crate::core::action::Action::None,
    }
}
