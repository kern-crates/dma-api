use core::{alloc::Layout, ptr::NonNull};

use crate::{flush, invalidate, map, unmap, Direction};

pub mod r#box;
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
            Some(Self {
                bus_addr,
                addr: addr.cast(),
                layout,
                direction,
            })
        }
    }

    pub fn preper_read(&self, ptr: NonNull<T>, size: usize) {
        if matches!(
            self.direction,
            Direction::FromDevice | Direction::Bidirectional
        ) {
            invalidate(ptr.cast(), size);
        }
    }

    pub fn preper_write(&self, ptr: NonNull<T>, size: usize) {
        if matches!(
            self.direction,
            Direction::ToDevice | Direction::Bidirectional
        ) {
            flush(ptr.cast(), size)
        }
    }
}

impl<T> Drop for DCommon<T> {
    fn drop(&mut self) {
        unmap(self.addr.cast(), self.layout.size());

        unsafe { alloc::alloc::dealloc(self.addr.as_ptr() as _, self.layout) };
    }
}
