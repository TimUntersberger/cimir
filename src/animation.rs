pub use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Animation {
    start_time: Instant,
    duration: Duration,
    transitions: Vec<Transition>,
    pub done: bool,
}

impl Animation {
    pub fn new(duration: Duration, transitions: Vec<Transition>) -> Self {
        Animation {
            start_time: Instant::now(),
            duration,
            transitions,
            done: false,
        }
    }

    pub fn reset(&mut self) {
        self.done = false;
        self.start_time = Instant::now()
    }

    pub fn animate(&mut self) -> Vec<f32> {
        if self.done {
            return self.transitions.iter().map(|t| t.get_done()).collect();
        }
        let progress = (self.start_time.elapsed().as_millis() as f32
            / self.duration.as_millis() as f32)
            .min(1.0);
        if progress == 1.0 {
            self.done = true;
        }
        self.transitions
            .iter()
            .map(|t| t.calculate(progress))
            .collect()
    }
}

#[derive(Clone, Debug)]
pub enum Transition {
    Linear(f32, f32),
}

impl Transition {
    pub fn calculate(&self, progress: f32) -> f32 {
        match self {
            Self::Linear(from, to) => {
                let d = to - from;
                from + d * progress
            }
        }
    }
    pub fn get_done(&self) -> f32 {
        match self {
            Self::Linear(_, end) => *end,
        }
    }
}
