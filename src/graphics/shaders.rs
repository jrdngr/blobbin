pub mod basic;

use anyhow::Context;

pub struct ShaderCompiler {
    compiler: shaderc::Compiler,
}

impl ShaderCompiler {
    pub fn new() -> anyhow::Result<Self> {
        let compiler = shaderc::Compiler::new().context("Failed to create shader compiler")?;

        Ok(Self { compiler })
    }

    pub fn create_fragment_shader(
        &mut self,
        source: impl AsRef<str>,
        name: impl AsRef<str>,
        entry_point: impl AsRef<str>,
    ) -> anyhow::Result<Vec<u32>> {
        self.create_shader(source, name, entry_point, shaderc::ShaderKind::Fragment)
    }

    pub fn create_vertex_shader(
        &mut self,
        source: impl AsRef<str>,
        name: impl AsRef<str>,
        entry_point: impl AsRef<str>,
    ) -> anyhow::Result<Vec<u32>> {
        self.create_shader(source, name, entry_point, shaderc::ShaderKind::Vertex)
    }

    pub fn create_compute_shader(
        &mut self,
        source: impl AsRef<str>,
        name: impl AsRef<str>,
        entry_point: impl AsRef<str>,
    ) -> anyhow::Result<Vec<u32>> {
        self.create_shader(source, name, entry_point, shaderc::ShaderKind::Compute)
    }

    fn create_shader(
        &mut self,
        source: impl AsRef<str>,
        name: impl AsRef<str>,
        entry_point: impl AsRef<str>,
        kind: shaderc::ShaderKind,
    ) -> anyhow::Result<Vec<u32>> {
        let spirv = self.compiler.compile_into_spirv(
            source.as_ref(),
            kind,
            name.as_ref(),
            entry_point.as_ref(),
            None,
        )?;
        let data = wgpu::read_spirv(std::io::Cursor::new(spirv.as_binary_u8()))?;

        Ok(data)
    }
}
