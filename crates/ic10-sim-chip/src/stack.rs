/// IC10 stack.
///
/// The stack can hold 512 `f64` values.
#[derive(Debug)]
pub(crate) struct Stack {
    stack: [f64; Self::SIZE],
    pointer: usize,
}

impl Stack {
    /// The number of values hold by the stack.
    pub(crate) const SIZE: usize = 512;

    /// Returns the value of the stack pointer.
    #[inline(always)]
    pub(crate) fn pointer(&self) -> usize {
        self.pointer
    }

    /// Returns the values of the stack.
    #[inline(always)]
    pub(crate) fn values(&self) -> &[f64; Self::SIZE] {
        &self.stack
    }
}

impl Default for Stack {
    #[inline(always)]
    fn default() -> Self {
        Self {
            stack: [0.; Self::SIZE],
            pointer: 0,
        }
    }
}
