/*use core::mem::transmute;

#[repr(C)]
pub struct FatPointer<T: ?Sized>{
    ptr: *mut T,
    size: usize,
}

impl<T: ?Sized> FatPointer<T>{
    pub unsafe fn into_ref<'a>(self) -> &'a T {
        transmute::<Self, &T>(self)
    }

    // ***** Struct Init *****
    pub fn new(ptr: *mut T, size: usize) -> FatPointer<T> {
        FatPointer{
            ptr,
            size
        }
    }
}*/