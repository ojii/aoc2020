use crate::maybe_from::MaybeFrom;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Token {
    Num(u64),
    Add,
    Mul,
    OpenParen,
    ClosingParen,
}

impl Token {
    fn get_num(&self) -> Option<u64> {
        match self {
            Token::Num(num) => Some(*num),
            _ => None,
        }
    }
}

impl MaybeFrom<&str> for Token {
    fn maybe_from(value: &str) -> Option<Self> {
        use Token::*;
        match value {
            "+" => Some(Add),
            "*" => Some(Mul),
            "(" => Some(OpenParen),
            ")" => Some(ClosingParen),
            num => num.parse::<u64>().ok().map(|num| Num(num)),
        }
    }
}

fn tokenize(expr: &str) -> Vec<Token> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split(" ")
        .flat_map(Token::maybe_from)
        .collect()
}

#[derive(Debug, Clone)]
enum Value {
    Const(u64),
    Expr(Vec<Expr>),
}

impl Value {
    fn resolve(&self) -> u64 {
        match self {
            Value::Const(num) => *num,
            Value::Expr(exprs) => evaluate(exprs),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn evaluate(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Value(Value),
    Operator(Operator),
}

impl Expr {
    fn value(&self) -> Option<Value> {
        match self {
            Expr::Value(value) => Some(value.clone()),
            _ => None,
        }
    }

    fn op(&self) -> Option<Operator> {
        match self {
            Expr::Operator(op) => Some(*op),
            _ => None,
        }
    }
}

fn deparen(tokens: Vec<Token>) -> Vec<Expr> {
    use Token::*;
    let mut result = Vec::new();
    let mut skip = 0;
    for (index, token) in tokens.iter().enumerate() {
        if skip > 0 {
            skip -= 1;
        } else {
            result.push(match token {
                Num(num) => Expr::Value(Value::Const(*num)),
                Add => Expr::Operator(Operator::Add),
                Mul => Expr::Operator(Operator::Mul),
                OpenParen => {
                    let mut inner = Vec::new();
                    let mut count = 0;
                    let mut level = 1;
                    for token in tokens.iter().skip(index + 1) {
                        count += 1;
                        match token {
                            OpenParen => {
                                level += 1;
                            }
                            ClosingParen => {
                                level -= 1;
                                if level == 0 {
                                    break;
                                }
                            }
                            _ => (),
                        }
                        inner.push(token.clone());
                    }
                    skip = count;
                    Expr::Value(Value::Expr(deparen(inner)))
                }
                ClosingParen => panic!("unexpected closing paren"),
            })
        }
    }
    result
}

fn evaluate_triple(triple: [&Expr; 3]) -> u64 {
    triple[1].op().expect("middle expr not an op").evaluate(
        triple[0].value().expect("lhs not value").resolve(),
        triple[2].value().expect("rhs not value").resolve(),
    )
}

fn evaluate(exprs: &[Expr]) -> u64 {
    match exprs.len() {
        1 => exprs[0].value().expect("single expr not a value").resolve(),
        3 => evaluate_triple([&exprs[0], &exprs[1], &exprs[2]]),
        n if n > 3 => {
            let value = evaluate_triple([&exprs[0], &exprs[1], &exprs[2]]);
            evaluate(
                &vec![Expr::Value(Value::Const(value))]
                    .iter()
                    .chain(exprs.iter().skip(3))
                    .cloned()
                    .collect_vec(),
            )
        }
        _ => panic!("Unexpected exprs len: {:?}", exprs),
    }
}

fn math(expr: &str) -> u64 {
    let tokens = tokenize(expr);
    let exprs = deparen(tokens);
    evaluate(&exprs)
}

pub fn run() {
    println!(
        "{}",
        include_str!("data/18/1")
            .lines()
            .map(|line| math(line))
            .sum::<u64>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(case = {
        ("1 + 2 * 3 + 4 * 5 + 6", 71),
        ("1 + (2 * 3) + (4 * (5 + 6))", 51),
        ("2 * 3 + (4 * 5)", 26),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632)
    })]
    #[test]
    fn test_math(case: (&str, u64)) {
        assert_eq!(math(case.0), case.1)
    }
}
