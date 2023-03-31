use crate::stack::Stack;

pub enum State {
    Dangling,
    Literal { value: char, next: Box<State> },
    Alternation { left: Box<State>, right: Box<State> },
    Matching,
}

pub struct Fragment<'a> {
    start: State,
    out: Vec<&'a State>,
}

impl<'a> Fragment<'a> {
    pub fn new(start: State, out: Vec<&'a State>) -> Fragment {
        Fragment { start, out }
    }
}

pub fn postfix_to_nfa(tokens: Vec<char>) {
    let mut stack = Stack::new();

    for t in tokens {
        match t {
            c @ _ => {
                let next = State::Dangling;

                let s = State::Literal {
                    value: c,
                    next: Box::new(next),
                };
            }
        }
    }
}
