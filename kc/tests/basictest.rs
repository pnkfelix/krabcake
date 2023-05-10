use test_dependencies::VgKrabcakeClientRequest;

#[repr(C)]
pub(crate) struct Data<T> {
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

// When the necessary rustc machinery is all in place, all instances
// of `kc_borrow_mut!(PLACE)` will be replaced with `&mut PLACE`.
#[macro_export]
macro_rules! kc_borrow_mut {
    ( $data:expr ) => {{
        // let place = ::std::ptr::addr_of_mut!($data);
        let mut place = &mut $data; // do the borrow, but pass along the
                                    // *location* of where we are keeping
                                    // that borrow up to valgrind.
        let _place_ptr = place as *mut u8;
        let stash = &mut place;
        /*
                println!(
                    "pre_ place: {:?} stash: {:?}",
                    _place_ptr, stash as *mut &mut u8
                );
        */
        let _ignored = valgrind_do_client_request_expr!(
            0x90, // we return this if we are not running under valgrind
            VgKrabcakeClientRequest::BorrowMut,
            stash as *mut &mut u8, // we pass this up to valgrind
            0x91,
            0x92,
            0x93,
            0x94
        );
        /*
                println!(
                    "post place: {:?} stash: {:?}",
                    _place_ptr, stash as *mut &mut u8
                );
        */
        if true {
            // We load the borrow out from the memory location in `stash`, so that any tags
            // that the tool now associated with that memory location via `stash` will be
            // propagated along.
            unsafe { ::std::ptr::read(stash) }
        } else {
            // However, we return the original `&mut` on a false branch,
            // to force the lifetimes on the `&mut` above to match what
            // lifetimes were assigned to the original place.
            place
        }
    }};
}

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
