use std::fmt::Display;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "( {} {} )\n( {} {} )\n",
            self.0, self.1, self.2, self.3
        ))
    }
}

fn transpose(m: &Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}
pub fn main(_: Vec<String>) {
    let m = Matrix(1f32, 2f32, 3f32, 4f32);
    println!("{}\n{}", m, transpose(&m));
}
