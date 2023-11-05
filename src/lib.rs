/* Copyright (c) 2015 The Robigalia Project Developers
 * Licensed under the Apache License, Version 2.0
 * <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT
 * license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
 * at your option. All files in the project carrying such
 * notice may not be copied, modified, or distributed except
 * according to those terms.
 */

#![no_std]
#![no_main]
#![feature(lang_items, core_intrinsics, naked_functions, thread_local)]

/// seL4 Types & Bindings, types taken from selfe-sys, with rust-bingen
/// used to generate bindings from libsel4
mod bindings;

/// ELF Headers and types
mod elf;

/// Thread environment
pub mod env;

/// Debugging Utilities
pub mod debug;

/// Startup functions
mod start;

#[cfg(feature = "panic_handler")]
mod panic;


use debug::DebugOutHandle;
use core::fmt::Write;
use core::panic::PanicInfo;
use elf::ProgramHeader64 as ElfProgramHeader;
use elf::ProgramHeaderType as ElfProgramHeaderType;

pub use bindings::*;

#[allow(unused)]
pub fn debug_panic_handler(info: &PanicInfo) -> ! {
    let _res = writeln!(DebugOutHandle, "*** Panic: {:#?}", info);

    unsafe {
        core::intrinsics::abort();
    }
}

#[lang = "eh_personality"]
#[cfg(not(test))]
pub fn eh_personality() {
    core::intrinsics::abort();
}

/// This should never be called!
#[lang = "start"]
#[cfg(not(test))]
pub fn lang_start<T: Termination + 'static>(
    main: fn() -> T,
    _argc: isize,
    _argv: *const *const u8,
    sigpipe: u8,
) -> isize {
    main();
    0
}

#[lang = "termination"]
#[cfg(not(test))]
pub trait Termination {
    fn report(self) -> i32;
}

#[cfg(not(test))]
impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}
