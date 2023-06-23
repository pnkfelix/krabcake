// FIXME we haven't implemented raw pointers (i.e. \bot Tag and SharedRW Item)
// so the test output we currently have for this is bogus.

use test_dependencies::{kc_as_raw, kc_borrow_mut, print_stack_of, print_tag_of};

// This is slightly modified from the presentation in the POPL 2020 paper; we
// want to observe the stack state as we run.

#[allow(non_upper_case_globals)]
static mut l: *const i32 = std::ptr::null();

fn main() {
    unsafe {
        let mut local = 5;
        l = std::ptr::addr_of!(local);
        // Currently the stack is emtpy, but there should be a tag Unique(0) for destination
        // Once that behavior is added to Krabcake, update the comments below
        // to include Unique(0).
        print_stack_of(b"l\0".as_ptr(), l);

        let raw_pointer = kc_as_raw!(kc_borrow_mut!(local), i32); // kc_as_raw == as *mut i32
        // Now we have a Unique(1) tag for borrow_mut, and SharedRW for as_raw
        // stack = [Unique(1), SharedRW]
        print_stack_of(b"l\0".as_ptr(), l);
    
        let result = {
            let arg1 = kc_borrow_mut!(*raw_pointer);
            print_tag_of(b"arg1\0".as_ptr(), arg1);
            // Now we have a Unique(2) added
            // stack = [Unique(1), SharedRW, Unique(2)]
            print_stack_of(b"l\0".as_ptr(), l);
            let arg2 = kc_borrow_mut!(*raw_pointer);
            print_tag_of(b"arg2\0".as_ptr(), arg2);
            // All tags before SharedRW should be popped (in this case, Unique(2))
            // stack = [Unique(1), SharedRW, Unique(3)]
            // At the time of this writing, this is not happening in Krabake!! Needs to be fixed.
            print_stack_of(b"l\0".as_ptr(), l);
            example1(arg1, arg2)
        };
        println!("{}", result);
    }
}

fn example1(x: &mut i32, y: &mut i32) -> i32 {
    unsafe {
        print_stack_of(b"l\0".as_ptr(), l);
    }
    *x = 42;
    *y = 13;
    return *x;
}
