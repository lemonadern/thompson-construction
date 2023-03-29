use thompson_construction::expression_conversion::{infix_to_postfix, regex_to_infix};

fn main() {
    let tokens: Vec<char> = "abc|(bc)+".chars().collect();

    let tokens = regex_to_infix(&tokens);
    let tokens = infix_to_postfix(&tokens);

    // TODO: toNFA

    // TODO: execute NFA

    println!("{:?}", tokens);
}
