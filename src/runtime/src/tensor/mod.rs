use crate::arc::{korlang_arc_alloc_mmap, korlang_arc_ptr, korlang_arc_release, ArcBuf};
use std::slice;
pub mod jit;

#[derive(Clone, Debug)]
pub struct Tensor {
    pub buf: *mut ArcBuf,
    pub len: usize,
    pub dims: Vec<usize>,
}

impl Tensor {
    pub fn new(dims: &[usize]) -> Self {
        let len = dims.iter().product::<usize>();
        let bytes = len * std::mem::size_of::<f32>();
        let buf = unsafe { korlang_arc_alloc_mmap(bytes) };
        Self { buf, len, dims: dims.to_vec() }
    }

    pub fn data_mut(&self) -> &mut [f32] {
        let ptr = unsafe { korlang_arc_ptr(self.buf) } as *mut f32;
        unsafe { slice::from_raw_parts_mut(ptr, self.len) }
    }

    pub fn data(&self) -> &[f32] {
        let ptr = unsafe { korlang_arc_ptr(self.buf) } as *const f32;
        unsafe { slice::from_raw_parts(ptr, self.len) }
    }
}

impl Drop for Tensor {
    fn drop(&mut self) {
        unsafe { korlang_arc_release(self.buf) };
    }
}

pub fn tensor_add(a: &Tensor, b: &Tensor) -> Tensor {
    let mut out = Tensor::new(&a.dims);
    for i in 0..a.len {
        out.data_mut()[i] = a.data()[i] + b.data()[i];
    }
    out
}

pub fn tensor_mul(a: &Tensor, b: &Tensor) -> Tensor {
    let mut out = Tensor::new(&a.dims);
    for i in 0..a.len {
        out.data_mut()[i] = a.data()[i] * b.data()[i];
    }
    out
}

// Automatic differentiation (minimal)
#[derive(Clone, Debug)]
pub struct TensorGrad {
    pub value: Tensor,
    pub grad: Option<Vec<f32>>,
}

impl TensorGrad {
    pub fn new(value: Tensor) -> Self {
        Self { value, grad: None }
    }

    pub fn backward(&mut self) {
        self.grad = Some(vec![1.0; self.value.len]);
    }
}

// Model loading stub
pub fn load_onnx(_path: &str) -> Result<(), String> {
    Err("ONNX loader not integrated".into())
}
