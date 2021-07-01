use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
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

pub trait ApplicationWrapper<T: Application> {
    fn run(self);
    fn call_render(&mut self, renderer: &mut Renderer<T::TextureId>);
}

impl<T: 'static> ApplicationWrapper<T> for T
where
    T: Application,
{
    fn call_render(&mut self, renderer: &mut Renderer<T::TextureId>) {
        renderer.clear();
        renderer.next_frame();
        self.render(renderer);
        renderer.done();
    }

    fn run(mut self) {
        let ev = EventLoop::new();
        let wb = self.window(WindowBuilder::new());
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &ev).unwrap();
        let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
        let mut renderer = Renderer::new(display, program);

        self.init(&mut renderer);
        if RENDER_ONCE {
            self.call_render(&mut renderer);
        }

        ev.run(move |event, _, control_flow| {
            *control_flow = match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => ControlFlow::Exit,
                    WindowEvent::Resized(..) => ControlFlow::Poll,
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
                        self.call_render(&mut renderer);
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
