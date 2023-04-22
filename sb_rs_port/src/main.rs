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

#[macro_export]
macro_rules! kc_borrow_mut {
    ( $data:expr ) => {{
        // let place = ::std::ptr::addr_of_mut!($data);
        let place = &mut $data;
        let raw_ptr = valgrind_do_client_request_expr!(
            place as *mut u8,
            crate::krabcake::VgKrabcakeClientRequest::BorrowMut,
            place as *mut u8,
            0x91,
            0x92,
            0x93,
            0x94
        );
        // When the necessary rustc machinery is all in place, all instances
        // of `kc_borrow_mut!(PLACE)` will be replaced with `&mut PLACE`.
        // Therefore, we go ahead and convert the `&raw` place above into an
        // `&mut`, so that the appropriate type is inferred for the
        // expression.
        if true {
            unsafe { &mut *raw_ptr }
        } else {
            // However, we return the original `&mut` on a false branch,
            // to force the lifetimes on the `&mut` above to match what
            // lifetimes were assigned to the original place.
            place
        }
    }};
}

pub fn main() {
    use krabcake::VgKrabcakeClientRequest as ClientRequest;
    println!("Hello world (from `sb_rs_port/main.rs`)!");
    println!("BorrowMut is {:x}", ClientRequest::BorrowMut as u32);

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
