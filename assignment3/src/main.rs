use std::env;
use std::io::{self, Read, Write};
use std::process::{self, Command, Stdio};

fn run_child() -> io::Result<()> {
    let child_pid = process::id();
    eprintln!("Child [PID {}] started", child_pid);

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let received = input.trim_end_matches(['\n', '\r']);

    eprintln!("Child [PID {}] received data: {}", child_pid, received);
    let transformed = received.to_uppercase();
    eprintln!("Child [PID {}] transformed data", child_pid);

    println!("{}", transformed);
    Ok(())
}

fn run_parent() -> io::Result<()> {
    let parent_pid = process::id();
    println!("Parent [PID {}] starting", parent_pid);

    let current_executable = env::current_exe()?;
    let mut child = Command::new(current_executable)
        .arg("--child")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_pid = child.id();
    println!(
        "Parent [PID {}] spawned child [PID {}]",
        parent_pid, child_pid
    );

    let message = "hello from parent process";
    println!(
        "Parent [PID {}] sending data to child [PID {}]: {}",
        parent_pid, child_pid, message
    );

    let mut child_stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::new(io::ErrorKind::BrokenPipe, "Failed to access child stdin"))?;
    child_stdin.write_all(message.as_bytes())?;
    child_stdin.write_all(b"\n")?;
    drop(child_stdin);

    let mut response = String::new();
    let mut child_stdout = child
        .stdout
        .take()
        .ok_or_else(|| io::Error::new(io::ErrorKind::BrokenPipe, "Failed to access child stdout"))?;
    child_stdout.read_to_string(&mut response)?;

    let status = child.wait()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Child exited with status: {}", status),
        ));
    }

    println!(
        "Parent [PID {}] received response from child [PID {}]: {}",
        parent_pid,
        child_pid,
        response.trim_end()
    );

    Ok(())
}

fn main() {
    let is_child = env::args().any(|arg| arg == "--child");

    let result = if is_child { run_child() } else { run_parent() };
    if let Err(err) = result {
        eprintln!("Process [PID {}] error: {}", process::id(), err);
        process::exit(1);
    }
}
