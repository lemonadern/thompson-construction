// 連接の演算子として明示的に`.`を利用するトークン列を返す
fn regex_to_infix(tokens: Vec<char>) -> Vec<char> {
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

fn main() {
    println!("abcdef");
}

#[test]
fn case1() {
    let expected: Vec<char> = "a.a.a.b.c".chars().collect();
    assert_eq!(regex_to_infix("aaabc".chars().collect()), expected)
}

#[test]
fn case2() {
    let expected: Vec<char> = "a(b)c.c".chars().collect();
    assert_eq!(regex_to_infix("a(b)cc".chars().collect()), expected)
}

#[test]
fn case3() {
    let expected: Vec<char> = "a|b.a|c(c.c)".chars().collect();
    assert_eq!(regex_to_infix("a|ba|c(cc)".chars().collect()), expected)
}

#[test]
fn case4() {
    let expected: Vec<char> = "(a|a.b)a+a.b*".chars().collect();
    assert_eq!(regex_to_infix("(a|ab)a+ab*".chars().collect()), expected)
}
