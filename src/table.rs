use blkid_sys::*;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::str;

pub struct Table<'a> {
    pub(crate) table: blkid_parttable,
    pub(crate) _marker: PhantomData<&'a blkid_parttable>,
}

impl<'a> Table<'a> {
    pub fn get_type(&self) -> &str {
        unsafe {
            let t = blkid_parttable_get_type(self.table);
            assert!(!t.is_null());
            str::from_utf8_unchecked(CStr::from_ptr(t).to_bytes())
        }
    }
}
