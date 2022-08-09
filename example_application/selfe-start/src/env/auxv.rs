use elf::ProgramHeader64 as ElfProgramHeader;
use selfe_sys::seL4_BootInfo;
use selfe_sys::seL4_IPCBuffer;

/// Not a full implementation for now, just the ones we need
#[repr(u32)]
#[repr(C)]
pub enum AuxVariable {
    Null = 0,
    Load = 1,
    Dynamic = 2,
    Phdr(*const ElfProgramHeader) = 3,
    Phent(usize) = 4,
    Phnum(usize) = 5,
    PtTls = 7,
    PtNum = 8,
    AtSysInfo = 32,
    AtSel4BootInfo(*const seL4_BootInfo) = 64,
    AtSel4CSpaceDescriptor = 65,
    AtSel4VSysCall = 66,
    AtSel4IPCBufferPtr(*mut seL4_IPCBuffer) = 67,
    AtSel4IPCBuffer = 68,
    AtSel4TCB(usize) = 69,
    AtSel4CNode = 70,
    AtSel4VSpace = 71,
    AtSel4AsidPool = 72
}

