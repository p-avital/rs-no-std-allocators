
type MallocFunction = unsafe extern "C" fn(usize) -> *mut core::ffi::c_void;
type FreeFunction = unsafe extern "C" fn(*mut core::ffi::c_void);

pub struct CAllocator {
    malloc: MallocFunction,
    free: FreeFunction,
}

unsafe extern "C" fn __malloc_placeholder(_: usize) -> *mut core::ffi::c_void {
    panic!("malloc function not provided")
}
unsafe extern "C" fn __free_placeholder(_: *mut core::ffi::c_void) {
    panic!("free function not provided")
}

pub static mut RUST_CSTD_ALLOCATOR: CAllocator = CAllocator {malloc: __malloc_placeholder, free: __free_placeholder};

#[no_mangle]
pub unsafe extern "C" fn rust_allocator_set_alloc_functions(malloc: MallocFunction, free: FreeFunction) {
    RUST_CSTD_ALLOCATOR.malloc = malloc;
    RUST_CSTD_ALLOCATOR.free = free;
}

unsafe impl core::alloc::GlobalAlloc for CAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        (self.malloc)(layout.size()) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        (self.free)(ptr as *mut core::ffi::c_void)
    }
}

impl CAllocator {
    pub fn malloc<T>(&self) -> *mut T {
        use core::alloc::GlobalAlloc;
        unsafe {self.alloc(core::alloc::Layout::new::<T>()) as *mut T}
    }
    pub fn array<T>(&self, n: usize) -> *mut T {
        unsafe {(self.malloc)(core::alloc::Layout::new::<T>().size() * n) as *mut T}
    }
    pub fn free<T>(&self, ptr: *mut T) {
        unsafe {(self.free)(ptr as *mut core::ffi::c_void)}
    }
}