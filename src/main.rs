
const DEPTH: usize = 32;

/// # ｎ進数を配列型に展開する
fn exlog(_dat: &mut [u32; DEPTH], _val: u32, _log: u32) -> usize {
    let mut val = _val; // 引数はミュータブルじゃないのかぁ～
    let n: usize = 0;
    for n in 0..DEPTH {
        _dat[n] = val % _log;
        val = val / _log;
        if val == 0 { return 1 + n; }
    }
    n
}

/// # 回文かどうかを確認する
fn check(_dat: &mut [u32; DEPTH], _n: usize) -> bool {
    for n in 0..(_n / 2) {
        if _dat[n] != _dat[_n - 1 - n] { return false; }
    }
    true
}

//
fn q01() -> u32 {
    let mut array: [u32; DEPTH] = Default::default();
    for n in 10..1000 {
        let len = exlog(&mut array, n, 2);
        if !check(&mut array, len) { continue; }
        let len = exlog(&mut array, n, 8);
        if !check(&mut array, len) { continue; }
        let len = exlog(&mut array, n, 10);
        if !check(&mut array, len) { continue; }
        return n;
    }
    0
}

#[cfg(test)]
mod q01_tests {
    #[test]
    fn exlog_test() {
        use crate::*;
        let mut array: [u32; DEPTH] = Default::default();
        let mut rs: usize;
        rs = exlog(&mut array, 0b101, 2);
        assert_eq!(3, rs);
        assert_eq!(1, array[0]);
        assert_eq!(0, array[1]);
        assert_eq!(1, array[2]);
        rs = exlog(&mut array, 0o321, 8);
        assert_eq!(3, rs);
        assert_eq!(1, array[0]);
        assert_eq!(2, array[1]);
        assert_eq!(3, array[2]);
        rs = exlog(&mut array, 123, 10);
        assert_eq!(3, rs);
        assert_eq!(3, array[0]);
        assert_eq!(2, array[1]);
        assert_eq!(1, array[2]);
    }
    #[test]
    fn check_test() {
        use crate::*;
        let mut array: [u32; DEPTH] = Default::default();
        let rs: bool;
        let n = exlog(&mut array, 0b101, 2);
        rs = check(&mut array, n);
        assert_eq!(true, rs);
    }
    #[test]
    fn q01_test() {
        use crate::*;
        assert_eq!(585, q01());
    }
}

fn main() {
    println!("Hello, world!");
    println!("Q01 Result = {}", q01());
}
