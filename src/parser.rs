/// The AST node for expressions.
#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(String),
    Add(Box<Expr>, Box<Expr>),
}

peg::parser!(pub grammar parser() for str {
    pub rule statements() -> Vec<Expr>
        = s:(statement()*) { s }

    rule statement() -> Expr
        = _ e:expression() { e }

    rule expression() -> Expr
        = binary_op()

    rule binary_op() -> Expr = precedence!{
        a:@ _ "+" _ b:(@) { Expr::Add(Box::new(a), Box::new(b)) }
        --
        l:literal() { l }
    }

    rule literal() -> Expr
        = n:$(['0'..='9']+) { Expr::Literal(n.to_owned()) }

    rule _() = quiet!{[' ' | '\t']*}
});

#[cfg(test)]
mod test {
    use super::*;
    use peg::error::*;

    #[test]
    fn test_parse_int() {
        let s = "0";
        let ast = parser::statements(&s).unwrap();
        assert_eq!(ast[0], Expr::Literal("0".to_string()));

        let s = "10";
        let ast = parser::statements(&s).unwrap();
        assert_eq!(ast[0], Expr::Literal("10".to_string()));

        let s = "999999";
        let ast = parser::statements(&s).unwrap();
        assert_eq!(ast[0], Expr::Literal("999999".to_string()));
    }

    #[test]
    fn test_parse_plus() {
        let s = "1 + 2";
        let ast = parser::statements(&s).unwrap();
        assert_eq!(
            ast[0],
            Expr::Add(
                Box::new(Expr::Literal("1".to_string())),
                Box::new(Expr::Literal("2".to_string())),
            )
        );

        let s = "9999 + 0";
        let ast = parser::statements(&s).unwrap();
        assert_eq!(
            ast[0],
            Expr::Add(
                Box::new(Expr::Literal("9999".to_string())),
                Box::new(Expr::Literal("0".to_string())),
            )
        );

        let s = "99 +";
        let ast = parser::statements(&s);
        assert!(matches!(ast, Err(ParseError { .. })));

        let s = "+1000";
        let ast = parser::statements(&s);
        assert!(matches!(ast, Err(ParseError { .. })));
    }
}
