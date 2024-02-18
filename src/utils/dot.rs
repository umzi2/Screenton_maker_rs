use ndarray::Array2;

const X: f32 = 0.1;
const Y: f32 = 0.15;

fn math(dot_size: usize) -> (f32, (f32, f32)) {
    let point = (dot_size as f32 / 2.0 + X, dot_size as f32 / 2.0 + Y);
    let step = (1.0 - 0.5) / ((dot_size as f32).powi(2) - 1.0);
    return (step, point);
}

fn dot_inv(dot_size: usize) -> Array2<f32> {
    let mut coordinates_and_values: Vec<(usize, usize, f32)> = vec![];
    let mut dot: Array2<f32> = Array2::zeros((dot_size, dot_size));
    let (step, point) = math(dot_size);
    for i in 0..dot_size {
        for b in 0..dot_size {
            let value = ((i as f32 - point.0).powi(2) + (b as f32 - point.1).powi(2)).sqrt();
            dot[[i, b]] = value;
            coordinates_and_values.push((i, b, value))
        }
    }


    // Сортируем вектор по значению (в порядке убывания)
    coordinates_and_values.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut n = 0;

    for &(i, j, _) in &coordinates_and_values {
        let s = 0.5 - (step * n as f32);
        dot[[i, j]] = s;
        n += 1;
    }

    return dot;
}

fn dot(dot_inv: Array2<f32>) -> Array2<f32> {
    let dot = dot_inv * -1.0 + 1.0;

    return dot;
}

pub fn create_dot(dot_size: usize) -> (Array2<f32>, Array2<f32>) {
    let dot_inv = dot_inv(dot_size);
    let dot = dot(dot_inv.clone());
    let dot_inv = dot_inv + 0.003;
    return (dot, dot_inv);
}

