// TODO Copyright Header

//! A very basic libstd front
//!
//! This is almost totally independent of the runtime of rust, relying only
//! on the presence of allocation defined within liballoc.

#![crate_name="basicstd"]
#![crate_type="rlib"]
#![doc(html_logo_url = "https://avatars.io/gravatar/d0ad9c6f37bb5aceac2d7ac95ba82607?size=large",
       html_favicon_url="https://avatars.io/gravatar/d0ad9c6f37bb5aceac2d7ac95ba82607?size=small")]
#![feature(unsafe_destructor, int_uint, box_syntax, macro_reexport)]
#![feature(optin_builtin_traits)]
#![feature(core)]
#![feature(alloc)]
#![feature(unicode)]
#![feature(collections)]
#![feature(rand)]
#![feature(hash)]
#![allow(deprecated)]
#![no_std]

#[macro_reexport(assert, assert_eq, debug_assert, write, writeln)]
#[macro_use] extern crate core;
#[macro_reexport(vec)]
#[macro_use] extern crate "collections" as core_collections;
extern crate "rand" as rrand;
extern crate alloc;
extern crate unicode;

pub use alloc::{boxed, rc};
pub use core::{any, borrow, cell, clone, cmp, default, error};
pub use core::{f32, f64, finally, hash, i16, i32, i64, i8, int, intrinsics};
pub use core::{isize, iter, marker, mem, num, ops, option, ptr, raw};
pub use core::{result, simd, u16, u32, u64, u8, uint, usize};
pub use core_collections::{str, string, slice, vec};
pub use unicode::char;

#[path = "../rust/src/libstd/macros.rs"]
#[macro_use] mod macros;

#[path = "../rust/src/libstd/ascii.rs"]
pub mod ascii;
#[path = "../rust/src/libstd/fmt.rs"]
pub mod fmt;
#[path = "../rust/src/libstd/collections/mod.rs"]
pub mod collections;

pub mod rand {
    pub use rrand::*;
    /// This is just a front. We will use what we have, which is a IsaacRng.
    #[derive(Clone)]
    #[allow(missing_copy_implementations)]
    pub struct ThreadRng(IsaacRng);
    impl Rng for ThreadRng {
        fn next_u32(&mut self) -> u32 { self.0.next_u32() }
    }
    static mut base_seed : [u32; 256] = [0; 256];
    /// Get an rng
    pub fn thread_rng() -> ThreadRng {
        use ::slice::*;
        let mut rng = IsaacRng::new_unseeded();
        rng.reseed(unsafe { &base_seed });
        let bs : &'static mut [u32] = unsafe { &mut base_seed };
        let len = bs.len();
        bs[rng.gen_range(0, len)] = rng.gen();
        ThreadRng(rng)
    }
}

pub mod thread {
    #[allow(missing_copy_implementations)]
    pub struct Thread;
    impl Thread {
        pub fn panicking() -> bool {
            false
        }
    }
}
pub mod sync;

pub mod rt {
    pub use alloc::heap;

    pub fn begin_unwind(msg: &str, fl: &(&'static str, usize)) -> ! {
        ::core::panicking::panic_fmt(format_args!("{}", msg), fl)
    }
    pub fn begin_unwind_fmt(msg: ::fmt::Arguments, file_line: &(&'static str, usize)) -> ! {
        ::core::panicking::panic_fmt(msg, file_line)
    }
}

pub mod prelude {
    pub mod v1 {
        pub use marker::{Copy, Send, Sized, Sync};
        pub use ops::{Drop, Fn, FnMut, FnOnce};
        pub use mem::drop;
        pub use boxed::Box;
        pub use char::CharExt;
        pub use clone::Clone;
        pub use cmp::{PartialEq, PartialOrd, Eq, Ord};
        pub use iter::DoubleEndedIterator;
        pub use iter::ExactSizeIterator;
        pub use iter::{Iterator, IteratorExt, Extend};
        pub use option::Option::{self, Some, None};
        pub use ptr::{PtrExt, MutPtrExt};
        pub use result::Result::{self, Ok, Err};
        pub use slice::AsSlice;
        pub use slice::{SliceExt, SliceConcatExt};
        pub use str::{Str, StrExt};
        pub use string::{String, ToString};
        pub use vec::Vec;
        pub use iter::range;
    }
}

#[doc(hidden)]
mod std {
    pub use super::*;
}
