#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

// #[cfg(feature = "alloc")]
extern crate alloc;

use core::{alloc::Layout, ptr::NonNull};

mod dma;

pub use dma::alloc::{r#box::DBox, vec::DVec};
pub use dma::slice::{DSlice, DSliceMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    ToDevice,
    FromDevice,
    Bidirectional,
}

pub trait Impl {
    /// map virt address to physical address
    fn map(addr: NonNull<u8>, size: usize, direction: Direction) -> u64;
    /// unmap virt address
    fn unmap(addr: NonNull<u8>, size: usize);
    /// write cache back to memory
    fn flush(addr: NonNull<u8>, size: usize);
    /// invalidate cache
    fn invalidate(addr: NonNull<u8>, size: usize);

    /// alloc memory.
    ///
    /// # Safety
    ///
    /// layout must have non-zero size. Attempting to allocate for a zero-sized layout may result in undefined behavior.
    #[allow(unused_variables)]
    unsafe fn alloc(layout: Layout) -> *mut u8 {
        #[cfg(feature = "alloc")]
        unsafe {
            alloc::alloc::alloc(layout)
        }
        #[cfg(not(feature = "alloc"))]
        core::ptr::null_mut()
    }

    /// Deallocates the block of memory at the given `ptr` pointer with the given `layout`.
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    ///
    /// * `ptr` is a block of memory currently allocated via this allocator and,
    ///
    /// * `layout` is the same layout that was used to allocate that block of
    ///   memory.
    ///
    /// Otherwise undefined behavior can result.
    #[allow(unused_variables)]
    unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
        #[cfg(feature = "alloc")]
        unsafe {
            alloc::alloc::dealloc(ptr, layout)
        }
    }
}

extern "Rust" {
    fn __dma_api_map(addr: NonNull<u8>, size: usize, direction: Direction) -> u64;
    fn __dma_api_unmap(addr: NonNull<u8>, size: usize);
    fn __dma_api_flush(addr: NonNull<u8>, size: usize);
    fn __dma_api_invalidate(addr: NonNull<u8>, size: usize);
    fn __dma_api_alloc(layout: Layout) -> *mut u8;
    fn __dma_api_dealloc(ptr: *mut u8, layout: Layout);
}

fn map(addr: NonNull<u8>, size: usize, direction: Direction) -> u64 {
    unsafe { __dma_api_map(addr, size, direction) }
}

fn unmap(addr: NonNull<u8>, size: usize) {
    unsafe { __dma_api_unmap(addr, size) }
}

fn flush(addr: NonNull<u8>, size: usize) {
    unsafe { __dma_api_flush(addr, size) }
}

fn invalidate(addr: NonNull<u8>, size: usize) {
    unsafe { __dma_api_invalidate(addr, size) }
}

unsafe fn alloc(layout: Layout) -> *mut u8 {
    unsafe { __dma_api_alloc(layout) }
}

unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    unsafe { __dma_api_dealloc(ptr, layout) }
}

#[macro_export]
macro_rules! set_impl {
    ($t: ty) => {
        #[no_mangle]
        fn __dma_api_map(
            addr: core::ptr::NonNull<u8>,
            size: usize,
            direction: $crate::Direction,
        ) -> u64 {
            <$t as $crate::Impl>::map(addr, size, direction)
        }
        #[no_mangle]
        fn __dma_api_unmap(addr: core::ptr::NonNull<u8>, size: usize) {
            <$t as $crate::Impl>::unmap(addr, size)
        }
        #[no_mangle]
        fn __dma_api_flush(addr: core::ptr::NonNull<u8>, size: usize) {
            <$t as $crate::Impl>::flush(addr, size)
        }
        #[no_mangle]
        fn __dma_api_invalidate(addr: core::ptr::NonNull<u8>, size: usize) {
            <$t as $crate::Impl>::invalidate(addr, size)
        }
        #[no_mangle]
        fn __dma_api_alloc(layout: core::alloc::Layout) -> *mut u8 {
            unsafe { <$t as $crate::Impl>::alloc(layout) }
        }
        #[no_mangle]
        fn __dma_api_dealloc(ptr: *mut u8, layout: core::alloc::Layout) {
            unsafe { <$t as $crate::Impl>::dealloc(ptr, layout) }
        }
    };
}
