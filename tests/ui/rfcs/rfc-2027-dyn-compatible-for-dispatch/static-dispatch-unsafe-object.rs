// Check that we can statically dispatch methods for object
// unsafe trait objects, directly and indirectly
//
//@ check-pass

#![feature(dyn_compatible_for_dispatch)]

trait Statics {
    fn plain() {}
    fn generic<T>() {}
}

trait Trait: Sized {}

impl<'a> Statics for dyn Trait + 'a {}

fn static_poly<T: Statics + ?Sized>() {
    T::plain();
    T::generic::<usize>();
}

fn inferred_poly<T: Statics + ?Sized>(t: &T) {
    static_poly::<T>();
    T::plain();
    T::generic::<usize>();
}

fn call(t: &dyn Trait) {
    static_poly::<dyn Trait>();
    inferred_poly(t);
}

fn main() {
    static_poly::<dyn Trait>();
    <dyn Trait>::plain();
    <dyn Trait>::generic::<usize>()
}

// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
// ferrocene-annotations: fls_46ork6fz5o2e
// Implementation Coherence
