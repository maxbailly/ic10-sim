mod stack;

use stack::Stack;

/// Represents an IC10 chip.
///
/// An IC10 chip has 16 registers (r0 to r15) holding `f64` values, two special registers that are the return address register (`ra`) pointer and stack pointer (`sp`).
///
/// The chip has a stack of 512 `f64` values.
#[derive(Debug, Default)]
pub struct Chip {
    registers: [f64; 16],
    ra: usize,

    stack: Stack,
}

impl Chip {
    /// Returns a new default [`Chip`].
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the registers values of the chip.
    #[inline(always)]
    pub fn registers(&self) -> &[f64; 16] {
        &self.registers
    }

    /// Returns the value of the return address register.
    #[inline(always)]
    pub fn return_address(&self) -> usize {
        self.ra
    }

    /// Returns the value of the stack pointer register.
    #[inline(always)]
    pub fn stack_pointer(&self) -> usize {
        self.stack.pointer()
    }

    /// Returns the values of the stack.
    #[inline(always)]
    pub fn stack(&self) -> &[f64; 512] {
        self.stack.values()
    }
}
