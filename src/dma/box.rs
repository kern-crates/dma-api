use core::alloc::Layout;

use crate::Direction;

use super::DmaCommon;

pub struct DBox<T> {
    inner: DmaCommon<T>,
}

impl<T> DBox<T> {
    pub fn zero(direction: Direction) -> Option<Self> {
        let layout = Layout::new::<T>();

        Some(Self {
            inner: DmaCommon::zeros(layout, direction)?,
        })
    }
    pub fn bus_addr(&self) -> u64 {
        self.inner.bus_addr
    }

    pub fn read(&self) -> T {
        unsafe {
            let ptr = self.inner.addr;

            self.inner.preper_read(ptr);

            ptr.read_volatile()
        }
    }

    pub fn write(&mut self, value: T) {
        unsafe {
            let ptr = self.inner.addr;

            ptr.write_volatile(value);

            self.inner.preper_write(ptr);
        }
    }

    pub fn modify(&mut self, f: impl FnOnce(&mut T)) {
        unsafe {
            let mut ptr = self.inner.addr;

            self.inner.preper_read(ptr);

            f(ptr.as_mut());

            self.inner.preper_write(ptr);
        }
    }
}
