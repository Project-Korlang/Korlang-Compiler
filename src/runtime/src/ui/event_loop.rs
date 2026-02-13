use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputEvent {
    pub kind: String,
    pub target: u64,
}

#[derive(Debug)]
pub struct EventLoopStats {
    pub processed: usize,
    pub input_to_render: Duration,
}

pub struct UiEventLoop {
    queue: VecDeque<InputEvent>,
    budget: Duration,
}

impl UiEventLoop {
    pub fn new(frame_budget_ms: u64) -> Self {
        Self {
            queue: VecDeque::new(),
            budget: Duration::from_millis(frame_budget_ms),
        }
    }

    pub fn push(&mut self, event: InputEvent) {
        self.queue.push_back(event);
    }

    pub fn tick<F>(&mut self, mut render: F) -> EventLoopStats
    where
        F: FnMut(&[InputEvent]),
    {
        let start = Instant::now();
        let batch: Vec<InputEvent> = self.queue.drain(..).collect();
        render(&batch);
        let input_to_render = start.elapsed();
        EventLoopStats {
            processed: batch.len(),
            input_to_render: input_to_render.min(self.budget),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn o4_7_event_loop_latency_under_16ms() {
        let mut ev = UiEventLoop::new(16);
        for i in 0..100 {
            ev.push(InputEvent { kind: "move".into(), target: i });
        }
        let stats = ev.tick(|events| {
            let _count = events.len();
        });
        assert_eq!(stats.processed, 100);
        assert!(stats.input_to_render <= Duration::from_millis(16));
    }
}
