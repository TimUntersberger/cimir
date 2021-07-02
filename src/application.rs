use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent, MouseButton},
    event_loop::ControlFlow,
    event_loop::EventLoop,
    window::WindowBuilder,
};

use glium::{glutin::ContextBuilder, Display, Program};

use std::hash::Hash;

use crate::renderer::Renderer;
use crate::shaders::{FRAGMENT_SHADER, VERTEX_SHADER};

/// Used for debugging
const RENDER_ONCE: bool = false;

pub trait Application {
    type TextureId: Eq + Hash;

    fn init(&mut self, _renderer: &mut Renderer<Self::TextureId>) {}
    fn render(&mut self, renderer: &mut Renderer<Self::TextureId>);
    fn window(&mut self, w: WindowBuilder) -> WindowBuilder {
        w
    }
    fn on_event(
        &mut self,
        _event: Event<()>,
        _r: &mut Renderer<Self::TextureId>,
    ) -> Option<ControlFlow> {
        None
    }
    fn on_key_down(
        &mut self,
        _key: VirtualKeyCode,
        _r: &mut Renderer<Self::TextureId>,
    ) -> Option<ControlFlow> {
        None
    }
    fn on_key_up(
        &mut self,
        _key: VirtualKeyCode,
        _r: &mut Renderer<Self::TextureId>,
    ) -> Option<ControlFlow> {
        None
    }
}

pub struct MouseInfo {
    pub x: f64,
    pub y: f64,
    pub lmouseclick: bool,
    pub rmouseclick: bool
}

pub trait ApplicationWrapper<T: Application> {
    fn run(self);
    fn call_render(&mut self, renderer: &mut Renderer<T::TextureId>, mouse: &mut MouseInfo);
}

impl<T: 'static> ApplicationWrapper<T> for T
where
    T: Application,
{
    fn call_render(&mut self, renderer: &mut Renderer<T::TextureId>, mouse: &mut MouseInfo) {
        renderer.clear();
        renderer.next_frame();
        renderer.active_id = renderer.hitboxes
            .iter()
            .find(|(_, hb)| hb.contains_pos(mouse.x as f32, mouse.y as f32))
            .map(|(id, _)| *id);
        mouse.lmouseclick = false;
        mouse.rmouseclick = false;
        renderer.clear_hitboxes();
        self.render(renderer);
        renderer.done();
    }

    fn run(mut self) {
        let ev = EventLoop::new();
        let wb = self.window(WindowBuilder::new());
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &ev).unwrap();
        let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
        let mut mouse_info = MouseInfo {
            x: 0.0,
            y: 0.0,
            lmouseclick: false,
            rmouseclick: false,
        };
        let mut renderer = Renderer::new(display, program);

        self.init(&mut renderer);
        if RENDER_ONCE {
            self.call_render(&mut renderer, &mut mouse_info);
        }

        ev.run(move |event, _, control_flow| {
            *control_flow = match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => ControlFlow::Exit,
                    WindowEvent::Resized(..) => ControlFlow::Poll,
                    WindowEvent::CursorMoved { position, .. } => {
                        mouse_info.x = position.x;
                        mouse_info.y = position.y;
                        ControlFlow::Poll
                    },
                    WindowEvent::MouseInput { state, button, .. } => {
                        match (state, button) {
                            (ElementState::Released, MouseButton::Left) => mouse_info.lmouseclick = true,
                            (ElementState::Released, MouseButton::Right) => mouse_info.lmouseclick = true,
                            _ => {}
                        };
                        ControlFlow::Poll
                    },
                    WindowEvent::KeyboardInput { input, .. } => input
                        .virtual_keycode
                        .and_then(|key| match input.state {
                            ElementState::Pressed => self.on_key_down(key, &mut renderer),
                            ElementState::Released => self.on_key_up(key, &mut renderer),
                        })
                        .unwrap_or(ControlFlow::Poll),
                    _ => ControlFlow::Poll,
                },
                Event::MainEventsCleared => {
                    if !RENDER_ONCE {
                        self.call_render(&mut renderer, &mut mouse_info);
                    }
                    ControlFlow::Poll
                }
                _ => ControlFlow::Poll,
            };

            if let Some(cf) = self.on_event(event, &mut renderer) {
                *control_flow = cf;
            }
        });
    }
}
