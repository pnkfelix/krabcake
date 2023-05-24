use test_dependencies::VgKrabcakeClientRequest;
use test_dependencies::kc_borrow_mut;

pub fn main() {
    println!("Hello world (from `sb_rs_port/main.rs`)!");
    println!(
        "BorrowMut is {:x}",
        VgKrabcakeClientRequest::BorrowMut as u32
    );

    let mut val: u8 = 101;
    println!("about to &mut into x");
    let x = kc_borrow_mut!(val); // x = &mut val;
    println!("borrowed &mut val into x");
    let x_alias = x as *mut u8;
    println!("made alias of x in x_alias");
    println!("about to &mut into y");
    let y = kc_borrow_mut!(*x);
    println!("borrowed &mut val into y");

    println!("about to mutate val via y");
    *y = 105;
    println!("finished mutate val via y");

    println!("about to mutate val via x_alias");
    unsafe {
        *x_alias = 103;
    }
    println!("finished mutate val via x_alias");

    println!("about to read val via y");
    let end = *y;

    // Note: I didn't see a load against `y` above without a use
    // of `end` here. It would be nice to avoid requiring that,
    // but maybe such is life in release mode. Look into it.
    println!("Goodbye world, end: {}!", end);
}
