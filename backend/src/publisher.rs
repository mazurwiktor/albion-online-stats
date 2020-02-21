use crate::game::Event;

type Callback = dyn FnMut(Event);

#[derive(Default)]
pub struct Publisher {
    subscribers: Vec<Box<Callback>>
}

impl Publisher {
    pub fn new(subscribers: Vec<Box<Callback>>) -> Self {
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