extern crate nix;

use std::io::{self, BufRead, Write, stdout};
use std::str;
use std::ffi::CString;
use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::*;

mod builtin;


fn main() {
    rshell_loop();
    std::process::exit(0);
}

fn rshell_loop() {
    loop {
        let stdin = io::stdin();
        let mut line = String::new();

        print!("# ");
        stdout().flush().ok();

        stdin.lock().read_line(&mut line).expect("Could not read line");
        let cstring_line: Vec<CString> =
            line.split_whitespace().map(|ch| CString::new(ch).unwrap()).collect();

        match fork().expect("fork failed") {
            ForkResult::Parent { child } => {
                let wait_status = waitpid(child, None);
                match wait_status {
                    // assert that waitpid returned correct status and the pid is the one of the child
                    Ok(WaitStatus::Exited(pid, status)) => {
                        println!("child process with pid {} exited with status: {}",
                                 pid,
                                 status);
                    }
                    // panic, must never happen
                    Ok(_) => panic!("Child still alive, should never happen"),
                    // panic, waitpid should never fail
                    Err(_) => panic!("Error: waitpid Failed"),
                }
            }
            ForkResult::Child => {
                // exec the command entered
                execvp(&cstring_line[0], &cstring_line).unwrap();
            }
        }
    }
}
