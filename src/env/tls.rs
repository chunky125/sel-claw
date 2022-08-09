use env::MIN_ALIGN_BYTES;
use crate::elf::ProgramHeader64 as ElfProgramHeader;
use bindings::seL4_IPCBuffer;
// :use selfe_sys::__sel4_ipc_buffer;

use core::convert::TryInto;

/// Assembly functions

extern "C" {
    // Write the register
    fn _sel_claw_write_tpidr_el0(value: *const u8);

    // Read it
    fn _sel_claw_read_tpidr_el0() -> *const u8;

    // Link to the extern variable
//    static mut __sel4_ipc_buffer : *mut seL4_IPCBuffer;
}



/// Round up memory to correct alignment
pub fn round_up(value: u64, align: u64) -> u64 {

    if value % align == 0 {
        value
    } else {
        value + align - (value % align)
    }
}

/// Static TLS 
const STATIC_TLS_SIZE : usize = 16384;
static STATIC_TLS : [u8; STATIC_TLS_SIZE] = [0; STATIC_TLS_SIZE];

/// TLS Image
#[repr (C)]
pub struct ThreadLocalStorage {
    // Location of initial image in memory
    pub image : u64, 
    // Size of the inital image
    pub image_size : u64,
    // Size needed to store full TLS
    pub memory_size : u64,
    // Size needed to store the TLS and thread structure
    pub region_size : u64,
    // Alignment needed for TLS data
    pub align : u64,
    // Offset of TLS data from thread pointer
    pub offset : u64
}

impl ThreadLocalStorage {

    /// Internal function to calculate region size in memory
    /// based on alignment and buffer areas
    fn update_region_size(&mut self)
    {

        self.region_size = self.align 
            + round_up(core::mem::size_of::<*const ThreadLocalStorage>() as u64, self.align)
            + if cfg!(gap_above_tp = "true") {
            //    round_up(GAP_ABOVE_TP, self.align)
            0
            }else {
             0
            }
            + round_up(self.memory_size, self.align);

    }

    /// Empty the TLS
    pub fn empty(&mut self) {

        self.image = 0;
        self.align = MIN_ALIGN_BYTES;
        self.image_size = 0;
        self.memory_size = 0;
        
        self.update_region_size();
    }

    /// Load Program Header into TLS
    pub unsafe fn load(&mut self, header: *const ElfProgramHeader)  {

        self.image = (*header).virtual_addr;

        if (*header).align > MIN_ALIGN_BYTES {
            self.align = (*header).align;
        } else {
            self.align = MIN_ALIGN_BYTES;
        }

        self.image_size = (*header).file_size;

        self.memory_size = round_up((*header).mem_size, (*header).align);

        self.update_region_size();
    }
        
    /// Init from static TLS
    pub unsafe fn try_init_static(&mut self, 
                           initial_ipc_buffer: &mut *mut seL4_IPCBuffer,
                           initial_thread_tls_base: &mut *const u8) {

        let static_tls_aligned_size = core::mem::size_of::<[u8; STATIC_TLS_SIZE]>() as u64;

        if self.region_size <= static_tls_aligned_size {
            self.move_initial_tls(&STATIC_TLS[0], 
                                  initial_ipc_buffer, 
                                  initial_thread_tls_base);
        }
    }

    /// Move the initial TLS
    unsafe fn move_initial_tls(&mut self, 
                        tls_memory : *const u8,
                        initial_ipc_buffer: &mut *mut seL4_IPCBuffer,
                        initial_thread_tls_base: &mut *const u8) -> *const u8 {
        
        if tls_memory == core::ptr::null::<u8>() {
            core::ptr::null::<u8>()
        } else {
            let tls_base = self.write_tls_image(tls_memory);

            if tls_base == core::ptr::null::<u8>() {
                core::ptr::null::<u8>()
            } else {

                self.set_tls_base(tls_base);

                if *initial_ipc_buffer != 
                    core::ptr::null::<seL4_IPCBuffer>() as *mut seL4_IPCBuffer {
//                   __sel4_ipc_buffer = *initial_ipc_buffer;
                }

                *initial_thread_tls_base = tls_base;

                *initial_thread_tls_base
            }
                
        }
    }

    /// Write TLS Base
    unsafe fn write_tls_image(&mut self, tls_memory: *const u8) -> *const u8 {
        
        if tls_memory == core::ptr::null::<u8>() {
            core::ptr::null::<u8>()
        } else {
            self.copy_tls_data(tls_memory);

            self.tls_base_from_tls_region(tls_memory)
        }
    }

    /// Copy TLS Data
    unsafe fn copy_tls_data(&mut self, tls_region: *const u8) {

        let tls : *mut u8 = self.tls_base_from_tls_region(tls_region) as *mut u8;

        core::ptr::copy(tls, self.image as *mut u8, self.image_size.try_into().unwrap());

        let tbss : *mut u8 = tls.offset(self.image_size.try_into().unwrap());

        let len : usize  = self.memory_size as usize - self.image_size as usize;

        core::ptr::write_bytes(tbss, 0, len);

    }

    /// 
    unsafe fn tls_from_tls_base(&mut self, tls_base: *const u8) -> *const u8 {
    
        let mut tls_addr : *const u8 = tls_base;

        if cfg!(tls_above_tp = "true") {
            tls_addr.offset(0) // GAP_ABOVE_TP
        } else {
            let memory_size : isize = self.memory_size.try_into().unwrap();
            let neg_image_size : isize = memory_size * - 1;

            tls_addr.offset(neg_image_size)
        }

    }

    unsafe fn tls_base_from_tls_region(&mut self, tls_region: *const u8) -> *const u8 {

        let mut tls_base = tls_region;

        if cfg!(tls_above_tp = "true") {
            tls_base.offset(0); // GAP_ABOVE_TP
        }

        round_up(tls_base as u64, self.align) as *const u8
    }

    unsafe fn set_tls_base(&mut self, tls_base: *const u8) {
        
        _sel_claw_write_tpidr_el0(tls_base);

    }
            

}
