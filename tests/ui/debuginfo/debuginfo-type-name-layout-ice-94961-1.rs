// Make sure the compiler does not ICE when trying to generate the debuginfo name of a type that
// causes a layout error. See https://github.com/rust-lang/rust/issues/94961.

//@ compile-flags:-C debuginfo=2
//@ build-fail
//@ error-pattern: too big for the target architecture
//@ normalize-stderr-64bit: "18446744073709551615" -> "SIZE"
//@ normalize-stderr-32bit: "4294967295" -> "SIZE"

#![crate_type = "rlib"]

pub struct Foo<T>([T; usize::MAX]);

pub fn foo() -> usize {
    std::mem::size_of::<Foo<u8>>()
}

// ferrocene-annotations: um_rustc_C_debuginfo
