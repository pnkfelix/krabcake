use test_dependencies::{kc_borrow_mut, print_stack_of, print_tag_of};

// This is slightly modified from the presentation in the POPL 2020 paper; the
// code as written in the paper is statically rejected by the compiler, so we
// here replace its use of `&mut`-references with `*mut`-pointers, bypassing the
// static borrow-checking and enabling us to directly test what Krabcake reports
// for this test.

pub fn main() {
    unsafe {
        let mut local = 42;
        let l = std::ptr::addr_of!(local);
        print_stack_of(b"l\0".as_ptr(), l);
        let x = kc_borrow_mut!(local) as *mut _;
        print_tag_of(b"x\0".as_ptr(), x);
        print_stack_of(b"l\0".as_ptr(), l);
        let y = kc_borrow_mut!(*x) as *mut _;
        print_tag_of(b"y\0".as_ptr(), y);
        print_stack_of(b"l\0".as_ptr(), l);
        *x += 1;
        print_stack_of(b"l\0".as_ptr(), l);
        *y = 2;
        print_stack_of(b"l\0".as_ptr(), l);
    }
}
