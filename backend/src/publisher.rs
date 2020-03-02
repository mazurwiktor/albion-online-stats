use crate::game::Event;

pub type Subscriber = dyn FnMut(Event) + Send;
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

    use std::thread;

    #[test]
    fn test_simple_subscription() {
        let subscriber = |_| {};

        let mut publisher = Publisher::new(vec![Box::new(subscriber)]);
        publisher.publish(&Event::ZoneChange);
    }

    pub fn test_call_in_the_thread(subscribers: Subscribers)
    {
        thread::spawn(move || {
            let _ = Publisher::new(subscribers);
        });
    }

    #[test]
    fn test_threads() {
        test_call_in_the_thread(vec![]);
    }

}