use core::ffi::c_void;
use std::os::raw::c_int;

type c_size_t = usize;

#[link(name = "numa", kind = "dylib")]
extern "C" {
    fn numa_alloc_local(size: c_size_t) -> *mut c_void;
    fn numa_free(start: *mut c_void, size: c_size_t);
    fn numa_getpagesize() -> c_int;
}

#[inline]
fn round_up(unrounded: usize, align: usize) -> Result<usize, ()> {
    if align.is_power_of_two() {
        Ok((unrounded + align - 1) & !(align - 1))
    } else {
        Err(())
    }
}
