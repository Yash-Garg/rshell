use std::io::{stdout, Write};
use std::{ffi::CString, io::stdin};

use nix::sys::signal::Signal::SIGINT;
use nix::sys::signal::{signal, SigHandler};

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

            // Show prompt with user and current directory
            print!("{} in {}> ", user, user_dir.display());
            stdout().flush().unwrap();

            let cmd = Self::get_input();

            unsafe {
                // Ignore SIGINT in parent process
                signal(SIGINT, SigHandler::SigIgn).unwrap();
            }

            match unsafe { fork() } {
                Ok(ForkResult::Parent { child, .. }) => {
                    if child.as_raw() > 0 {
                        waitpid(child, Some(WaitPidFlag::WUNTRACED)).unwrap();
                    } else {
                        eprintln!("error: fork failed")
                    }
                }

                Ok(ForkResult::Child) => match cmd.program.as_str() {
                    // TODO: This does not work as expected
                    "exit" => std::process::exit(0),

                    "cd" => {
                        let path = if cmd.args.len() < 2 {
                            continue;
                        } else {
                            cmd.args[1].to_str().unwrap()
                        };

                        match chdir(path) {
                            Ok(_) => {}
                            Err(e) => eprintln!("{}", e),
                        }

                        continue;
                    }

                    _ => {
                        // Restore default SIGINT handler
                        unsafe { signal(SIGINT, SigHandler::SigDfl).unwrap() };

                        let filename = CString::new(cmd.program.as_str()).unwrap();
                        let result = execvp(&filename, &cmd.args);

                        match result {
                            Ok(_) => {}
                            Err(_) => {
                                eprintln!("{}: command not found", filename.to_str().unwrap())
                            }
                        }
                    }
                },

                Err(e) => eprintln!("{}", e),
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
}
