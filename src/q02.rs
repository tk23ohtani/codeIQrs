const MAX: usize = 10;

struct eBuf {
    data: [u32; MAX],
    ptr: my_rslib::RingPtr,
}
impl eBuf {
    fn new() -> eBuf {
        eBuf {
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
    let x = eBuf::new();
    0
}
