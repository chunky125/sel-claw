# sel-claw

This is a fork of [selfe-start](https://github.com/auxoncorp/selfe-sys/tree/
master/example_application/selfe-start) which is itself a fork of 
[sel4-start](https://gitlab.com/robigalia/sel4-start).

sel-claw aims to provide a rust-centric runtime for seL4 development, it 
incorportates functionality found in selfe-start (sel4 startup), selfe-sys
 (bindings to libsel4), and eventually ferros (rust aligned interfaces).

The primary development target is aarch64 on the various raspberry pi
platforms.

## Getting Started

You'll need the following base packages:
- C and C++ compilers for target architecture
- Linking/Binary tools for target architecture
- QEMU for target architecture
- Rust Build Environment
- CMake
- Device Tree Compiler (dtc)
- Python with pyfdt, jinja2, six, future, ply modules
- libxml2 tools (xmllint specifically)

## Status

Very scratchy!
