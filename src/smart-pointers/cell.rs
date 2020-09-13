pub struct Cell<T> {
    value: T,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {value}
    }

    pub fn set(&self, value: T) {
        self.value = value;
    }

    pub fn get(&self) -> T {
        self.value
    }
}