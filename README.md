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

### How to build the example application

Get the source

    git clone https://github.com/chunky125/sel-claw.git

Change to the folder

    cd sel-claw

First install sel-claw-build.  This is the build tool for rust seL4 projects
provided by sel-claw, it must be installed before compiling the example 
application.

    cd sel-claw-build 
    cargo install --path . --bin sel-claw-build --features bin --force

Then you can build the example application

    cd ..
    cd example-application
    sel-claw-build build --platform virt --sel4_arch aarch64

sel-claw-build will download the seL4 sources and compile the example application. 
You can then simulate the new application using:

    sel-claw-build simulate --platform virt --sel4_arch aarch64



## Status

Very scratchy!
