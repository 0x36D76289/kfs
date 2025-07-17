use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Assemble boot.S to boot.o
    let boot_o = out_dir.join("boot.o");
    let mut cmd = Command::new("as");
    cmd.arg("--32")
       .arg("boot/boot.S")
       .arg("-o")
       .arg(&boot_o);
    
    let output = cmd.output().expect("Failed to execute assembler");
    if !output.status.success() {
        panic!("Failed to assemble boot.S: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Link the boot object file
    println!("cargo:rustc-link-arg={}", boot_o.display());
    
    // Lier le script de linkage
    println!("cargo:rustc-link-arg=-T");
    println!("cargo:rustc-link-arg=boot/linker.ld");
    
    // Assembler le code de boot
    println!("cargo:rerun-if-changed=boot/boot.S");
    println!("cargo:rerun-if-changed=boot/linker.ld");
    
    // Configuration pour le target bare metal
    println!("cargo:rustc-link-arg=-nostdlib");
    println!("cargo:rustc-link-arg=-static");
}
