use typenum::Unsigned;
pub struct StackedAllocator<T, N> where N: Unsigned {
    buffers: [T; <N as Unsigned>::to_usize()],
    stack: [usize; <N as Unsigned>::to_usize()],
    stack_cursor: usize,
    n: core::marker::PhantomData<N>,
}

impl<T, N: typenum::Unsigned> StackedAllocator<T, N> {
    fn new() -> Self {
        unsafe {
            let buffers = core::mem::uninitialized();
            let mut stack = core::mem::uninitialized();

            StackedAllocator {buffers, stack, stack_cursor: 0, n: core::marker::PhantomData::default()}
        }
    }
}