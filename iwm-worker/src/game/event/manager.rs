use std::pin::Pin;

use super::events::Events;
use crate::game::process::init_process::Process;

pub(crate) type EventHandler =
    for<'a> fn(EventContext<'a>) -> Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>>;

pub(crate) struct EventContext<'a> {
    pub process: &'a mut Process,
    pub event_type: Events,
    pub event_entity_id: i32,
    pub handler_entity_id: i32,
}

pub(crate) struct EventMenager {
    pub events: Vec<(Events, i32)>,
    pub handlers: Vec<(Events, i32, EventHandler)>,
}

impl EventMenager {
    pub fn new() -> Self {
        EventMenager {
            events: Vec::new(),
            handlers: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: Events, event_entity_id: i32) {
        self.events.push((event, event_entity_id));
    }

    pub fn add_handler(&mut self, event: Events, handler_entity_id: i32, handler: EventHandler) {
        self.handlers.push((event, handler_entity_id, handler));
    }

    pub async fn run_all(&mut self, process: &mut Process) {
        while !self.events.is_empty() {
            for (event, handler_entity_id, handler) in &self.handlers {
                if &self.events[0].0 == event {
                    let ctx = EventContext {
                        process,
                        event_type: event.clone(),
                        event_entity_id: self.events[0].1,
                        handler_entity_id: *handler_entity_id,
                    };

                    handler(ctx).await;
                }
            }

            self.events.remove(0);
        }
    }
}
