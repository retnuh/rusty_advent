#[macro_use]
extern crate nom;

use std::cell::Cell;
use std::collections::HashMap;
use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map_res,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
enum Input {
    Value(u16),
    Wire(String),
}

impl Input {
    fn value(&self, state: &HashMap<String, Gate>) -> u16 {
        match self {
            Input::Value(val) => *val,
            Input::Wire(name) => {
                // TODO should memoization happen here also?
                state.get(name.as_str()).unwrap().value(state)
            }
        }
    }
}

// struct Signal<'a> {
//     wire: String,
//     value: Option<u16>,
//     source: &'a Gate,
// }
//
// impl Signal<'_> {
//     fn new<'a>(wire: &str, source: &'a Gate) -> Signal<'a> {
//         Signal {
//             wire: wire.to_owned(),
//             value: None,
//             source,
//         }
//     }
// }

// #[derive(Debug)]
struct Gate {
    value: Cell<Option<u16>>,
    op: Box<dyn Fn(&HashMap<String, Gate>) -> u16>,
}

impl Gate {
    fn value(&self, state: &HashMap<String, Gate>) -> u16 {
        match self.value.get() {
            Some(val) => val,
            None => {
                let val = (*self.op)(state);
                self.value.set(Some(val));
                val
            }
        }
    }

    fn new(op: Box<dyn Fn(&HashMap<String, Gate>) -> u16>) -> Gate {
        Gate {
            value: Cell::new(None),
            op,
        }
    }
    fn simple(input: Input) -> Gate {
        Gate::new(Box::new(move |state| input.value(state)))
    }

    fn not(input: Input) -> Gate {
        Gate::new(Box::new(move |state| {
            let val = input.value(state);
            !val
        }))
    }

    fn and(left: Input, right: Input) -> Gate {
        Gate::new(Box::new(move |state| {
            let left = left.value(state);
            let right = right.value(state);
            left & right
        }))
    }

    fn or(left: Input, right: Input) -> Gate {
        Gate::new(Box::new(move |state| {
            let left = left.value(state);
            let right = right.value(state);
            left | right
        }))
    }

    fn lshift(left: Input, amt: u16) -> Gate {
        Gate::new(Box::new(move |state| {
            let left = left.value(state);
            left << amt
        }))
    }

    fn rshift(left: Input, amt: u16) -> Gate {
        Gate::new(Box::new(move |state| {
            let left = left.value(state);
            left >> amt
        }))
    }
}

fn u16_parser(input: &str) -> IResult<&str, u16> {
    map_res(digit1, |val: &str| val.parse::<u16>())(input)
}

named!(
    input_parser<&str, Input>,
    alt!(
        alpha1 => { |name: &str| Input::Wire(name.to_owned()) } |
        u16_parser => { |val: u16|  Input::Value(val) }
    )
);

#[test]
fn test_input_parser() {
    assert_eq!(
        input_parser("foo bar"),
        Ok((" bar", Input::Wire("foo".to_owned()))),
    );
    assert_eq!(
        input_parser("12345 bar"),
        Ok((" bar", Input::Value(12345u16))),
    )
}

named!(output_parser<&str, &str>, preceded!(tag(" -> "), alpha1));

#[test]
fn test_output_parser() {
    assert_eq!(output_parser(" -> foo"), Ok(("", "foo")),);
    assert_eq!(output_parser(" -> foo\n"), Ok(("\n", "foo")),);
}

named!(gate_parser<&str, Gate>,
    alt!(
        complete!(tuple!(input_parser, tag!(" AND "), input_parser)) => { |(l, _, r)| Gate::and(l,r) } |
        complete!(tuple!(input_parser, tag!(" OR "), input_parser)) => { |(l, _, r)| Gate::or(l,r) } |
        complete!(tuple!(input_parser, tag!(" LSHIFT "), u16_parser)) => { |(l, _, r)| Gate::lshift(l,r) } |
        complete!(tuple!(input_parser, tag!(" RSHIFT "), u16_parser)) => { |(l, _, r)| Gate::rshift(l,r) } |
        preceded!(tag!("NOT "), input_parser) => { |input: Input| Gate::not(input) } |
        input_parser => { |input: Input| Gate::simple(input) }
    )
);

#[test]
fn test_gate_parser() {
    let mut state: HashMap<String, Gate> = HashMap::new();
    state.insert("simple".to_owned(), gate_parser("12345").unwrap().1);
    state.insert("not".to_owned(), gate_parser("NOT simple").unwrap().1);
    state.insert("three".to_owned(), gate_parser("3").unwrap().1);
    state.insert("two".to_owned(), gate_parser("2").unwrap().1);
    state.insert("one".to_owned(), gate_parser("1").unwrap().1);
    state.insert("zero".to_owned(), gate_parser("0").unwrap().1);
    state.insert("and".to_owned(), gate_parser("one AND three").unwrap().1);
    state.insert("or".to_owned(), gate_parser("one OR two").unwrap().1);
    state.insert("rshift".to_owned(), gate_parser("12 RSHIFT 2").unwrap().1);
    state.insert(
        "lshift".to_owned(),
        gate_parser("three LSHIFT 2").unwrap().1,
    );

    assert_eq!(state.get("simple").unwrap().value(&state), 12345);
    assert_eq!(state.get("not").unwrap().value(&state), !12345);
    assert_eq!(state.get("and").unwrap().value(&state), 1);
    assert_eq!(state.get("or").unwrap().value(&state), 3);
    assert_eq!(state.get("rshift").unwrap().value(&state), 3);
    assert_eq!(state.get("lshift").unwrap().value(&state), 12);
}

named!(signal_parser<&str, (String, Gate)>,
    map!(tuple!(gate_parser, output_parser), |(g,n)| (n.to_owned(),g))
);

named!(circuit_parser<&str, HashMap<String, Gate>>,
    map!(
        separated_list1!(complete!(tag!("\n")), complete!(signal_parser)),
        |v| v.into_iter().collect::<HashMap<String, Gate>>()
    )
);

#[test]
fn test_circuit_parser() {
    let state = circuit_parser("1 -> foo\nfoo LSHIFT 3 -> bar\nbar -> baz")
        .unwrap()
        .1;
    assert_eq!(state.get("baz").unwrap().value(&state), 8);
}

fn main() {
    let text = fs::read_to_string("inputs/2015/day7.txt").unwrap();
    let part1_state = circuit_parser(&text).unwrap().1;
    let part1_a_val = part1_state.get("a").unwrap().value(&part1_state);
    println!("Part1 a's output: {}", part1_a_val);
    let mut part2_state = circuit_parser(&text).unwrap().1;
    let (b_name, b_gate) = signal_parser(&format!("{} -> b", part1_a_val)).unwrap().1;
    part2_state.insert(b_name, b_gate);
    let part2_a_val = part2_state.get("a").unwrap().value(&part2_state);
    println!("Part2 a's output: {}", part2_a_val);
}
