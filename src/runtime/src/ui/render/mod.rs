use super::{DiffOp, ViewNode};

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
        // TODO: integrate wgpu pipeline
    }
}

