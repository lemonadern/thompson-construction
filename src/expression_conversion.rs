use crate::stack::Stack;

pub fn infix_to_postfix(input: &Vec<char>) -> Vec<char> {
    let mut stack: Stack<char> = Stack::new();
    let mut output: Vec<char> = Vec::new();

    for c in input {
        match c {
            '(' => stack.push('('),
            ')' => {
                while let Some(c) = stack.peek() {
                    if c == &'(' {
                        break;
                    }

                    output.push(stack.pop().unwrap())
                }
                stack.pop();
            }
            current @ _ => {
                while let Some(peeked) = stack.peek() {
                    if precedence(*peeked) >= precedence(*current) {
                        output.push(stack.pop().unwrap())
                    } else {
                        break;
                    }
                }
                stack.push(*c);
            }
        }
    }

    while let Some(c) = stack.pop() {
        output.push(c);
    }

    output
}

fn precedence(c: char) -> u8 {
    match c {
        '(' => 1,
        '|' => 2,
        '.' => 3,
        '?' => 4,
        '*' => 4,
        '+' => 4,
        _ => 5,
    }
}

// トークン列を受け取り、連接の演算子として明示的に`.`を利用するトークン列を返す
//      e.g. abc|aa => a.b.c|a.a
pub fn regex_to_infix(tokens: &Vec<char>) -> Vec<char> {
    let mut v = Vec::new();

    let mut literal_is_detected = false;
    for t in tokens.iter() {
        match t {
            c @ ('(' | ')' | '|' | '?' | '+' | '*') => {
                v.push(*c);
                literal_is_detected = false;
            }
            c @ _ => {
                if literal_is_detected {
                    v.push('.');
                }
                v.push(*c);
                literal_is_detected = true;
            }
        }
    }
    v
}
