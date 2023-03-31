pub trait Observer<T> {
    fn receive(&self, value: &T);
}

pub trait Subject<T> {
    fn observers(&self) -> &Vec<Box<dyn Observer<T>>>;

    fn value(&self) -> &T;

    fn subscribe(&mut self, observer: Box<dyn Observer<T>>);

    fn notify(&self) {
        self.observers()
            .iter()
            .for_each(|o| o.receive(self.value()));
    }
}