mod command;

use nix::sys::signal::{signal, SigHandler, Signal::SIGINT};

extern "C" fn handler(_: i32) {
    println!("Caught SIGINT");
}

fn main() {
    loop {
        let handled = unsafe { signal(SIGINT, SigHandler::Handler(handler)) };

        match handled {
            Ok(_) => {}
            Err(_) => println!("Error: signal handler failed"),
        }
    }

    // Command::listen();
}
