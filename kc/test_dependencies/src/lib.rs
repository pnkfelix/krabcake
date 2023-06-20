// FIXME: it shoudn't be necessary to duplicate the definition of the
// valgrind_do_client_request_expr macro, but I had trouble dealing with tryign
// to make a single definition that could handle the various ways one might need
// to reference the VgKrabcakeClientRequestData struct. Look into the standard
// ways to address this (perhaps Macros 2.0 or even 1.5 is the answer)

pub use self::print_methods::{print_stack_of, print_tag_of};

mod print_methods {
    use super::VgKrabcakeClientRequest;
    use super::VgKrabcakeClientRequestData;

    macro_rules! valgrind_do_client_request_expr {
        ( $zzq_default:expr, $request_code:expr,
          $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr ) => {
            {
                let zzq_args = VgKrabcakeClientRequestData {
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
                        inout("dx") zzq_result,
                        in("ax") &zzq_args,
                    );
                }
                zzq_result
            }
        }
    }

    pub fn print_tag_of<T>(name: *const u8, addr: *const T) -> u64 {
        let stash = &addr;
        valgrind_do_client_request_expr!(
            0x90,
            VgKrabcakeClientRequest::PrintTagOf,
            stash as *const _ as *mut u8,
            name as *const _ as u64,
            0x92,
            0x93,
            0x94
        )
    }

    pub fn print_stack_of<T>(name: *const u8, addr: *const T) -> u64 {
        let stash = &addr;
        valgrind_do_client_request_expr!(
            0x90,
            VgKrabcakeClientRequest::PrintStackOf,
            stash as *const _ as *mut u8,
            name as *const _ as u64,
            0x92,
            0x93,
            0x94
        )
    }
}


// When the necessary rustc machinery is all in place, all instances
// of `kc_borrow_mut!(PLACE)` will be replaced with `&mut PLACE`.
#[macro_export]
macro_rules! kc_borrow_mut {
    ( $data:expr ) => {{
        macro_rules! valgrind_do_client_request_expr {
            ( $zzq_default:expr, $request_code:expr,
              $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr ) => {
                {
                    let zzq_args = ::test_dependencies::VgKrabcakeClientRequestData {
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
                            inout("dx") zzq_result,
                            in("ax") &zzq_args,
                        );
                    }
                    zzq_result
                }
            }
        }

        // let place = ::std::ptr::addr_of_mut!($data);
        let mut place = &mut $data; // do the borrow, but pass along the
                                    // *location* of where we are keeping
                                    // that borrow up to valgrind.
        let _place_ptr = place as *mut _ as *mut u8;
        let stash = &mut place;
        /*
                println!(
                    "pre_ place: {:?} stash: {:?}",
                    _place_ptr, stash as *mut &mut u8
                );
        */
        let _ignored = valgrind_do_client_request_expr!(
            0x90, // we return this if we are not running under valgrind
            ::test_dependencies::VgKrabcakeClientRequest::BorrowMut,
            stash as *mut &mut _ as *mut &mut u8, // we pass this up to valgrind
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


const fn vg_userreq_tool_base(a: u32, b: u32) -> u32 {
    ((a) & 0xff) << 24 | ((b) & 0xff) << 16
}

#[allow(dead_code)]
#[repr(u32)]
pub enum VgKrabcakeClientRequest {
    BorrowMut = vg_userreq_tool_base('K' as u32, 'C' as u32),
    BorrowShr,
    AsRaw,
    AsBorrowMut,
    AsBorrowShr,
    RetagFnPrologue,
    RetagAssign,
    RetagRaw,
    IntrinsicsAssume,
    PrintTagOf,
    PrintStackOf,
    KrabcakeRecordOverlapError = vg_userreq_tool_base('K' as u32, 'C' as u32) + 256,
}

#[repr(C)]
pub struct VgKrabcakeClientRequestData<T> {
    pub request_code: u64,
    pub arg1: *mut T,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
    pub arg5: u64,
}

