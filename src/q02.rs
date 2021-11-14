const MAX: usize = 10;

struct EBuf {
    data: [u32; MAX],
    ptr: my_rslib::RingPtr,
}
impl EBuf {
    fn new() -> EBuf {
        EBuf {
            data: [0; MAX],
            ptr: my_rslib::RingPtr::new(MAX),
        }
    }
    fn push(&mut self, v: u32) -> Option<u32> {
        if let Some(n) = self.ptr.push() {
            self.data[n] = v;
            return Some(v);
        }
        None
    }
    fn pop(&mut self) -> Option<u32> {
        if let Some(n) = self.ptr.pop() {
            return Some(self.data[n]);
        }
        None
    }
    fn get_left(&self) -> usize {
        self.ptr.get_left()
    }
}


pub fn q02() -> u32 {
    let x = EBuf::new();
    0
}

#[cfg(test)]
mod q02_tests {
    use crate::q02::*;
    #[test]
    fn q02_test() {
        assert_eq!(0, q02());
    }
}
