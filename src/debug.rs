use core::fmt;
use crate::bindings::seL4_DebugPutChar;

pub struct DebugOutHandle;

impl fmt::Write for DebugOutHandle {
    #[cfg(KernelPrinting)]
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for &b in s.as_bytes() {
            unsafe { seL4_DebugPutChar(b as i8) };
        }
        Ok(())
    }

    #[cfg(not(KernelPrinting))]
    fn write_str(&mut self, _s: &str) -> ::core::fmt::Result {
        Ok(())
    }
}
