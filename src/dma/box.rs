use core::alloc::Layout;

use crate::Direction;

use super::DCommon;

pub struct DBox<T> {
    inner: DCommon<T>,
}

impl<T> DBox<T> {
    const SIZE: usize = size_of::<T>();

    pub fn zero(direction: Direction) -> Option<Self> {
        let layout = Layout::new::<T>();

        Some(Self {
            inner: DCommon::zeros(layout, direction)?,
        })
    }
    pub fn bus_addr(&self) -> u64 {
        self.inner.bus_addr
    }

    pub fn read(&self) -> T {
        unsafe {
            let ptr = self.inner.addr;

            self.inner.preper_read(ptr, Self::SIZE);

            ptr.read_volatile()
        }
    }

    pub fn write(&mut self, value: T) {
        unsafe {
            let ptr = self.inner.addr;

            ptr.write_volatile(value);

            self.inner.confirm_write(ptr, Self::SIZE);
        }
    }

    pub fn modify(&mut self, f: impl FnOnce(&mut T)) {
        unsafe {
            let mut ptr = self.inner.addr;

            self.inner.preper_read(ptr, Self::SIZE);

            f(ptr.as_mut());

            self.inner.confirm_write(ptr, Self::SIZE);
        }
    }
}
