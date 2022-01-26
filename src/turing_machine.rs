/*
 * turingmachine.rs - Functions to simulate a turing machine.
 *
 * (C) 2021 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Aüache-2.0
 *
 */

use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::{HashMap, VecDeque};
use std::fmt;

#[derive(Debug)]
pub struct TuringMachine {
    state: String,
    table: Table,
    //table: HashMap<String, HashMap<String, (String, Move, String)>>,
    tape: Tape,
    counter: BigUint,
}

impl TuringMachine {
    pub fn new(
        s: String,
        tb: HashMap<String, HashMap<String, (String, Move, String)>>,
        tp: Tape,
    ) -> TuringMachine {
        TuringMachine {
            state: s,
            table: Table::new(tb),
            tape: tp,
            counter: Zero::zero(),
        }
    }

    pub fn run(&mut self) {
        while self.state != "HALT" {
            self.step();
        }
    }

    pub fn step(&mut self) {
        if self.state != "HALT" {
            self.counter += 1_usize;
            let next = match self.table.table.get(&self.state) {
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

#[derive(Debug)]
pub struct Table {
    table: HashMap<String, HashMap<String, (String, Move, String)>>,
}

impl Table {
    fn new(ta: HashMap<String, HashMap<String, (String, Move, String)>>) -> Table {
        Table { table: ta }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Move {
    L,
    R,
    N,
}

#[derive(Debug)]
pub struct Tape {
    left: VecDeque<String>,
    center: String,
    right: VecDeque<String>,
}

impl Tape {
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

    pub fn new() -> Tape {
        Tape {
            left: VecDeque::from(vec!["0".to_owned(); 4]),
            center: "0".to_string(),
            right: VecDeque::from(vec!["0".to_owned(); 4]),
        }
    }
}

impl fmt::Display for TuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Steps: {}\nState: {}\nTable:\n",
            self.counter, self.state
        )?;
        write!(f, "{}", self.table)?;
        writeln!(f, "{}", self.tape)
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (count, (s, c)) in self.table.iter().enumerate() {
            if count != 0 {
                writeln!(f)?;
            }
            write!(f, "{}:", s)?;
            for (r, a) in c {
                write!(f, "  {}: |{} {} {}|", r, a.0, a.1, a.2)?;
            }
        }
        writeln!(f)
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
        for t in self.left.iter() {
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
