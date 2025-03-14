use crate::{flush, invalidate, Direction};
use core::ptr::NonNull;

pub mod slice;
pub mod alloc;

impl Direction {
    pub fn preper_read(self, ptr: NonNull<u8>, size: usize) {
        if matches!(self, Direction::FromDevice | Direction::Bidirectional) {
            invalidate(ptr, size);
        }
    }
    pub fn confirm_write(self, ptr: NonNull<u8>, size: usize) {
        if matches!(self, Direction::ToDevice | Direction::Bidirectional) {
            flush(ptr, size)
        }
    }
}
