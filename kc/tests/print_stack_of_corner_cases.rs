use test_dependencies::{kc_borrow_mut, print_stack_of};

fn main() {
    let mut percent_s = 5;
    let p = kc_borrow_mut!(percent_s);
    let msg = b"C format sequences are %% %b %q %d %i %o %u %x %X %f %e %E %g %G %c %s\0";
    print_stack_of(msg.as_ptr(), p);
}
