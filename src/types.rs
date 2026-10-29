use std::collections::VecDeque;

#[derive(Debug)]
pub enum Value{
    String(String),
    Integer(i64),
    List(VecDeque<String>),
}