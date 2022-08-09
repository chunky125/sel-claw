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
#![feature(lang_items, core_intrinsics, asm, naked_functions, llvm_asm, thread_local)]
#![cfg_attr(
    any(
        all(target_arch = "arm", target_pointer_width = "32"),
        target_arch = "aarch64"
    ),
    feature(global_asm)
)]

extern crate selfe_sys;

mod elf;
pub mod env;
pub mod debug;
#[cfg(feature = "panic_handler")]
mod panic;

mod start;


use debug::DebugOutHandle;
use core::fmt::Write;
use core::panic::PanicInfo;
use selfe_sys::*;
use elf::ProgramHeader64 as ElfProgramHeader;
use elf::ProgramHeaderType as ElfProgramHeaderType;

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

#[lang = "start"]
#[cfg(not(test))]
pub fn lang_start<T: Termination + 'static>(
    main: fn() -> T,
    _argc: isize,
    _argv: *const *const u8,
) -> isize {
    main();
    0
}

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

