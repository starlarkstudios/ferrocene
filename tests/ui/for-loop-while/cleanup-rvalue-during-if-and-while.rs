//@ run-pass
// This test verifies that temporaries created for `while`'s and `if`
// conditions are dropped after the condition is evaluated.

// FIXME(static_mut_refs): this could use an atomic
#![allow(static_mut_refs)]

struct Temporary;

static mut DROPPED: isize = 0;

impl Drop for Temporary {
    fn drop(&mut self) {
        unsafe { DROPPED += 1; }
    }
}

impl Temporary {
    fn do_stuff(&self) -> bool {true}
}

fn borrow() -> Box<Temporary> { Box::new(Temporary) }


pub fn main() {
    let mut i = 0;

    // This loop's condition
    // should call `Temporary`'s
    // `drop` 6 times.
    while borrow().do_stuff() {
        i += 1;
        unsafe { assert_eq!(DROPPED, i) }
        if i > 5 {
            break;
        }
    }

    // This if condition should
    // call it 1 time
    if borrow().do_stuff() {
        unsafe { assert_eq!(DROPPED, i + 1) }
    }
}

// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_afafmafz4hf2
// Drop Order
//
// ferrocene-annotations: fls_5eima0pd31c0
// Drop Scope Extension
//
// ferrocene-annotations: fls_rm4ncoopcdvj
// Drop Scopes
//
// ferrocene-annotations: fls_cleoffpn5ew6
// Temporaries
//
// ferrocene-annotations: fls_5jjm1kt43axd
// While Loops
//
// ferrocene-annotations: fls_gho955gmob73
// Variables
