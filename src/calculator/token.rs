#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i32),
    Operator(char),
}
