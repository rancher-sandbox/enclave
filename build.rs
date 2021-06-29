use std::path::Path;

use libbpf_cargo::SkeletonBuilder;

#[derive(thiserror::Error, Debug)]
enum BuildError {
    #[error("could not convert to string")]
    StrconvError,
}

fn main() -> anyhow::Result<()> {
    let bpf_dir = Path::new("src").join("bpf");
    let src = bpf_dir.join("enclave.bpf.c");
    let skel = bpf_dir.join("mod.rs");
    let clang = match std::env::var("CLANG") {
        Ok(val) => val,
        Err(_) => std::string::String::from("/usr/bin/clang"),
    };

    SkeletonBuilder::new(&src).clang(clang).generate(&skel)?;
    println!(
        "cargo:rerun-if-changed={}",
        src.to_str().ok_or(BuildError::StrconvError)?
    );

    Ok(())
}