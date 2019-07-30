mod calculator;

fn main() {
    let mut interpreter = calculator::Interpreter::new("5+3");
    println!("{:?}", interpreter.eval());
    println!("Hello, world!");
}
