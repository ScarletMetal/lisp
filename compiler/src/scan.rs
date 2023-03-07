
pub struct Scanner<'a, T> {
    items: &'a [T],
    current: usize
}

impl<'a, T> Scanner<'a, T> where T: Clone + Copy {
    pub fn new(items: &'a [T]) -> Self {
        Self {
            items,
            current: 0
        }
    }

    pub fn has_next(&self) -> bool {
        self.current < self.items.len()
    }

    pub fn peek(&self) -> Option<T> {
        if self.has_next() {
            return Some(self.items[self.current]);
        }
        return None
    }

    pub fn next(&mut self) {
        if self.has_next() {
            self.current += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let value = self.peek();
        self.next();
        return value;
    }

}
