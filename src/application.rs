use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent, MouseButton},
    event_loop::ControlFlow,
    event_loop::EventLoop,
    window::WindowBuilder,
};

use glium::{glutin::ContextBuilder, Display, Program};

use std::hash::Hash;

use crate::renderer::Renderer;
use crate::key::Key;
use crate::shaders::{FRAGMENT_SHADER, VERTEX_SHADER};

/// Used for debugging
const RENDER_ONCE: bool = false;

pub trait Application {
    fn init(&mut self, _renderer: &mut Renderer) {}
    fn render(&mut self, renderer: &mut Renderer);
    fn window(&mut self, w: WindowBuilder) -> WindowBuilder {
        w
    }
    fn on_event(
        &mut self,
        _event: Event<()>,
        _r: &mut Renderer,
    ) -> Option<ControlFlow> {
        None
    }
    fn on_text_input(
        &mut self,
        _c: char,
        _r: &mut Renderer,
    ) -> Option<ControlFlow> {
        None
    }
    fn on_key_down(
        &mut self,
        _key: Key,
        _r: &mut Renderer,
    ) -> Option<ControlFlow> {
        None
    }
    fn on_key_up(
        &mut self,
        _key: Key,
        _r: &mut Renderer,
    ) -> Option<ControlFlow> {
        None
    }
    fn on_mouse_down(
        &mut self,
        _left: bool,
        _x: f32,
        _y: f32,
        _r: &mut Renderer,
    ) -> Option<ControlFlow> {
        None
    }
}

pub trait ApplicationWrapper<T: Application> {
    fn run(self);
    fn call_render(&mut self, renderer: &mut Renderer);
}

impl<T: 'static> ApplicationWrapper<T> for T
where
    T: Application,
{
    fn call_render(&mut self, renderer: &mut Renderer) {
        renderer.clear();
        renderer.next_frame();
        renderer.hot_id = renderer.get_hit(renderer.mouse.x as f32, renderer.mouse.y as f32);
        renderer.mouse.lmouseclick = false;
        renderer.mouse.rmouseclick = false;
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
                    WindowEvent::CursorMoved { position, .. } => {
                        renderer.mouse.x = position.x;
                        renderer.mouse.y = position.y;
                        ControlFlow::Poll
                    },
                    WindowEvent::MouseInput { state, button, .. } => {
                        match (state, button) {
                            (ElementState::Released, MouseButton::Left) => {
                                renderer.mouse.lmouseclick = true;
                                ControlFlow::Poll
                            },
                            (ElementState::Released, MouseButton::Right) => {
                                renderer.mouse.lmouseclick = true;
                                ControlFlow::Poll
                            },
                            (ElementState::Pressed, mb) => self.on_mouse_down(
                                *mb == MouseButton::Left, 
                                renderer.mouse.x as f32, 
                                renderer.mouse.y as f32, 
                                &mut renderer
                            ).unwrap_or(ControlFlow::Poll),
                            _ => ControlFlow::Poll
                        }
                    },
                    WindowEvent::ModifiersChanged(state) => {
                        renderer.modifiers = *state;
                        ControlFlow::Poll
                    },
                    WindowEvent::ReceivedCharacter(c) => {
                        renderer.input.push(*c);
                        self.on_text_input(*c, &mut renderer).unwrap_or(ControlFlow::Poll)
                    },
                    WindowEvent::KeyboardInput { input, .. } => {
                        input.virtual_keycode.map(Key::from).and_then(|key| {
                            match input.state {
                                ElementState::Pressed => {
                                    renderer.keys.push(key);
                                    self.on_key_down(key, &mut renderer)
                                },
                                ElementState::Released => self.on_key_up(key, &mut renderer),
                            }
                        })
                        .unwrap_or(ControlFlow::Poll)
                    }
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
