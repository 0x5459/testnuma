use core::ffi::c_void;
use std::{marker::PhantomData, os::raw::c_int, ptr::NonNull};

use anyhow::Result;

type c_size_t = usize;

#[link(name = "numa", kind = "dylib")]
extern "C" {
    fn numa_alloc_local(size: c_size_t) -> *mut c_void;
    fn numa_free(start: *mut c_void, size: c_size_t);
    fn numa_getpagesize() -> c_int;
}

pub enum Memory {
    Raw(Vec<u8>),
    Numa(NumaMemory),
}

impl AsMut<[u8]> for Memory {
    fn as_mut(&mut self) -> &mut [u8] {
        match self {
            Memory::Raw(v) => v.as_mut_slice(),
            Memory::Numa(m) => unsafe { std::slice::from_raw_parts_mut(m.ptr.as_ptr(), m.size) },
        }
    }
}

pub struct NumaMemory {
    ptr: NonNull<u8>,
    size: usize,
}

impl Drop for NumaMemory {
    fn drop(&mut self) {
        unsafe {
            numa_free(self.ptr.as_ptr() as *mut c_void, self.size);
        }
        println!("dropped");
    }
}

pub fn allocate_layer(sector_size: usize) -> Memory {
    unsafe {
        let ptr = numa_alloc_local(sector_size as c_size_t) as *mut u8;
        if ptr.is_null() {
            println!("raw, vec");
            Memory::Raw(Vec::with_capacity(sector_size))
        } else {
            println!("raw, numa");
            Memory::Numa(NumaMemory {
                ptr: NonNull::new_unchecked(ptr),
                size: sector_size,
            })
        }
    }
}

#[inline]
fn round_up(unrounded: usize, align: usize) -> Result<usize, ()> {
    if align.is_power_of_two() {
        Ok((unrounded + align - 1) & !(align - 1))
    } else {
        Err(())
    }
}
