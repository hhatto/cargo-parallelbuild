use std::process::Command;
use std::thread;

fn exec_build(mode: &str) {
    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("build");
    match mode {
        "release" => {
            cargo_command.arg("--release");
        }
        _ => {}
    }

    let output = cargo_command.output().expect("fail");
    if !output.status.success() {
        println!("{:?}", output);
        return;
    }
    let stderr = output.stderr;
    println!("{} build:", mode);
    println!("{}", std::str::from_utf8(&stderr).unwrap());
}

fn main() {
    let mut children = vec![];
    for mode in vec!["release", "debug"].into_iter() {
        children.push(thread::spawn(move || {
            exec_build(mode);
        }));
    }

    for child in children {
        let ret = child.join();
        if ret.is_err() {
            println!("{:?}", ret);
        }
    }
}
