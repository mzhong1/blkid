// Copyright (c) 2017 Chris Holcombe

// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT> This file may not be copied, modified,
// or distributed except according to those terms.

//! See https://www.kernel.org/pub/linux/utils/util-linux/v2.21/libblkid-docs/index.html
//! for the reference manual to the FFI bindings
use blkid_sys::*;

use std::ffi::{CStr, CString};

pub fn known_fstype(fstype: &str) -> Result<bool, BlkIdError> {
    let fstype = CString::new(fstype).expect("interior null byte in UTF-8 string");

    unsafe { cvt(blkid_known_fstype(fstype.as_ptr())).map(|v| v == 1) }
}

mod cache;
mod dev;
mod errors;
mod part_list;
mod partition;
mod probe;
mod table;
mod tag;
mod topology;

pub use crate::cache::*;
pub use crate::dev::*;
pub use crate::errors::*;
pub use crate::part_list::*;
pub use crate::probe::*;
pub use crate::table::*;
pub use crate::tag::*;
pub use crate::topology::*;

pub(crate) fn cstr_to_str<'a>(value: *const libc::c_char) -> Option<&'a str> {
    if value.is_null() {
        return None;
    }

    unsafe { CStr::from_ptr(value).to_str().ok() }
}
