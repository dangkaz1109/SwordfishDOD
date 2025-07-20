use std::alloc::{alloc, dealloc, Layout};
use std::mem::size_of;
use std::ptr::null_mut;

pub struct Pool<T> {
    pub block_number: usize,
    pub init_pointer: *mut T,
}

unsafe impl<T> Sync for Pool<T> {}
unsafe impl<T> Send for Pool<T> {}

impl<T> Pool<T> {
    pub fn new(block_number: usize) -> Self {
        let layout = Layout::from_size_align(size_of::<T>() * block_number, size_of::<T>()).unwrap();
        let first_ptr = unsafe { alloc(layout) as *mut T };
        Self {
            block_number,
            init_pointer: first_ptr,
        }
    }

    pub fn write(&self, index: usize, data: T) {
        if index < self.block_number {
            let offset_ptr = unsafe { self.init_pointer.add(index) };
            unsafe {
                offset_ptr.write(data);
            }
        }
    }

    pub fn get(&self, index: usize) -> &mut T {
        if index < self.block_number {
            let offset_ptr = unsafe { self.init_pointer.add(index) };
            unsafe { &mut *offset_ptr }
        } else {
            panic!("Index out of bounds in Pool!");
        }
    }
}

impl<T> Drop for Pool<T> {
    fn drop(&mut self) {
        if !self.init_pointer.is_null() {
            let layout = Layout::from_size_align(size_of::<T>() * self.block_number, size_of::<T>()).unwrap();
            unsafe {
                dealloc(self.init_pointer as *mut u8, layout);
            }
        }
    }
}
