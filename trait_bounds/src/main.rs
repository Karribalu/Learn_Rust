trait Double {
    fn double(&self) -> Self;
}
impl Double for i32 {
    fn double(&self) -> Self {
        self * 2
    }
}
impl Double for f32 {
    fn double(&self) -> Self {
        self * 2.0
    }
}

fn quadriple<T: Double>(x: T) -> T {
    x.double().double()
}
fn main() {
    let hello = 10;
    println!("{} {}", quadriple(30_i32), quadriple(50.0));
}
