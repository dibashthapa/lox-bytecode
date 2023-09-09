pub type Value = f64;

#[derive(Clone, Debug)]
pub struct ValueArray {
    pub values: Vec<Value>,
}
pub fn print_value(value: Value) {
    print!("'{}'", value);
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) -> usize {
        let count = self.values.len();
        self.values.push(value);
        count
    }

    pub fn free(&mut self) {
        self.values = vec![];
    }
}
