pub mod window;

pub trait Observer<T> {
    fn receive(&self, value: T);
}

pub trait Subject<T> {
    fn observers(&self) -> Vec<Box<dyn Observer<T>>>;

    fn value(&self) -> T;

    fn subscribe(&self, observer: Box<dyn Observer<T>>);

    fn notify(&self) {
        self.observers()
            .iter()
            .for_each(|o| o.receive(self.value()));
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
