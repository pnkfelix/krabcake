use test_dependencies::kc_borrow_mut;

// This is slightly modified from the presentation in the POPL 2020 paper; the
// code as written in the paper is statically rejected by the compiler, so we
// here replace its use of `&mut`-references with `*mut`-pointers, bypassing the
// static borrow-checking and enabling us to directly test what Krabcake reports
// for this test.

pub fn main() {
    unsafe {
        let mut local = 0;
        let x = kc_borrow_mut!(local) as *mut _;
        let y = kc_borrow_mut!(*x) as *mut _;
        *x = 1;
        *y = 2;
    }
}
