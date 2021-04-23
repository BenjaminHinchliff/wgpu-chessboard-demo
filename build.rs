use anyhow::{bail, Context, Result};
use globset::Glob;
use std::{fs, path::PathBuf};

struct ShaderData {
    src: String,
    src_path: PathBuf,
    spv_path: PathBuf,
    kind: shaderc::ShaderKind,
}

impl ShaderData {
    pub fn load(src_path: PathBuf) -> Result<ShaderData> {
        let extension = src_path
            .extension()
            .context("File has no extension")?
            .to_str()
            .context("Extension cannot be converted to &str")?;
        let kind = match extension {
            "vert" => shaderc::ShaderKind::Vertex,
            "frag" => shaderc::ShaderKind::Fragment,
            "comp" => shaderc::ShaderKind::Compute,
            _ => bail!("Unsupported shader type: {}", src_path.display()),
        };

        let src = fs::read_to_string(src_path.clone())?;
        let spv_path = src_path.with_extension(format!("{}.spv", extension));

        Ok(ShaderData {
            src,
            src_path,
            spv_path,
            kind,
        })
    }
}

fn main() -> Result<()> {
    let shader_glob = Glob::new("*.{vert,frag,comp}")
        .context("Unable to create shader glob")?
        .compile_matcher();
    let mut compiler = shaderc::Compiler::new().context("Unable to create shader compiler")?;
    for file in fs::read_dir("src/shaders/").context("shader directory doesn't exist")? {
        let path = file?.path();
        if shader_glob.is_match(&path) {
            let ShaderData {
                src,
                src_path,
                spv_path,
                kind,
            } = ShaderData::load(path)?;

            let compiled = compiler.compile_into_spirv(
                &src,
                kind,
                src_path.to_str().unwrap(),
                "main",
                None,
            )?;
            fs::write(spv_path, compiled.as_binary_u8())?;
            println!("cargo:rerun-if-changed={}", src_path.to_str().unwrap());
        }
    }

    Ok(())
}
