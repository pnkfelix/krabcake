mod krabcake;

#[repr(C)]
struct Data<T> {
    request_code: u64,
    arg1: *mut T,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
}

#[macro_export]
macro_rules! valgrind_do_client_request_expr {
    ( $zzq_default:expr, $request_code:expr,
      $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr ) => {
        {
            let zzq_args = crate::Data {
                request_code: $request_code as u64,
                arg1: $arg1,
                arg2: $arg2,
                arg3: $arg3,
                arg4: $arg4,
                arg5: $arg5,
            };
            let mut zzq_result = $zzq_default;
            #[allow(unused_unsafe)]
            unsafe {
                ::std::arch::asm!(
                    "rol rdi, 3",
                    "rol rdi, 13",
                    "rol rdi, 61",
                    "rol rdi, 51",
                    "xchg rbx, rbx",
                    inout("di") zzq_result,
                    in("ax") &zzq_args,
                );
            }
            zzq_result
        }
    }
}

macro_rules! kc_retag { ( $var:ident ) => { $var = kc_borrow_mut!(*$var) } }

#[cfg(not_now)]
#[macro_export]
macro_rules! kc_borrow_mut { ( $data:expr ) => { &mut $data } }

#[macro_export]
macro_rules! kc_borrow_mut {
    ( $data:expr ) => {
        {
            let place = ::std::ptr::addr_of_mut!($data);
            let raw_ptr = valgrind_do_client_request_expr!(place, crate::krabcake::VgKrabcakeClientRequest::BorrowMut, place, 0x91, 0x92, 0x93, 0x94);
            // When the necessary rustc machinery is all in place, all instances
            // of `kc_borrow_mut!(PLACE)` will be replaced with `&mut PLACE`.
            // Therefore, we go ahead and convert the `&raw` place above into an
            // `&mut`, so that the appropriate type is inferred for the
            // expression.
            unsafe { &mut *raw_ptr }
        }
    }
}

pub fn sb_in_c() {
    println!("Hello world!");

    let mut val: u8 = 1;
    let x = kc_borrow_mut!(val); // x = &mut val;
    let y = kc_borrow_mut!(*x);

    println!("before *y = 5, val: {}", val);
    *y = 5;
    println!("after *y = 5, val: {}", val);

    println!("before *x = 3, val: {}", val);
    // Write through a pointer aliasing `y`
    *x = 3;
    println!("after *x = 3, val: {}", val);

    let end = *y;

    println!("Goodbye world, end: {}!", end);
}

pub fn main() {
    println!("PAGE 1");
    page_01::main();

    println!("PAGE 7");
    page_07::main();

    println!("PAGE 9");
    page_09::main();

    println!("PAGE 10");
    page_10::main();

    sb_in_c();
}


mod page_01 {
    use super::kc_borrow_mut;
    pub fn example1(x: &mut i32, y: &mut i32) -> i32 {
        *x = 42;
        println!("expect KC error from above operation (see page 10)");
        *y = 13;
        return *x;
    }

    pub fn main() {
        let mut local = 5;
        let raw_pointer = kc_borrow_mut!(local) as *mut i32;
        let result = unsafe { example1(&mut *raw_pointer, &mut *raw_pointer) };
        println!("result: {}, may be 13 or 42 due to UB", result);
    }
}

mod page_07 {
    use super::kc_borrow_mut;
    pub fn main() {
        let mut local = 0;
        let x = kc_borrow_mut!(local);
        let y = kc_borrow_mut!(*x); // reborrow x to y
        *x = 1; // use x again
        *y = 2; // KC error! y used after x got used
        println!("expect KC error from above operation");
    }
}

mod page_09 {
    use super::kc_borrow_mut;
    pub fn main() {
        let mut local = 42; // stored at location:L with tag:0
        // H(L) = (42, [Unique(0)])
        let x = kc_borrow_mut!(local); // x = Pointer(L, tag:1)
        // H(L) = (42, [Unique(0), Unique(1)])
        let y = kc_borrow_mut!(*x);    // y = Pointer(L, tag:2)
        // H(L) = (42, [Unique(0), Unique(1), Unique(2)])
        *x += 1; // pops until we find x's tag:1.
        // H(L) = (42, [Unique(0), Unique(1)]
        *y = 2;
        // UB: y's tag:2 not in the stack.
        println!("expect KC error from above operation");
    }
}

mod page_10 {
    pub fn main() {
        let mut local = 5; // stored t location:L with tag:0
        // H(L) = (5, [Unique(0)])
        let raw_pointer = kc_borrow_mut!(local) as *mut i32; // = Pointer(L, \bot)
        // H(L) = (5, [Unique(0), Unique(1), SharedRW])
        let result = unsafe {
            example1(
                kc_borrow_mut!(*raw_pointer), // = Pointer(L, 2)
                kc_borrow_mut!(*raw_pointer), // = Pointer(L, 3)
            )
        };
        println!("{}", result);
    }

    fn example1(x: &mut i32, y: &mut i32) -> i32 {
        // x = Pointer(L,2), y = Pointer(L,3), H(L) = (5, [Unique(0), Unique(1), SharedRW, Unique(3)])
        *x = 42;
        println!("expect KC error from above operation");
        *y = 13;
        return *x;
    }
}

mod page_11 {

    // FIXME: once retag is supported natively, get rid of these `mut` on `x` and `y`.
    fn example1(mut x: &mut i32, mut y: &mut i32) -> i32 {
        kc_retag!(x);
        kc_retag!(y);
        *x = 42;
        *y = 13;
        return *x;
    }

    // (below is cut-and-pasted from page_10::main above.)
    pub fn main() {
        let mut local = 5; // stored t location:L with tag:0
        // H(L) = (5, [Unique(0)])
        let raw_pointer = kc_borrow_mut!(local) as *mut i32; // = Pointer(L, \bot)
        // H(L) = (5, [Unique(0), Unique(1), SharedRW])
        let result = unsafe {
            example1(
                kc_borrow_mut!(*raw_pointer), // = Pointer(L, 2)
                kc_borrow_mut!(*raw_pointer), // = Pointer(L, 3)
            )
        };
        println!("{}", result);
    }
}
