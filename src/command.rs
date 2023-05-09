use std::io::{stdout, Write};
use std::{ffi::CString, io::stdin};

use nix::{libc, unistd::write};

use nix::{
    sys::wait::{waitpid, WaitPidFlag},
    unistd::{chdir, execvp, fork, ForkResult},
};

#[derive(Debug, Clone)]
pub struct Command {
    pub program: String,
    pub args: Vec<CString>,
}

#[allow(dead_code)]
impl Command {
    pub fn listen() {
        loop {
            let user = std::env::var("USER").unwrap_or_default();
            let user_dir = std::env::current_dir().unwrap();

            print!("{} in {}> ", user, user_dir.display());
            stdout().flush().unwrap();

            let cmd = Self::get_input();
            match unsafe { fork() } {
                Ok(ForkResult::Parent { child, .. }) => {
                    if child.as_raw() > 0 {
                        waitpid(child, Some(WaitPidFlag::WUNTRACED)).unwrap();
                    } else {
                        Self::print("Error: fork failed")
                    }
                }

                Ok(ForkResult::Child) => match cmd.program.as_str() {
                    "exit" => std::process::exit(0),

                    "cd" => {
                        let path = if cmd.args.len() < 2 {
                            continue;
                        } else {
                            cmd.args[1].to_str().unwrap()
                        };

                        match chdir(path) {
                            Ok(_) => {}
                            Err(e) => Self::print(e.desc()),
                        }

                        continue;
                    }

                    _ => {
                        let filename = CString::new(cmd.program.as_str()).unwrap();
                        let result = execvp(&filename, &cmd.args);

                        match result {
                            Ok(_) => {}
                            Err(_) => Self::print(
                                format!("{}: command not found", filename.to_string_lossy())
                                    .as_str(),
                            ),
                        }
                    }
                },

                Err(e) => Self::print(e.desc()),
            }
        }
    }

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

    fn print(message: &str) {
        write(libc::STDOUT_FILENO, format!("{}\n", message).as_bytes()).unwrap();
    }
}
