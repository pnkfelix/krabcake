use test_dependencies::VgKrabcakeClientRequest;
use test_dependencies::kc_borrow_mut;

pub fn main() {
    println!("Hello world (from `sb_rs_port/main.rs`)!");
    println!(
        "BorrowMut is {:x}",
        VgKrabcakeClientRequest::BorrowMut as u32
    );

    let mut val: u8 = 101;
    let x = kc_borrow_mut!(val); // x = &mut val;
    let x_alias = x as *mut u8;
    let y = kc_borrow_mut!(*x);

    *y = 105;

    unsafe {
        *x_alias = 103;
    }

    let end = *y;

    // Note: I didn't see a load against `y` above without a use
    // of `end` here. It would be nice to avoid requiring that,
    // but maybe such is life in release mode. Look into it.
    println!("Goodbye world, end: {}!", end);
}
