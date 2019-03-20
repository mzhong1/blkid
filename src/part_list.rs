use crate::partition::Partition;
use blkid_sys::*;
use errors::*;
use std::marker::PhantomData;
use table::Table;

pub struct PartList<'a> {
    pub(crate) list: blkid_partlist,
    pub(crate) _marker: PhantomData<&'a blkid_partlist>,
}

impl<'a> PartList<'a> {
    pub fn get_partition(&self, partition: i32) -> Option<Partition> {
        unsafe {
            let res = blkid_partlist_get_partition(self.list, partition as libc::c_int);
            if res.is_null() {
                None
            } else {
                Some(Partition {
                    partition: res,
                    _marker: PhantomData,
                })
            }
        }
    }

    pub fn get_partition_by_partno(&self, partition: i32) -> Option<Partition> {
        unsafe {
            let res = blkid_partlist_get_partition_by_partno(self.list, partition as libc::c_int);
            if res.is_null() {
                None
            } else {
                Some(Partition {
                    partition: res,
                    _marker: PhantomData,
                })
            }
        }
    }

    pub fn get_table(&self) -> Option<Table> {
        unsafe {
            let table = blkid_partlist_get_table(self.list);
            if table.is_null() {
                None
            } else {
                Some(Table {
                    table,
                    _marker: PhantomData,
                })
            }
        }
    }

    pub fn numof_partitions(&self) -> Result<u32, BlkIdError> {
        unsafe { cvt(blkid_partlist_numof_partitions(self.list)).map(|v| v as u32) }
    }
}
