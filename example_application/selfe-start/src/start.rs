extern crate selfe_sys;

use selfe_sys::seL4_BootInfo;
use selfe_sys::seL4_CapInitThreadTCB;
use crate::elf::ProgramHeader64 as ElfProgramHeader;
use crate::elf::ProgramHeaderType as ElfProgramHeaderType;
use crate::env::auxv::AuxVariable;
use crate::env;

/// Thread data and bss segments, these cannot
/// exist for root-server, so we create them in the image
extern "C" {
    static _tdata_start : u8;
    static _tdata_end : u8;
    static _tbss_end : u8;
}

/// Reference to the main function
extern "Rust" {
    fn main() -> i32;    
}

#[repr(align(4096))]
#[doc(hidden)]
/// A wrapper around our stack so that we can specify its alignment requirement.
struct Stack {
    stack: [u8; STACK_SIZE],
}

/// The size of the initial root thread stack. This stack is located in the root
/// task image data section.
pub const STACK_SIZE: usize = 1024 * 68;

#[used]
#[doc(hidden)]
/// The stack for our initial root task thread.
static mut STACK: Stack = Stack {
    stack: [0u8; STACK_SIZE],
};

#[no_mangle]
/// This is the entry for a root task, there are no useful things passed
/// other than the BootInfo, we need to create everything ourselves!
///
/// Initialise thread specific data structures (TLS) from the bootinfo
/// Can only be called once - it sets a private flag when it is called 
/// and will not modify `BOOTINFO` if that flag is set.
pub unsafe extern "C" fn __sel4_start_root(bootinfo: *const seL4_BootInfo) {

    let tdata_start = &_tdata_start as *const u8 as u64;
    let tdata_end = &_tdata_end as *const u8 as u64;
    let tbss_end = &_tbss_end as *const u8 as u64;

    // Create an ELF Header
    let root_task_tls_header = ElfProgramHeader {
        type_ : ElfProgramHeaderType::Tls,
        flags : 0,
        offset : 0,
        virtual_addr : tdata_start,
        physical_addr : 0,
        file_size : tdata_end - tdata_start, // Define these in linker
        mem_size : tbss_end - tdata_start, // Define in linker
        align : core::mem::align_of::<usize>() as u64,
    };

    // Create variable array
    let process_name = "root_server";
    let argv : [*const u8; 2] = [process_name.as_ptr(),
                                 core::ptr::null::<u8>()];
    let sel4_true_str = "seL4=1";
    let envp : [*const u8; 2] = [sel4_true_str.as_ptr(),
                                 core::ptr::null::<u8>()];
    let auxv : [AuxVariable; 7] = [
            AuxVariable::Phent (core::mem::size_of::<ElfProgramHeader>()),
            AuxVariable::Phnum (1),
            AuxVariable::Phdr (&root_task_tls_header),
            AuxVariable::AtSysInfo,
            AuxVariable::AtSel4BootInfo (bootinfo),
            AuxVariable::AtSel4TCB (seL4_CapInitThreadTCB as usize),
            AuxVariable::Null
    ];

    __sel4_start_main(2, 
                      &argv as *const *const u8, 
                      &envp as *const *const u8,
                      &auxv as *const AuxVariable);

}

#[no_mangle]
/// Entry point for non-root programs
///
/// Simply passed pointer to the stack, with stack being:
///
/// argument count
/// array of argument pointers
/// empty string
/// array of environment pointers
/// null terminator
/// array of auxiliary vector entries
/// zero auxialiary vector
/// unspecified data
///
pub unsafe extern "C" fn __sel4_start(stack : *const usize)
{
    // First word is the argument count
    let argc : usize = *stack;

    // Second word is the start of the argument vector
    let argv : *const *const u8 = stack.offset(1) as *const *const u8;

    // Environment pointer is next
    let envp : *const *const u8 = stack.offset(2) as *const *const u8;

    // Get count of environment variables
    let mut envc : isize = 0;
    while (*stack.offset(2 + envc)) != 0
    {
        envc = envc + 1;
    }

    // Get auxiliary vector
    let auxv : *const AuxVariable = 
        stack.offset(3 + envc) as *const AuxVariable;

    // Auxillary vector follows the environment pointer vector
    __sel4_start_main(argc, argv, envp, auxv);
}

/// Entry point after setup for root or a normal program
pub unsafe fn __sel4_start_main(
//    main: fn() -> i32,
    argc: usize,
    argv: *const *const u8,
    envp: *const *const u8,
    auxv: *const AuxVariable)
{
   
    let thread_env = env::load_thread_environment(argc, argv, envp, auxv);

    // Launch main
    let retval = main();

    // Run exit callbacks if needed
    // sel4runtime_exit(retval);
}
