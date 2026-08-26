use std::{
    fs,
    path::{Path, PathBuf},
    process::{self, Command},
};

use speckylang::compiler::emit_llvm;

pub fn compile(source: &str, output: Option<&Path>) {
    let output = output
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("specky_program"));
    let ir_path = output.with_extension("ll");
    fs::write(&ir_path, emit_llvm(source)).unwrap_or_else(|error| {
        eprintln!("could not write LLVM IR: {error}");
        process::exit(1);
    });

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target = if cfg!(target_os = "windows") {
        "x86_64-pc-windows-gnu"
    } else {
        ""
    };
    let mut build_args = vec!["build", "--release", "--lib"];
    if !target.is_empty() {
        build_args.extend(["--target", target]);
    }
    let status = Command::new("cargo")
        .args(build_args)
        .current_dir(&manifest_dir)
        .status()
        .unwrap_or_else(|error| {
            eprintln!("could not start cargo: {error}");
            process::exit(1);
        });
    if !status.success() {
        process::exit(status.code().unwrap_or(1));
    }

    let library = if !target.is_empty() {
        manifest_dir.join(format!("target/{target}/release/libspeckylang.a"))
    } else {
        manifest_dir.join("target/release/libspeckylang.a")
    };
    let status = Command::new("clang")
        .arg(&ir_path)
        .arg(&library)
        .args(if cfg!(target_os = "windows") {
            vec!["-lntdll", "-lws2_32", "-luserenv", "-lbcrypt", "-ladvapi32"]
        } else {
            Vec::new()
        })
        .arg("-o")
        .arg(&output)
        .status()
        .unwrap_or_else(|error| {
            eprintln!(
                "could not start clang; install LLVM/Clang and try again: {error}"
            );
            process::exit(1);
        });
    if !status.success() {
        process::exit(status.code().unwrap_or(1));
    }
}