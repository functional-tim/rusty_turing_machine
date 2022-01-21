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

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Move {
    L,
    R,
    S,
}

#[derive(Debug)]
pub struct TuringMachine {
    state: String,
    table: HashMap<String, HashMap<String, (String, Move, String)>>,
    tape: Tape,
    counter: BigUint,
}

impl TuringMachine {
    pub fn new(s: String, tb: HashMap<String, HashMap<String, (String, Move, String)>>, tp: Tape) -> TuringMachine {
        TuringMachine {
            state: s,
            table: tb,
            tape: tp,
            counter: Zero::zero(),
        }
    }

    pub fn step(&mut self) {
        if self.state != "HALT" {
            let next = match self.table.get(&self.state) {
                Some(x) => match x.get(&self.tape.center) {
                    Some(x) => x,
                    None => panic!("Error1"),
                }
                None => panic!("Error2"),
            };
            self.tape.center = next.0.clone();
            self.tape.mov(next.1);
            self.state = next.2.to_string();
        }
    }

    pub fn run(&mut self) {
        while self.state != "HALT" {
            self.step();
        }
    }
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
        }
    }

    pub fn new() -> Tape {
        Tape {
            left: VecDeque::new(),
            center: "0".to_string(),
            right: VecDeque::new(),
        }
    }
}
