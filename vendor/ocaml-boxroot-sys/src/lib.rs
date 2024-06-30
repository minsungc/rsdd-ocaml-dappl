/* SPDX-License-Identifier: MIT */

/*! The documentation of the functions can be found inside
boxroot/boxroot.h (including rules for safe usage) */

// Do not link to the std unless if running the tests
#![cfg_attr(not(test), no_std)]

pub type Value = isize;
pub type ValueCell = core::cell::UnsafeCell<Value>;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct BoxRoot {
    contents: core::ptr::NonNull<ValueCell>,
}

#[inline]
pub fn boxroot_get_ref(br: BoxRoot) -> *const ValueCell {
    br.contents.as_ptr()
}

#[inline]
pub unsafe fn boxroot_get(br: BoxRoot) -> Value {
    *(core::cell::UnsafeCell::raw_get(boxroot_get_ref(br)))
}

extern "C" {
    pub fn boxroot_create(v: Value) -> Option<BoxRoot>;
    pub fn boxroot_delete(br: BoxRoot);
    pub fn boxroot_modify(br: *mut BoxRoot, v: Value) -> bool;

    /** With OCaml 4 and when using threads, `boxroot_setup()` must be
    called before creating any thread and before allocating any
    boxroot, but after calling `caml_thread_initialize()`, which must
    be called after OCaml startup (implementing essentially the
    function `boxroot_setup_systhreads` from `boxroot.h`). Obsolete
    under OCaml ≥ 5.0.
     */
    pub fn boxroot_setup() -> bool;
}

#[repr(C)]
#[non_exhaustive]
pub enum Status {
    NotSetup,
    Running,
    ToreDown,
    Invalid,
}

extern "C" {
    pub fn boxroot_teardown();
    pub fn boxroot_status() -> Status;
    pub fn boxroot_print_stats();
}

// Just a test to verify that it compiles and links right
// Run with: cargo test --features "link-ocaml-runtime-and-dummy-program"
#[cfg(test)]
mod tests {
    use crate::{
        boxroot_create, boxroot_delete, boxroot_get, boxroot_get_ref, boxroot_modify,
        boxroot_teardown,
    };

    extern "C" {
        pub fn caml_startup(argv: *const *const i8);
        pub fn caml_shutdown();
    }

    #[test]
    fn it_works() {
        unsafe {
            let arg0 = "ocaml\0".as_ptr() as *const i8;
            let c_args = vec![arg0, core::ptr::null()];

            caml_startup(c_args.as_ptr());

            let mut br = boxroot_create(1).unwrap();
            let v1 = *core::cell::UnsafeCell::raw_get(boxroot_get_ref(br));

            boxroot_modify(&mut br, 2);
            let v2 = boxroot_get(br);

            boxroot_delete(br);

            assert_eq!(v1, 1);
            assert_eq!(v2, 2);

            boxroot_teardown();

            caml_shutdown();
        }
    }
}
