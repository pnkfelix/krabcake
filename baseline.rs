// This is a simple test file that side-steps the ui-test framework
// and external crates, by importing submodule structure based on
// knowledge of how the basictest is structured.
//
// This is probably an anti-pattern, but it was the easiest way for
// pnkfelix to make my Makefile useful again for me.

mod test_dependencies {
    include!("kc/test_dependencies/src/lib.rs");
}

pub fn main() {
    use test_dependencies::print_tag_of;
    println!("Hello world (from `sb_rs_port/main.rs`)!");
    println!(
        "BorrowMut is {:x}",
        test_dependencies::VgKrabcakeClientRequest::BorrowMut as u32
    );

    let mut val: u8 = 101;
    let x = kc_borrow_mut!(val); // x = &mut val;
    // print_tag_of(b"x\0".as_ptr(), x);
    let x_alias = x as *mut u8;
    // println!("x_alias: {:08x}", x_alias as u64);
    // print_tag_of(b"x_alias\0".as_ptr(), x_alias);
    let y = kc_borrow_mut!(*x);
    let y_addr = y as *const u8 as u64;
    let retval = print_tag_of(b"y\0".as_ptr(), y);
    // println!("print_tag_of returned: {:08x}", retval);

    *y = 105;

    unsafe {
        *x_alias = 103;
    }

    let end = *y;
    // println!("y_addr: {:08x}", y_addr);

    // Note: I didn't see a load against `y` above without a use
    // of `end` here. It would be nice to avoid requiring that,
    // but maybe such is life in release mode. Look into it.
    println!("Goodbye world, end: {}!", end);
}
