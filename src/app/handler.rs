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

// wgpu 学习应用的应用处理器
#[derive(Default)]
pub struct AppHandler {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

// 处理 wgpu 学习应用的窗口事件与应用状态
impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .unwrap();
        self.window = Some(Arc::new(window));
        let size = self.window.as_ref().unwrap().inner_size();
        let renderer = Renderer::new(
            self.window.as_ref().unwrap().clone(),
            size.width,
            size.height,
        );
        self.renderer = Some(pollster::block_on(renderer));
    }

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
                        error!("{e}");
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
                        state: state,
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
