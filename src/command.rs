use std::{ffi::CString, io::stdin};

#[derive(Debug, Clone)]
pub struct Command {
    pub program: String,
    pub args: Vec<CString>,
}

impl Command {
    pub fn get() -> Self {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap_or_default();

        let parts = input.trim().split_whitespace();
        let command = parts.to_owned().nth(0);

        let args = parts
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<_>>();

        Self {
            program: command.unwrap_or_default().to_string(),
            args,
        }
    }
}
