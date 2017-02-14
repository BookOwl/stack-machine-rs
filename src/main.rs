struct Frame {
    num_bindings: usize,
    bindings: Vec<Value>,
}
enum Value {
    Int(i32),
    Float(f32),
}
type Stack = Vec<Frame>;

fn main() {
    println!("Hello, world!");
}
