extern crate simple_error;
#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::process::Command;
use std::option::Option;
use std::io::Error;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_pid: i32 = match args.len() {
        1 => 1,
        2 => i32::from_str_radix(&args[1], 10).expect("could not parse arg to int"),
        _ => panic!("too many arguments"),
    };

    println!("{:?}", bsd_ps(target_pid));
}

#[derive(Debug)]
struct ProcessInfo {
    pid: i32,
    command: String,
    ppid: Option<i32>,

}

fn bsd_ps(pid: i32) -> Result<ProcessInfo, Error> {
    lazy_static! {
        static ref SPACES: Regex = Regex::new(r"\s+").unwrap();
    }

    let out = try!(Command::new("ps")
        .arg("-p").arg(&pid.to_string())
        .arg("-o").arg("pid=")       // pid: always get this, for debug
        .arg("-o").arg("ppid=")      // ppid: parent PID
        .arg("-o").arg("command=")
        .output());  // command: some kinda args

    // TODO: handle non-zero exit
    // TODO: handle parse errors for real

    let stdout = String::from_utf8(out.stdout).unwrap();
    let fields: Vec<&str> = SPACES.split(&stdout.trim()).collect();

    let pid: &str = fields[0];
    let ppid = fields[1];
    let command = fields[2..fields.len()].join(" ");

    return Ok(ProcessInfo {
        pid: i32::from_str_radix(pid, 10).unwrap(),
        ppid: match ppid {
            "0" => None,
            _ => Some(i32::from_str_radix(ppid, 10).unwrap()),
        },
        command: command,
    })
}
