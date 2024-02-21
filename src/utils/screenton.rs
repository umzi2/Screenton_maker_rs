
use ndarray::{Array2, ArrayView2};
use crate::utils::dot;


pub struct Screenton {
    dot_size: usize,
    dot: Array2<f32>,
    dot_inv: Array2<f32>,
    lx_plus: usize,
    ly_plus: usize,
}

impl Screenton {
    pub fn new(dot_size: usize, lx_plus: usize, ly_plus:usize) -> Self {
        let (dot, dot_inv) = dot::create_dot(dot_size);

        Self {
            dot_size,
            dot,
            dot_inv,
            lx_plus,
            ly_plus
        }
    }
    pub fn run(&mut self, array: &mut Array2<f32>) {

        let (height,width)=(array.shape()[0],array.shape()[1]);

        for ly in 0..height {
            for lx in 0..width {
                let lx2 = lx+self.lx_plus;
                let ly2 = ly+self.ly_plus;
                let row = lx2 / self.dot_size;
                let column = ly2 / self.dot_size;
                let i = lx2 % self.dot_size;
                let j = ly2 % self.dot_size;

                let src: ArrayView2<f32> = if (row + column) % 2 == 1 {
                    self.dot_inv.view()
                } else {
                    self.dot.view()
                };
                if array[[ly, lx]] < src[[i,j]] {
                    array[[ly, lx]] = 0.0;
                } else {
                    array[[ly, lx]] = 1.0
                }
            }
        }

    }
}
