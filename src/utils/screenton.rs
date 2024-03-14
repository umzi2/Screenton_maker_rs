use ndarray::Array2;
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
        let dot_size = self.dot_size;
        let dot = &self.dot;
        let dot_inv = &self.dot_inv;
        let ly_plus = &self.ly_plus;
        let lx_plus = &self.lx_plus;
        let mut src_values:f32;
        let mut colum :usize;
        let(w,h)=(array.shape()[0],array.shape()[1]);
        let ww=0..w;
        let hh=0..h;
        for ly in ww {
            let ly2 = ly+ly_plus;
            colum = ly2/self.dot_size;
            for lx in hh.clone() {
                let value = &mut array[[ly, lx]];
                if *value > 0.0 && *value < 1.0 {
                    let lx2 = lx+lx_plus;
                    src_values= if (colum+ lx2/dot_size) % 2 == 1 {
                        dot_inv[[lx2 % dot_size,ly2 % dot_size]]
                    } else {
                        dot[[lx2 % dot_size,ly2 % dot_size]]
                    };
                    let src_value = src_values;
                    *value = if *value < src_value { 0.0 } else { 1.0 };
                }
            }
        }
    }
}

