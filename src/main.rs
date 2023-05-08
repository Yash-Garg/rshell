use std::{
    ffi::CString,
    io::{stdin, stdout, Write},
};

use nix::{
    errno::Errno,
    libc,
    sys::wait::{waitpid, WaitPidFlag},
    unistd::{execvp, fork, write, ForkResult},
};

#[derive(Debug, Clone)]
struct Command {
    program: String,
    args: Vec<CString>,
}

impl Command {
    fn get_input() -> Self {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or_default();

        let args = parts
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<_>>();

        Self {
            program: command.to_string(),
            args,
        }
    }
}

fn main() {
    loop {
        print!("rshell> ");
        stdout().flush().unwrap();

        let cmd = Command::get_input();

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                if child.as_raw() != 0 {
                    waitpid(child, Some(WaitPidFlag::WUNTRACED)).unwrap();
                }
            }

            Ok(ForkResult::Child) => {
                if cmd.program.is_empty() {
                    continue;
                }

                let result = execvp(&CString::new(cmd.program).unwrap(), &cmd.args);
                match result {
                    Ok(_) => {}
                    Err(e) => eprint(e),
                }
            }

            Err(e) => eprint(e),
        }
    }
}

fn eprint(error: Errno) {
    write(libc::STDOUT_FILENO, format!("{}\n", error).as_bytes()).unwrap();
}
