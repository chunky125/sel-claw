extern crate selfe_config;
extern crate cc;

use selfe_config::build_helpers::*;

use cc::Build;

fn main() {
    BuildEnv::request_reruns();
    let config = load_config_from_env_or_default();
    config.print_boolean_feature_flags();

    // Compile the assembler instructions
    let assembly_modules = [ "entry", "entry_rootserver", "registers" ];

    // I want this to pull in a feature/config variable...
    let mut define_stack_size = String::from("-DCONFIG_SEL4RUNTIME_ROOT_STACK=16384");

    for module_name in assembly_modules {

       let mut file_path = String::from("src/arch/aarch64/");
       file_path.push_str(module_name);
       file_path.push_str(".S");

       Build::new()
           .file(file_path)
            .warnings(true)
           .no_default_flags(false)
           .flag(&define_stack_size)
           .flag("-nostdlib")
           .flag("-fno-builtin")
           .flag("-nolibc")
           .compiler("aarch64-linux-gnu-gcc")
           .compile(module_name);
    }

    println!("cargo:rerun-if-change=src/arch/aarch64/entry");
}
