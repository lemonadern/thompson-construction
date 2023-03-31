mod regex_to_infix {
    use thompson_construction::expression_conversion::regex_to_infix;

    #[test]
    fn test() {
        let mapping = vec![
            ("aaabc", "a.a.a.b.c"),
            ("a(b)cc", "a(b)c.c"),
            ("a|ba|c(cc)", "a|b.a|c(c.c)"),
            ("(a|(ab))a+ab*", "(a|(a.b))a+a.b*"),
        ];

        for (i, e) in mapping {
            let i: Vec<char> = i.chars().collect();
            let e: Vec<char> = e.chars().collect();
            assert_eq!(regex_to_infix(&i), e);
        }
    }
}

mod infix_to_postfix {
    use thompson_construction::expression_conversion::infix_to_postfix;

    #[test]
    fn test() {
        let mapping = vec![
            ("a.b.c", "ab.c."),
            ("a.b|c", "ab.c|"),
            ("a.b+.c", "ab+.c."),
            ("a.(b.b)+.c", "abb.+.c."),
            ("a.(b)+.c", "ab+.c."),
            ("(a.b|c).(b|c)", "ab.c|bc|."),
        ];

        for (i, e) in mapping {
            let i: Vec<char> = i.chars().collect();
            let e: Vec<char> = e.chars().collect();
            assert_eq!(infix_to_postfix(&i), e);
        }
    }
}
