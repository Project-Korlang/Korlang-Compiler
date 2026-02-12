use super::{Tensor, tensor_add, tensor_mul};

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Mul,
    Add,
}

// Fused kernel execution (single-pass) for ops like a * b + c
pub fn jit_fused(a: &Tensor, b: &Tensor, c: Option<&Tensor>, ops: &[Op]) -> Tensor {
    let mut out = Tensor::new(&a.dims);
    for i in 0..a.len {
        let mut v = a.data()[i];
        for op in ops {
            match op {
                Op::Mul => v *= b.data()[i],
                Op::Add => v += c.map(|t| t.data()[i]).unwrap_or(0.0),
            }
        }
        out.data_mut()[i] = v;
    }
    out
}

pub fn jit_from_pipeline(a: &Tensor, b: &Tensor, c: &Tensor) -> Tensor {
    // a -> multiply(b) -> add(c)
    jit_fused(a, b, Some(c), &[Op::Mul, Op::Add])
}

