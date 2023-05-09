// This is a simple test file that side-steps the ui-test framework
// and external crates, by importing submodule structure based on
// knowledge of how the basictest is structured.
//
// This is probably an anti-pattern, but it was the easiest way for
// pnkfelix to make my Makefile useful again for me.

#[path = "kc/test_dependencies/src/lib.rs"]
mod test_dependencies;

#[path = "kc/tests/basictest.rs"]
mod basictest;
// re-import this here so that the macro inside basictest that assumes
// it is crate root will still work.
use basictest::Data;

pub fn main() {
    basictest::main();
}
