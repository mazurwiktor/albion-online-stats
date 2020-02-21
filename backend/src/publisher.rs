use crate::game::Event;

pub type Subscriber = dyn FnMut(Event);
pub type Subscribers = Vec<Box<Subscriber>>;

#[derive(Default)]
pub struct Publisher {
    subscribers: Subscribers
}

impl Publisher {
    pub fn new(subscribers: Subscribers) -> Self {
        Self {subscribers, ..Default::default()}
    }

    pub fn publish(&mut self, event: &Event) {
        for subscriber in &mut self.subscribers {
            subscriber(event.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_subscription() {
        let subscriber = |_| {};

        let mut publisher = Publisher::new(vec![Box::new(subscriber)]);
        publisher.publish(&Event::ZoneChange);
    }
}