/// Holds a pointer internally as a different type. This allows it to be stored in a mutex
/// statically. Do not attempt to retrieve and use the internal pointer.
#[derive(PartialEq, Eq, Hash)]
pub struct PtrHolder {
    ptr: u32,
}

impl PtrHolder {
    pub fn new<T>(ptr: *mut T) -> PtrHolder {
        PtrHolder { ptr: ptr as u32 }
    }
}

unsafe impl Sync for PtrHolder {}

unsafe impl Send for PtrHolder {}
