use test_dependencies::{kc_borrow_mut, kc_as_raw, print_stack_of, print_tag_of};

pub fn main() {
    let mut val: u8 = 101;
    let l = std::ptr::addr_of!(val);
    print_stack_of(b"l_0\0".as_ptr(), l);
    let x = kc_borrow_mut!(val); // x = &mut val;
    print_stack_of(b"l_1\0".as_ptr(), l);
    let x_alias = kc_as_raw!(x, u8);
    print_stack_of(b"l_2\0".as_ptr(), l);
    let y = unsafe { kc_borrow_mut!(*x_alias) };
    print_tag_of(b"y\0".as_ptr(), y);
    print_stack_of(b"l_3\0".as_ptr(), l);
    let z = unsafe { kc_borrow_mut!(*x_alias) };
    print_tag_of(b"z\0".as_ptr(), z);
    print_stack_of(b"l_4\0".as_ptr(), l);
    println!("about to write via y");

    *y = 105;

    print_stack_of(b"l_5\0".as_ptr(), l);
    let end = *y;
    print_stack_of(b"l_6\0".as_ptr(), l);
    println!("Goodbye world, end: {}!", end);
    print_stack_of(b"l_7\0".as_ptr(), l);
}
