use crate::{flush, invalidate, map, unmap, Direction};
use alloc::vec::Vec;
use core::{
    alloc::Layout,
    mem::{self, size_of},
    ptr::NonNull,
};

pub mod r#box;
pub mod slice;
pub mod vec;

struct DCommon<T> {
    addr: NonNull<T>,
    bus_addr: u64,
    layout: Layout,
    direction: Direction,
}

impl<T> DCommon<T> {
    pub fn zeros(layout: Layout, direction: Direction) -> Option<Self> {
        unsafe {
            let addr = NonNull::new(alloc::alloc::alloc_zeroed(layout))?;
            let bus_addr = map(addr, layout.size(), direction);
            flush(addr, layout.size());
            Some(Self {
                bus_addr,
                addr: addr.cast(),
                layout,
                direction,
            })
        }
    }

    pub fn from_vec(mut value: Vec<T>, direction: Direction) -> Self {
        unsafe {
            let layout = Layout::from_size_align_unchecked(
                value.capacity() * size_of::<T>(),
                align_of::<T>(),
            );

            let addr = NonNull::new(value.as_mut_ptr()).unwrap();

            mem::forget(value);

            let bus_addr = map(addr.cast(), layout.size(), direction);
            flush(addr.cast(), layout.size());
            Self {
                bus_addr,
                addr: addr.cast(),
                layout,
                direction,
            }
        }
    }

    pub fn preper_read(&self, ptr: NonNull<u8>, size: usize) {
        self.direction.preper_read(ptr, size);
    }

    pub fn confirm_write(&self, ptr: NonNull<u8>, size: usize) {
        self.direction.confirm_write(ptr, size);
    }
}

impl<T> Drop for DCommon<T> {
    fn drop(&mut self) {
        if self.layout.size() > 0 {
            unmap(self.addr.cast(), self.layout.size());

            unsafe { alloc::alloc::dealloc(self.addr.as_ptr() as _, self.layout) };
        }
    }
}

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
