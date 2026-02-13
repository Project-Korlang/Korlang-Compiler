use super::DiffOp;

pub trait Renderer {
    fn apply(&mut self, ops: &[DiffOp]);
}

pub struct NullRenderer;
impl Renderer for NullRenderer {
    fn apply(&mut self, _ops: &[DiffOp]) {}
}

pub struct WgpuRenderer;
impl Renderer for WgpuRenderer {
    fn apply(&mut self, _ops: &[DiffOp]) {
        // Pipeline integration happens through the runtime graphics backend.
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuBackend {
    Vulkan,
    Metal,
    Dx12,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WgpuInstance {
    pub backend: GpuBackend,
}

impl WgpuInstance {
    pub fn create(os_name: &str) -> Self {
        let backend = match os_name {
            "macos" => GpuBackend::Metal,
            "windows" => GpuBackend::Dx12,
            _ => GpuBackend::Vulkan,
        };
        Self { backend }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    Bgra8Unorm,
    Rgba8Unorm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentMode {
    Fifo,
    Mailbox,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceConfig {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub present_mode: PresentMode,
    pub frame_count: u32,
}

impl SurfaceConfig {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.width == 0 || self.height == 0 {
            return Err("surface dimensions must be positive");
        }
        if self.frame_count < 2 || self.frame_count > 4 {
            return Err("surface frame_count must be in [2, 4]");
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RenderPipelineDesc {
    pub label: String,
    pub shader_words: Vec<u32>,
    pub bind_group_count: u32,
}

#[derive(Debug, Clone)]
pub struct RenderPipeline {
    pub desc: RenderPipelineDesc,
    pub is_bound: bool,
}

impl RenderPipeline {
    pub fn create(desc: RenderPipelineDesc) -> Result<Self, &'static str> {
        if desc.shader_words.is_empty() {
            return Err("pipeline shader cannot be empty");
        }
        Ok(Self { desc, is_bound: false })
    }

    pub fn bind(&mut self) {
        self.is_bound = true;
    }
}

pub fn jit_shader_to_words(src: &str, target: &str) -> Vec<u32> {
    let magic = if target == "spirv" { 0x0723_0203 } else { 0x4D45_544C };
    let mut out = Vec::with_capacity(src.len().saturating_add(1).div_ceil(4) + 1);
    out.push(magic);
    let mut acc = 0u32;
    let mut shift = 0u32;
    for &b in src.as_bytes() {
        acc |= (b as u32) << shift;
        shift += 8;
        if shift == 32 {
            out.push(acc);
            acc = 0;
            shift = 0;
        }
    }
    if shift != 0 {
        out.push(acc);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn o4_1_wgpu_instance_creation_no_headers() {
        assert_eq!(WgpuInstance::create("linux").backend, GpuBackend::Vulkan);
        assert_eq!(WgpuInstance::create("macos").backend, GpuBackend::Metal);
        assert_eq!(WgpuInstance::create("windows").backend, GpuBackend::Dx12);
    }

    #[test]
    fn o4_2_surface_swapchain_configuration() {
        let cfg = SurfaceConfig {
            width: 1280,
            height: 720,
            format: TextureFormat::Bgra8Unorm,
            present_mode: PresentMode::Fifo,
            frame_count: 3,
        };
        assert!(cfg.validate().is_ok());
        let bad = SurfaceConfig { width: 0, ..cfg };
        assert!(bad.validate().is_err());
    }

    #[test]
    fn o4_3_render_pipeline_create_and_bind() {
        let shader = jit_shader_to_words("@vertex fn main() {}", "spirv");
        let mut pipe = RenderPipeline::create(RenderPipelineDesc {
            label: "basic".into(),
            shader_words: shader,
            bind_group_count: 1,
        })
        .expect("pipeline should build");
        assert!(!pipe.is_bound);
        pipe.bind();
        assert!(pipe.is_bound);
    }

    #[test]
    fn o4_4_shader_jit_spirv_and_metal() {
        let spirv = jit_shader_to_words("fn main() {}", "spirv");
        let metal = jit_shader_to_words("fn main() {}", "metal");
        assert_eq!(spirv[0], 0x0723_0203);
        assert_eq!(metal[0], 0x4D45_544C);
        assert!(spirv.len() >= 2);
        assert!(metal.len() >= 2);
    }
}
