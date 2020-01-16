pub struct Bytes<S>(Option<S>);

impl<S> Bytes<S> {
    fn read(&self) -> &Option<S> { &self.0 }
    fn alloc(&mut self, size: S) { self.0 = Some(size); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let byte = Some([0]);
        let b = Bytes(byte);

        assert_eq!(b.read().unwrap(), [0]);
    }

    #[test]
    fn alloc() {
        let mut b = Bytes(Some([0]));
        
        b.alloc([9]);

        assert_eq!(b.read().unwrap(), [9]);
    }
}
