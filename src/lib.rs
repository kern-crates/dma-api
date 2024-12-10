#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::ptr::NonNull;

mod dma;

#[cfg(feature = "alloc")]
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
}

extern "Rust" {
    fn __dma_api_map(addr: NonNull<u8>, size: usize, direction: Direction) -> u64;
    fn __dma_api_unmap(addr: NonNull<u8>, size: usize);
    fn __dma_api_flush(addr: NonNull<u8>, size: usize);
    fn __dma_api_invalidate(addr: NonNull<u8>, size: usize);
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
    };
}
