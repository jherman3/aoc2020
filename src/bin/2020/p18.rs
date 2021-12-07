use aoc2020::each_line;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Token {
    Num(u64),
    LParen,
    RParen,
    Add,
    Mul,
}

fn closing_paren(input: &[Token]) -> usize {
    // dbg!(input);
    if input[0] != Token::LParen {
        panic!("not start paren");
    }
    let mut ctr = 1;
    for (i, c) in input.iter().enumerate().skip(1) {
        match c {
            Token::LParen => ctr += 1,
            Token::RParen => ctr -= 1,
            _ => {}
        }
        if ctr == 0 {
            return i;
        }
    }
    panic!("Unbalanced");
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut toks = Vec::new();
    let chars = input.chars().collect::<Vec<_>>();
    let mut i = 0;
    while i < input.len() {
        if chars[i].is_numeric() {
            let start = i;
            while i < input.len() && chars[i].is_numeric() {
                i += 1;
            }
            toks.push(Token::Num(
                chars[start..i].iter().collect::<String>().parse().unwrap(),
            ))
        } else {
            match chars[i] {
                '(' => {
                    toks.push(Token::LParen);
                }
                ')' => {
                    toks.push(Token::RParen);
                }
                '+' => {
                    toks.push(Token::Add);
                }
                '*' => {
                    toks.push(Token::Mul);
                }
                ' ' => {}
                _ => panic!("invalid"),
            }
            i += 1;
        }
    }
    toks
}

fn eval(toks: &[Token]) -> u64 {
    let mut i = 0;
    let mut cur_result = 0;
    let mut cur_op = Some(Token::Add);
    while i < toks.len() {
        if toks[i] == Token::LParen {
            let start = i + 1;
            i += closing_paren(&toks[i..]);
            match cur_op {
                Some(Token::Add) => cur_result += eval(&toks[start..i]),
                Some(Token::Mul) => cur_result *= eval(&toks[start..i]),
                _ => panic!("Invalid op"),
            };
            cur_op = None;
        } else if let Token::Num(x) = toks[i] {
            match cur_op {
                Some(Token::Add) => cur_result += x,
                Some(Token::Mul) => cur_result *= x,
                _ => panic!("Invalid op"),
            };
            cur_op = None;
        } else if let Token::Add = toks[i] {
            if cur_op.is_some() {
                panic!("double op");
            }
            cur_op = Some(Token::Add);
        } else if let Token::Mul = toks[i] {
            if cur_op.is_some() {
                panic!("double op");
            }
            cur_op = Some(Token::Mul);
        } else {
            panic!(format!("invalid token {:?}", toks[i]));
        }
        i += 1;
    }
    cur_result
}

fn parenthesize(toks: &[Token]) -> Vec<Token> {
    let mut res = Vec::new();
    let mut i = 0;
    let mut extra_parens = 0;
    while i < toks.len() {
        match toks[i] {
            Token::LParen => {
                let start = i + 1;
                i += closing_paren(&toks[i..]) + 1;
                res.push(Token::LParen);
                res.extend(parenthesize(&toks[start..i]).into_iter());
                // rparen is added by the real rparen
            }
            Token::Mul => {
                res.insert(0, Token::LParen);
                res.push(Token::RParen);
                res.push(Token::Mul);
                res.push(Token::LParen);
                extra_parens += 1;
                i += 1;
            },
            x => {
                res.push(x);
                i += 1;
            }
        }
    }
    for _ in 0..extra_parens {
        res.push(Token::RParen);
    }
    res
}

fn main() {
    let mut sum = 0;
    let mut sum2 = 0;
    for l in each_line("inputs/p18.txt") {
        let toks = tokenize(&l);
        sum += eval(&toks);
        sum2 += eval(&parenthesize(&toks));
    }
    dbg!(sum);
    dbg!(sum2);
}
