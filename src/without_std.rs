#[macro_use]
extern crate alloc;

pub mod without_std {
    pub use alloc::{borrow, collections, string, vec};
    pub use core::{cmp, convert, fmt, iter, mem, result, str};
    pub use no_std_io::{error, io};
    pub use spin::Mutex;
}
