/*
 * turingmachine.rs - Functions to simulate a turing machine.
 *
 * (C) 2021 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

use indexmap::map::IndexMap;
use serde_derive::{Serialize, Deserialize};
//use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;

/// Implementation of a Turing machine
#[derive(Debug, Deserialize, Serialize)]
pub struct TuringMachine {
    steps: usize,
    state: String,
    table: IndexMap<String, IndexMap<String, (String, Move, String)>>,
    tape: Tape,
}

impl TuringMachine {
    /// Count the ones on the tape especially useful for the busy beavers game
    pub fn count1s(&mut self) -> u128 {
        self.tape.count1s()
    }

    /// Run the Turing machine until it halts (if it halts ;) ).
    pub fn run(&mut self) {
        while self.state != "HALT" {
            self.step();
        }
    }

    /// Run the Turing machine until it halts (if it halts). Print every step of that.
    pub fn run_print(&mut self) {
        while self.state != "HALT" {
            self.step();
            println!("{}", self);
        }
    }

    /// Do one step of the Turing machine.
    pub fn step(&mut self) {
        if self.state != "HALT" {
            self.steps += 1;
            let next = match self.table.get(&self.state) {
                Some(x) => match x.get(&self.tape.center) {
                    Some(x) => x,
                    None => panic!("Error1"),
                },
                None => panic!("Error2"),
            };
            self.tape.center = next.0.clone();
            self.tape.mov(next.1);
            self.state = next.2.to_string();
        }
    }
}

/// Implementation of the movement of the head of the tape.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Move {
    L,
    R,
    N,
}

/// Implementation of the tape of the Turing machine.
#[derive(Debug, Deserialize, Serialize)]
pub struct Tape {
    left: VecDeque<String>,
    center: String,
    right: VecDeque<String>,
}

impl Tape {
    fn count1s(&mut self) -> u128 {
        let mut steps = 0;
        for t in self.left.iter() {
            if t == "1" {
                steps += 1;
            }
        }
        if self.center == "1" {
            steps += 1;
        }
        for t in self.right.iter() {
            if t == "1" {
                steps += 1;
            }
        }
        steps
    }

    fn mov(&mut self, dir: Move) {
        if dir == Move::L {
            self.right.push_front(self.center.clone());
            self.center = match self.left.pop_front() {
                Some(x) => x,
                None => "0".to_string(),
            };
        } else if dir == Move::R {
            self.left.push_front(self.center.clone());
            self.center = match self.right.pop_front() {
                Some(x) => x,
                None => "0".to_string(),
            };
        } else if dir == Move::N {
        }
    }
}

impl fmt::Display for TuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Steps: {}\nState: {}\nTable:\n",
            self.steps, self.state
        )?;
        for (count, (s, c)) in self.table.iter().enumerate() {
            if count != 0 {
                writeln!(f)?;
            }
            write!(f, "{}:", s)?;
            for (r, a) in c {
                write!(f, "  {}: |{} {} {:4}|", r, a.0, a.1, a.2)?;
            }
        }
        writeln!(f)?;
        writeln!(f, "{}", self.tape)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::L => write!(f, "L"),
            Move::R => write!(f, "R"),
            Move::N => write!(f, "N"),
        }
    }
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|")?;
        for t in self.left.iter().rev() {
            write!(f, "|{}", t)?;
        }
        write!(f, "[{}]", self.center)?;
        for (count, t) in self.right.iter().enumerate() {
            if count != 0 {
                write!(f, "|")?;
            }
            write!(f, "{}", t)?;
        }
        write!(f, "||")
    }
}
