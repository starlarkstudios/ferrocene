//@ run-pass
//@ aux-build:xc_call.rs


extern crate xc_call as aux;

use aux::Foo;

trait Bar : Foo {
    fn g(&self) -> isize;
}

impl Bar for aux::A {
    fn g(&self) -> isize { self.f() }
}

pub fn main() {
    let a = &aux::A { x: 3 };
    assert_eq!(a.g(), 10);
}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
