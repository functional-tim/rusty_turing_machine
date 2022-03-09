/*
 * main.rs - Console program to simulate a turing machine.
 *
 * (C) 2021 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;

mod turing_machine;

/// Struct for the parameters of the app.
#[derive(StructOpt)]
#[structopt(
    name = "rusty-turing-machine",
    about = "WIP"
)]
enum Command {
    /// Run the Turing machine until it halts (if it halts ;) )
    Run(OptR),

    /// Run only one step of the Turing Turing machine
    Step(OptS),
}

#[derive(StructOpt)]
struct OptR {
    /// Input file
    #[structopt(parse(from_os_str), short = "i", long = "input")]
    file: Option<PathBuf>,

    /// Print every step
    #[structopt(short = "p", long = "print")]
    pr: bool,

    /// Count 1s especially for busy beavers
    #[structopt(short = "c", long = "count")]
    count: bool,

    /// Output file
    #[structopt(short = "o", long = "output")]
    output: Option<PathBuf>,
}

#[derive(StructOpt)]
struct OptS {
    /// Input file
    #[structopt(parse(from_os_str), short = "i", long = "input")]
    file: Option<PathBuf>,

    /// Count 1s especially for busy beavers
    #[structopt(short = "c", long = "count")]
    count: bool,

    /// Output file
    #[structopt(short = "o", long = "output")]
    output: Option<PathBuf>,
}

fn main() {
    let mut tm: turing_machine::TuringMachine;
    let cmd = Command::from_args();
    let buildin_s = include_str!("../examples/tmBB2.toml");
    match cmd {
        Command::Run(opt) => {
            if opt.file != None {
                let mut file = match File::open(opt.file.unwrap()) {
                    Ok(file) => file,
                    Err(_) => exit(exitcode::NOINPUT),
                };
                let mut s = String::new();
                match file.read_to_string(&mut s) {
                    Ok(_) => (),
                    Err(_) => exit(exitcode::DATAERR),
                };
                tm = toml::from_str(s.as_str()).unwrap();
            } else {
                tm = toml::from_str(buildin_s).unwrap();
            }
            println!("{}", tm);
            if opt.pr {
                tm.run_print();
            } else {
                tm.run();
                println!("{}", tm);
            }
            if opt.count {
                println!("Number of ones: {}", tm.count1s());
            }
            if opt.output != None {

            }
        }
        Command::Step(opt) => {
            if opt.file != None {
                let mut file = match File::open(opt.file.unwrap()) {
                    Ok(file) => file,
                    Err(_) => exit(exitcode::NOINPUT),
                };
                let mut s = String::new();
                match file.read_to_string(&mut s) {
                    Ok(_) => (),
                    Err(_) => exit(exitcode::DATAERR),
                };
                tm = toml::from_str(s.as_str()).unwrap();
            } else {
                tm = toml::from_str(buildin_s).unwrap();
            }
            println!("{}", tm);
            tm.step();
            println!("{}", tm);
            if opt.count {
                println!("Number of ones: {}", tm.count1s());
            }
            if opt.output != None {

            }
        }
    }
}
