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

### Prequisites

You'll need the following base packages:
- C and C++ compilers for target architecture
- Linking/Binary tools for target architecture
- QEMU for target architecture
- Rust Build Environment
- CMake
- Device Tree Compiler (dtc)
- Python with pyfdt, jinja2, six, future, ply, libarchive, pyelftools modules
- libxml2 tools (xmllint specifically)

### Installing selfe build tool from selfe-config

selfe is the build tool for rust seL4 projects provided by sel-claw.  At this moment 
it must be installed carefully so as to ensure that the correct version is installed 
rather than the selfe-sys package provided by Auxon corp.  

    cargo install --path path_to_selfe-config --bin selfe --features bin --force

To check that this has installed correctly, run 

    selfe --version

This should report the version as 0.2.2 or greater.

## Status

Very scratchy!
