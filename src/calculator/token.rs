#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Integer(i32),
    Operator(char),
    End
}
