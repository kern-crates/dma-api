use core::{alloc::Layout, ops::Deref};

use crate::Direction;

use super::DCommon;

pub struct DVec<T> {
    inner: DCommon<T>,
}

impl<T> DVec<T> {
    const T_SIZE: usize = size_of::<T>();

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

            self.inner.preper_read(ptr, Self::T_SIZE);

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

            self.inner.confirm_write(ptr, Self::T_SIZE);
        }
    }

    fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.inner.addr.as_ptr(), self.len()) }
    }
}

impl<T: Copy> DVec<T> {
    pub fn copy_from_slice(&mut self, src: &[T]) {
        assert!(src.len() <= self.len());

        self.as_slice_mut().copy_from_slice(src);

        self.inner
            .confirm_write(self.inner.addr, Self::T_SIZE * src.len());
    }
}

impl<T> Deref for DVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.inner
            .preper_read(self.inner.addr, Self::T_SIZE * self.len());
        unsafe { core::slice::from_raw_parts(self.inner.addr.as_ptr(), self.len()) }
    }
}
