pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) -> () {
        self.elements.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
}
