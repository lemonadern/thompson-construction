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

fn infix_to_postfix(input: &str) -> String {
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

fn main() {
    let inputs = vec!["a.b.c", "a.b|c", "a.b+.c", "a.(b.b)+.c"];
    let expected = vec!["ab.c.", "ab.c|", "ab+.c.", "abb.+.c."];

    for (i, e) in inputs.iter().zip(expected.iter()) {
        assert_eq!(infix_to_postfix(*i), *e);

        println!();
        println!("output  : {}", infix_to_postfix(*i));
        println!("expected: {}", *e);
    }
}
