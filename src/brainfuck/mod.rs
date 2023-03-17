use std::env;
use std::io::{self, Stdout, StdoutLock, Write};

enum Op {
    Dec,
    Inc,
    Prev,
    Next,
    Print,
    Loop(Box<[Op]>),
}

struct Tape {
    pos: usize,
    tape: Vec<i32>,
}

impl Tape {
    fn new() -> Self {
        Self {
            pos: 0,
            tape: vec![0],
        }
    }

    fn current_cell(&self) -> i32 {
        // SAFETY: `self.pos` is already checked in `self.next()`.
        unsafe { *self.tape.get_unchecked(self.pos) }
    }

    fn inc(&mut self, x: i32) {
        // SAFETY: `self.pos` is already checked in `self.next()`.
        unsafe { *self.tape.get_unchecked_mut(self.pos) += x };
    }

    fn prev(&mut self) {
        self.pos -= 1;
    }

    fn next(&mut self) {
        self.pos += 1;
        if self.pos >= self.tape.len() {
            self.tape.resize(self.pos * 2, 0);
        }
    }
}

struct Printer<'a> {
    output: StdoutLock<'a>,
    sum1: i32,
    sum2: i32,
    quiet: bool,
}

impl Printer<'_> {
    fn new(output: &Stdout, quiet: bool) -> Self {
        Self {
            output: output.lock(),
            sum1: 0,
            sum2: 0,
            quiet,
        }
    }

    fn print(&mut self, n: i32) {
        if self.quiet {
            self.sum1 = (self.sum1 + n) % 255;
            self.sum2 = (self.sum2 + self.sum1) % 255;
        } else {
            self.output.write_all(&[n as u8]).unwrap();
            self.output.flush().unwrap();
        }
    }

    const fn checksum(&self) -> i32 {
        (self.sum2 << 8) | self.sum1
    }
}

fn run(program: &[Op], tape: &mut Tape, p: &mut Printer) {
    for op in program {
        match op {
            Op::Dec => tape.inc(-1),
            Op::Inc => tape.inc(1),
            Op::Prev => tape.prev(),
            Op::Next => tape.next(),
            Op::Print => p.print(tape.current_cell()),
            Op::Loop(program) => {
                while tape.current_cell() > 0 {
                    run(program, tape, p);
                }
            }
        }
    }
}

fn parse(it: &mut impl Iterator<Item = u8>) -> Box<[Op]> {
    let mut buf = vec![];
    while let Some(c) = it.next() {
        buf.push(match c {
            b'-' => Op::Dec,
            b'+' => Op::Inc,
            b'<' => Op::Prev,
            b'>' => Op::Next,
            b'.' => Op::Print,
            b'[' => Op::Loop(parse(it)),
            b']' => break,
            _ => continue,
        });
    }
    buf.into_boxed_slice()
}

struct Program {
    ops: Box<[Op]>,
}

impl Program {
    fn new(code: &[u8]) -> Self {
        Self {
            ops: parse(&mut code.iter().copied()),
        }
    }

    fn run(&self, p: &mut Printer) {
        let mut tape = Tape::new();
        run(&self.ops, &mut tape, p);
    }
}

fn verify() {
    const SOURCE: &[u8] = b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>\
                            ---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    let output = io::stdout();
    let left = {
        let mut p = Printer::new(&output, true);
        Program::new(SOURCE).run(&mut p);
        p.checksum()
    };

    let right = {
        let mut p = Printer::new(&output, true);
        for &c in b"Hello World!\n" {
            p.print(c as i32);
        }
        p.checksum()
    };

    assert_eq!(left, right);
}

pub fn execute() -> Option<i32> {
    verify();

    let source = include_bytes!("source.b");
    let output = io::stdout();
    let mut p = Printer::new(&output, env::var("QUIET").is_ok());

    Program::new(source).run(&mut p);

    if p.quiet {
        return Some(p.checksum());
    }

    None
}
