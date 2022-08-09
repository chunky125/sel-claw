pub mod auxv;
mod tls;

use selfe_sys::seL4_BootInfo;
use selfe_sys::seL4_IPCBuffer;
use self::auxv::AuxVariable;
use crate::elf::ProgramHeader64 as ElfProgramHeader;
use crate::elf::ProgramHeaderType as ElfProgramHeaderType;

/// ELF Header Structure
#[repr (C)]
struct ProgramHeaders {
   count: usize,
   size: usize,
   headers: *const ElfProgramHeader
}

extern "C" {
    fn _sel_claw_write_tpidr_el0(tls_base: *const ThreadEnvironment);
}


/// Thread Environment Header
#[repr (C)]
pub struct ThreadEnvironment {
    // ProcessName
    process_name : *const u8,

    // BootInfo
    boot_info: *const seL4_BootInfo,

    // TLS Base
    initial_tls_base: *const u8,

    // Inital thread control block
    initial_tcb : usize,

    // IPC Buffer
    initial_ipc_buffer : *mut seL4_IPCBuffer,

    // ELF Headers
    elf_headers: ProgramHeaders,    

    // TLS Images
    local_storage: tls::ThreadLocalStorage,

    // Arguments
    argv: *const *const u8,
    argc: usize,
    
    // Auxiliary Vector
    auxv: *const auxv::AuxVariable,
    
    // Environment Vector
    envp: *const *const u8,
    
    // Exit Callbacks

}

const MIN_ALIGN_BYTES : u64  = 16;

/// The environment for this thread
static mut THREAD_ENVIRONMENT : ThreadEnvironment = ThreadEnvironment {

    process_name: core::ptr::null::<u8>(),
    
    boot_info: core::ptr::null::<seL4_BootInfo>(),

    initial_tls_base: core::ptr::null::<u8>(),

    initial_tcb: 0,

    initial_ipc_buffer: core::ptr::null_mut::<seL4_IPCBuffer>(),

    elf_headers: ProgramHeaders {
        count: 0,
        size: 0,
        headers: core::ptr::null::<ElfProgramHeader>(),
    },

    local_storage: tls::ThreadLocalStorage {
        image: 0,
        image_size: 0,
        memory_size: 0,
        region_size: 0,
        align: 0,
        offset: 0
    },
    
    
    // It would be nice to rustify these, but should we maintain
    // the same structure as sel4runtime?
    argv: core::ptr::null::<*const u8>(),
    argc: 0,

    envp: core::ptr::null::<*const u8>(),

    auxv: core::ptr::null::<auxv::AuxVariable>()
};




/// Initialise a thread environment from process
pub unsafe fn load_thread_environment(argc : usize,
                           argv : *const *const u8,
                           envp : *const *const u8,
                           auxv : *const auxv::AuxVariable) 
    -> &'static ThreadEnvironment {

    //Empty TLS
    THREAD_ENVIRONMENT.local_storage.empty();     

    // Parse AUXV
    THREAD_ENVIRONMENT.parse_auxv(auxv);        
    
    // Parse Program Headers
    THREAD_ENVIRONMENT.parse_phdrs();
    
    // Get name from argv[0] if argc > 1
    if argc > 1 {
        THREAD_ENVIRONMENT.process_name = *argv;
    }

    // Init static TLS
    THREAD_ENVIRONMENT.local_storage.try_init_static(
        &mut THREAD_ENVIRONMENT.initial_ipc_buffer,
        &mut THREAD_ENVIRONMENT.initial_tls_base);
    
    // Initialise our environment
    THREAD_ENVIRONMENT.argc = argc;
    THREAD_ENVIRONMENT.argv = argv;
    THREAD_ENVIRONMENT.auxv = auxv;
    THREAD_ENVIRONMENT.envp = envp;

    // Run constructors
    
    // Return reference to the thread environment
    &THREAD_ENVIRONMENT

}

/// Get the sel4_BootInfo reference
pub unsafe fn bootinfo() -> &'static seL4_BootInfo {
    
    &(*THREAD_ENVIRONMENT.boot_info)
}



impl ThreadEnvironment {

    /// Parse auxillary variables into our ThreadEnvironment
    unsafe fn parse_auxv(&mut self, auxv : *const auxv::AuxVariable) {
    
        let mut currv = auxv;

        loop {
            match *currv {

                AuxVariable::Null => 
                    break,

                AuxVariable::Phdr(addr) => 
                    self.elf_headers.headers = addr,

                AuxVariable::Phent(size) => 
                    self.elf_headers.size = size as usize,
                    
                AuxVariable::Phnum(count) =>
                    self.elf_headers.count = count as usize,

                AuxVariable::AtSel4BootInfo(boot_info) =>
                    self.boot_info = boot_info,

                AuxVariable::AtSel4IPCBufferPtr(ipc_buffer_ptr) =>
                    self.initial_ipc_buffer = ipc_buffer_ptr,

                AuxVariable::AtSel4TCB(initial_tcb) =>
                    self.initial_tcb = initial_tcb as usize,

                _ => {},
            }

            // Move to next one
            currv = currv.add(1);
        }
    }

    /// Parse program headers
    unsafe fn parse_phdrs(&mut self) {

        for i in 0..self.elf_headers.count {
            let header : *const ElfProgramHeader = self.elf_headers.headers.add(i);

            match (*header).type_ {

                ElfProgramHeaderType::Tls =>
                    self.local_storage.load(header),

                _ => {}
            }


        }
    }


    fn get_argv() -> &'static str {
        "bob"
    }

    fn process_name() -> &'static str {
        "bob"
    }

    fn get_argc() -> usize {
        1
    }

    fn get_envp() -> usize {
        0
    }

    fn auxv() -> usize {
        0
    }


    fn get_tls_size() -> usize {
        0
    }
}

