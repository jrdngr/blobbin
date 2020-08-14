use wgpu::{Device, ShaderModule};

use super::ShaderCompiler;

pub fn fragment_module(
    device: &Device,
    compiler: &mut ShaderCompiler,
) -> anyhow::Result<ShaderModule> {
    let fs_src = include_str!("basic.frag");
    let fs_data = compiler.create_fragment_shader(fs_src, "basic.frag", "main")?;
    Ok(device.create_shader_module(&fs_data))
}

pub fn vertex_module(
    device: &Device,
    compiler: &mut ShaderCompiler,
) -> anyhow::Result<ShaderModule> {
    let vs_src = include_str!("basic.vert");
    let vs_data = compiler.create_vertex_shader(vs_src, "basic.vert", "main")?;
    Ok(device.create_shader_module(&vs_data))
}
