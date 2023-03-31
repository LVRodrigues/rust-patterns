use observer::{Observer, Subject};

pub struct Fibonacci {
    pub current: i32,
    pub previous: i32,
    pub steps: i32,
    observers: Vec<Box<dyn Observer<Fibonacci>>>,
}

impl Subject<Fibonacci> for Fibonacci {
    fn observers(&self) -> &Vec<Box<dyn Observer<Fibonacci>>> {
        &self.observers
    }

    fn value(&self) -> &Fibonacci {
        self
    }

    fn subscribe(&mut self, observer: Box<dyn Observer<Fibonacci>>) {
        self.observers.push(observer);
    }
}

impl Fibonacci {
    pub fn next(&mut self) {
        self.steps += 1;
        let value = self.current + self.previous;
        self.previous = self.current;
        self.current = value;
        self.notify();
    }
}