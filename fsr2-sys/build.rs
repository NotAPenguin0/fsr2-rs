use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[cfg(all(feature = "vk", feature = "dx12"))]
compile_error!("Only one of the Vulkan or DirectX12 backends may be enabled.");

use anyhow::{anyhow, Result};
use std::env;
use fs_extra::dir::CopyOptions;

const FSR2_SOURCE_DIR: &'static str = "./src/vendor/fsr2";

fn output_success(output: Output) -> Result<()> {
    if output.status.success() {
        Ok(())
    } else {
        eprintln!("Build error: {}", String::from_utf8(output.stdout).unwrap());
        Err(anyhow!("{}", String::from_utf8(output.stderr).unwrap()))
    }
}

/// Checks out submodules for FSR2
fn checkout() -> Result<()> {
    let output = Command::new("git")
        .current_dir(FSR2_SOURCE_DIR)
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .arg("--recursive")
        .output()?;

    output_success(output)
}

fn build_dir(api: &str) -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    Path::new(&out_dir).join("fsr2-build").join(api)
}

/// This function does the equivalent of GenerateSolutions.bat in the FSR2 repository,
/// except it only initializes one API.
fn initialize_api_build_dir(api: &str) -> Result<()> {
    println!("Generating FSR2 build files for backend {}", api);
    let api_build_dir = build_dir(api);
    std::fs::create_dir_all(&api_build_dir)?;
    let output = Command::new("cmake")
        .arg("-A x64")
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg(format!("-DGFX_API={api}"))
        .arg(format!("-S {FSR2_SOURCE_DIR}"))
        .arg(format!(
            "-B {}",
            api_build_dir.as_os_str().to_str().unwrap()
        ))
        .output()?;

    output_success(output)
}

fn build_fsr2_lib(api: &str) -> Result<()> {
    println!("Building FSR2 library for backend {}", api);
    let api_build_dir = build_dir(api);
    let output = Command::new("cmake")
        .arg("--build")
        .arg(format!("{}", api_build_dir.as_os_str().to_str().unwrap()))
        .arg("--config Release")
        .output()?;
    output_success(output)
}

fn copy_dir_contents<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    println!("In copy");
    let err = fs_extra::dir::copy(&src, &dst, &CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
        copy_inside: true,
        content_only: true,
        depth: 0,
    });
    if let Err(e) = err {
        println!("Copy error: {e}");
    }
    Ok(())
}

fn root_out_dir() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    Path::new(&out_dir).join("../../../")
}

/// FSR2 outputs all compilation artifacts to $FSR2_SOURCE_DIR/bin
/// This is not really desirable, so we will copy all interesting files to a better directory and then
/// delete this.
fn copy_build_artifacts() -> Result<()> {
    let fsr2_bin_dir = Path::new(FSR2_SOURCE_DIR).join("bin");
    let root_out_dir = root_out_dir();
    copy_dir_contents(fsr2_bin_dir.join("ffx_fsr2_api"), &root_out_dir)?;
    std::fs::remove_dir_all(&fsr2_bin_dir)?;
    Ok(())
}

fn main() -> Result<()> {
    #[cfg(feature = "vk")]
    const API: &'static str = "VK";

    #[cfg(feature = "dx12")]
    const API: &'static str = "DX12";

    checkout()?;
    initialize_api_build_dir(API)?;
    build_fsr2_lib(API)?;
    println!("FSR2 build success");
    copy_build_artifacts()?;

    let dir = root_out_dir();
    println!("cargo:rustc-link-search={}", dir.as_os_str().to_str().unwrap());
    println!("cargo:rustc-link-lib=./ffx_fsr2_api_x64");

    #[cfg(feature = "vk")]
    println!("cargo:rustc-link-lib=./ffx_fsr2_api_vk_x64");

    #[cfg(feature = "dx12")]
    println!("cargo:rustc-link-lib=./ffx_fsr2_api_dx12_x64");

    Ok(())
}
