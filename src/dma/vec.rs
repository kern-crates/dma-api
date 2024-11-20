use core::alloc::Layout;

use crate::Direction;

use super::DCommon;

pub struct DVec<T> {
    inner: DCommon<T>,
}

impl<T> DVec<T> {
    pub fn zeros(len: usize, align: usize, direction: Direction) -> Option<Self> {
        let size = len * size_of::<T>();
        let layout = Layout::from_size_align(size, align).unwrap();

        Some(Self {
            inner: DCommon::zeros(layout, direction)?,
        })
    }
    pub fn len(&self) -> usize {
        self.inner.layout.size() / size_of::<T>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn bus_addr(&self) -> u64 {
        self.inner.bus_addr
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.len() {
            return None;
        }

        unsafe {
            let ptr = self.inner.addr.add(index);

            self.inner.preper_read(ptr);

            Some(ptr.read_volatile())
        }
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index >= self.len() {
            return;
        }

        unsafe {
            let ptr = self.inner.addr.add(index);

            ptr.write_volatile(value);

            self.inner.preper_write(ptr);
        }
    }
}
