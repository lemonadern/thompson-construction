use thompson_construction::stack::Stack;

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

pub fn infix_to_postfix(input: &str) -> String {
    let mut stack: Stack<char> = Stack::new();
    let mut output: Vec<char> = Vec::new();

    for c in input.chars() {
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
                    if precedence(*peeked) >= precedence(current) {
                        output.push(stack.pop().unwrap())
                    } else {
                        break;
                    }
                }
                stack.push(c);
            }
        }
    }

    while let Some(c) = stack.pop() {
        output.push(c);
    }

    output.iter().map(|x| x.to_string()).collect::<String>()
}

// トークン列を受け取り、連接の演算子として明示的に`.`を利用するトークン列を返す
//      e.g. abc|aa => a.b.c|a.a
pub fn regex_to_infix(tokens: Vec<char>) -> Vec<char> {
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

mod tests {
    use super::*;

    #[test]
    fn test() {
        let inputs = vec![
            "a.b.c",
            "a.b|c",
            "a.b+.c",
            "a.(b.b)+.c",
            "a.(b)+.c",
            "(a.b|c).(b|c)",
        ];
        let expected = vec![
            "ab.c.",
            "ab.c|",
            "ab+.c.",
            "abb.+.c.",
            "ab+.c.",
            "ab.c|bc|.",
        ];

        for (i, e) in inputs.iter().zip(expected.iter()) {
            assert_eq!(infix_to_postfix(*i), *e);

            println!();
            println!("output  : {}", infix_to_postfix(*i));
            println!("expected: {}", *e);
        }
    }
}
