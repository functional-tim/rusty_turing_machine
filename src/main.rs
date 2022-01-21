/*
 * main.rs - Console program to simulate a turing machine.
 *
 * (C) 2021 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

use std::collections::HashMap;

mod turing_machine;

fn main() {
    let mut tm: turing_machine::TuringMachine = turing_machine::TuringMachine::new(
        String::from("A"),
        HashMap::from([
            (
                String::from("A"),
                HashMap::from([
                    (
                        String::from("0"),
                        (
                            String::from("1"),
                            turing_machine::Move::R,
                            String::from("B")
                        )
                    ),
                    (
                        String::from("1"),
                        (
                            String::from("1"),
                            turing_machine::Move::R,
                            String::from("HALT")
                        )
                    )
                ])
            ),
            (
                String::from("B"),
                HashMap::from([
                    (
                        String::from("0"),
                        (
                            String::from("1"),
                            turing_machine::Move::L,
                            String::from("B")
                        )
                    ),
                    (
                        String::from("1"),
                        (
                            String::from("0"),
                            turing_machine::Move::R,
                            String::from("C")
                        )
                    )
                ])
            ),
            (
                String::from("C"),
                HashMap::from([
                    (
                        String::from("0"),
                        (
                            String::from("1"),
                            turing_machine::Move::L,
                            String::from("C")
                        )
                    ),
                    (
                        String::from("1"),
                        (
                            String::from("1"),
                            turing_machine::Move::L,
                            String::from("A")
                        )
                    )
                ])

            ),
        ]),
        turing_machine::Tape::new()
    );
    println!("{:?}", tm);
    tm.run();
    println!("{:?}", tm);
}
