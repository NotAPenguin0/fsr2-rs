use std::path::Path;
use std::process::{Command, Output};

#[cfg(all(feature = "vk", feature = "dx12"))]
compile_error!("Only one of the Vulkan or DirectX12 backends may be enabled.");

use anyhow::{anyhow, Result};

const FSR2_SOURCE_DIR: &'static str = "./src/vendor/fsr2";

fn output_success(output: Output) -> Result<()> {
    if output.status.success() {
        Ok(())
    } else {
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

/// This function does the equivalent of GenerateSolutions.bat in the FSR2 repository,
/// except it only initializes one API.
fn initialize_api_build_dir(api: &str) -> Result<()> {
    let api_build_dir = Path::new(FSR2_SOURCE_DIR).join("build").join(api);
    std::fs::create_dir_all(&api_build_dir)?;
    let output = Command::new("cmake")
        .arg("-A x64")
        .arg(format!("-DGFX_API={api}"))
        .arg(format!("-S {FSR2_SOURCE_DIR}"))
        .arg(format!(
            "-B {}",
            api_build_dir.as_os_str().to_str().unwrap()
        ))
        .output()?;

    output_success(output)
}

fn main() -> Result<()> {
    #[cfg(feature = "vk")]
    const API: &'static str = "VK";

    #[cfg(feature = "dx12")]
    const API: &'static str = "DX12";

    checkout()?;
    initialize_api_build_dir(API)?;

    Ok(())
}
