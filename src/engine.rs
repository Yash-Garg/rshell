use crate::command::Command;

use std::ffi::CString;
use std::io::{stdout, Write};
use std::process::exit;

use nix::sys::signal::Signal::SIGINT;
use nix::sys::signal::{signal, SigHandler};

use nix::{
    sys::wait::{waitpid, WaitPidFlag},
    unistd::{chdir, execvp, fork, ForkResult},
};

pub struct Engine {}

impl Engine {
    pub fn listen() {
        unsafe {
            // Ignore SIGINT in parent process
            signal(SIGINT, SigHandler::SigIgn).unwrap();
        }

        loop {
            let user = std::env::var("USER").unwrap_or_default();
            let user_dir = std::env::current_dir().unwrap();

            // Show prompt with user and current directory
            print!("{} in {}> ", user, user_dir.display());
            stdout().flush().unwrap();

            let cmd = Command::get();

            match cmd.program.as_str() {
                "exit" => exit(1),

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

                _ => match unsafe { fork() } {
                    Ok(ForkResult::Parent { child, .. }) => {
                        unsafe {
                            // Restore default SIGINT handler in child process
                            signal(SIGINT, SigHandler::SigDfl).unwrap();
                        }

                        if child.as_raw() > 0 {
                            waitpid(child, Some(WaitPidFlag::WUNTRACED)).unwrap();
                        } else {
                            eprintln!("fork failed");
                            exit(1);
                        }
                    }

                    Ok(ForkResult::Child) => {
                        let filename = CString::new(cmd.program.as_str()).unwrap();
                        let result = execvp(&filename, &cmd.args);

                        match result {
                            Ok(_) => {}
                            Err(_) => {
                                eprintln!("{}: command not found", filename.to_str().unwrap())
                            }
                        }
                    }

                    Err(e) => eprintln!("{}", e),
                },
            }
        }
    }
}
