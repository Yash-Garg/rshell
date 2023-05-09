use std::{
    ffi::CString,
    io::{stdin, stdout, Write},
};

use nix::{
    libc,
    sys::wait::{waitpid, WaitPidFlag},
    unistd::{chdir, execvp, fork, write, ForkResult},
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

fn main() {
    loop {
        print!("rsh> ");
        stdout().flush().unwrap();

        let cmd = Command::get_input();

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                if child.as_raw() > 0 {
                    waitpid(child, Some(WaitPidFlag::WUNTRACED)).unwrap();
                } else {
                    std_print("Error: fork failed")
                }
            }

            Ok(ForkResult::Child) => match cmd.program.as_str() {
                "cd" => {
                    let path = if cmd.args.len() < 2 {
                        continue;
                    } else {
                        cmd.args[1].to_str().unwrap()
                    };

                    match chdir(path) {
                        Ok(_) => {}
                        Err(e) => std_print(e.desc()),
                    }

                    continue;
                }

                _ => {
                    let result = execvp(&CString::new(cmd.program).unwrap(), &cmd.args);
                    match result {
                        Ok(_) => {}
                        Err(e) => std_print(e.desc()),
                    }
                }
            },

            Err(e) => std_print(e.desc()),
        }
    }
}

fn std_print(message: &str) {
    write(libc::STDOUT_FILENO, format!("{}\n", message).as_bytes()).unwrap();
}
