use core::alloc::{GlobalAlloc, Layout};
use core::fmt::{Debug, Display, Formatter};
use core::ops::{Deref, DerefMut};
use core::ptr;
use crate::memory::global_alloc::GLOBAL_ALLOC;

#[allow(dead_code)]
pub struct Box<T: ?Sized> {
    ptr: *mut T
}

impl<T> Box<T> {
    // ***** Public Functions

    // ***** Private Functions

    // ***** Struct Init
    pub fn new(value: T) -> Box<T> {
        let layout = Layout::for_value(&value);
        let new_ptr = unsafe { GLOBAL_ALLOC.alloc(layout) as *mut T };

        unsafe { ptr::write(new_ptr, value); }

        Box{
            ptr: new_ptr
        }
    }
}

impl<T: ?Sized> Box<T> where for<'a> &'a T: Clone{
    pub fn from(value: &T) -> Box<T> where T: ptr::Pointee<Metadata = usize>{
        let layout = Layout::for_value(value);
        let size = layout.size();
        let new_ptr: *mut T = unsafe { ptr::from_raw_parts(GLOBAL_ALLOC.alloc(layout), size) as *const T } as *mut T;
        let byte_ptr: *const u8 = ptr::from_ref(value) as *const u8;

        unsafe { ptr::copy_nonoverlapping(byte_ptr, new_ptr as *mut u8, size) };

        Box{
            ptr: new_ptr
        }
    }
}

impl<T: ?Sized> Drop for Box<T>{
    fn drop(&mut self){
        unsafe{
            GLOBAL_ALLOC.dealloc(self.ptr as *mut u8, Layout::for_value_raw(self.ptr));
        }
    }
}

impl<T: ?Sized> Deref for Box<T>{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe{
            &*(self.ptr)
        }
    }
}

impl<T: ?Sized> DerefMut for Box<T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{
            &mut *self.ptr
        }
    }
}

impl<T: ?Sized> AsRef<T> for Box<T>{
    fn as_ref(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized + Display> Display for Box<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<T: ?Sized + Debug> Debug for Box<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}