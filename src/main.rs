use std::ffi::CString;

use nix::unistd::execvp;

fn main() {
    let ls_cmd = Command {
        program: "ls".to_string(),
        args: vec!["-l".to_string(), "-a".to_string()],
    };

    exec_external_command(ls_cmd);
}

#[derive(Debug, Clone)]
struct Command {
    program: String,
    args: Vec<String>,
}

fn exec_external_command(cmd: Command) {
    let args = cmd
        .args
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect::<Vec<_>>();

    match execvp(&CString::new(cmd.program).unwrap(), &args) {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    }
}
