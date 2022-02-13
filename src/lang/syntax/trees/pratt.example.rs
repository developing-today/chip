fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S {
    let mut lhs = match lexer.next() {
        Token::Atom(it) => {
            print!("{} ", it);
            S::Atom(it)
        }
        Token::Op('(') => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        }
        Token::Op(op) => {
            let ((), r_bp) = prefix_binding_power(op);
            let rhs = expr_bp(lexer, r_bp);
            print!("{} ", op);
            S::Cons(op, vec![rhs])
        }
        t => panic!("bad token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        if let Some((l_bp, ())) = postfix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            lhs = if op == '[' {
                let rhs = expr_bp(lexer, 0);
                assert_eq!(lexer.next(), Token::Op(']'));
                S::Cons(op, vec![lhs, rhs])
            } else {
                S::Cons(op, vec![lhs])
            };
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            lhs = if op == '?' {
                let mhs = expr_bp(lexer, 0);
                assert_eq!(lexer.next(), Token::Op(':'));
                let rhs = expr_bp(lexer, r_bp);
                S::Cons(op, vec![lhs, mhs, rhs])
            } else {
                let rhs = expr_bp(lexer, r_bp);
                S::Cons(op, vec![lhs, rhs])
            };
            continue;
        }

        break;
    }

    lhs
}

fn prefix_binding_power(op: char) -> ((), u8) {
    match op {
        '+' | '-' => ((), 9),
        _ => panic!("bad op: {:?}", op),
    }
}

fn postfix_binding_power(op: char) -> Option<(u8, ())> {
    let res = match op {
        '!' => (11, ()),
        '[' => (11, ()),
        _ => return None,
    };
    Some(res)
}

fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    let res = match op {
        '=' => (2, 1),
        '?' => (4, 3),
        '+' | '-' => (5, 6),
        '*' | '/' => (7, 8),
        '.' => (14, 13),
        _ => return None,
    };
    Some(res)
}

#[test]
fn tests() {
    let s = parse("1");
    let r = parsen("1");
    assert_eq!(s.to_string(), "1");
    assert_eq!(r.to_string(), "1");

    let s = expr("1 + 2 * 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
    assert_eq!(r.to_string(), "*2 3+1");

    let s = expr("a + b * c * d + e");
    assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");
    assert_eq!(r.to_string(), "*b c*d+ae");

    let s = expr("f . g . h");
    assert_eq!(s.to_string(), "(. f (. g h))");
    assert_eq!(r.to_string(), ".ghf");

    let s = expr("1 + 2 + f . g . h * 3 * 4");
    assert_eq!(s.to_string(), "(+ (+ 1 2) (* (* (. f (. g h)) 3) 4))");
    assert_eq!(r.to_string(), "+12.ghf*34");

    let s = expr("--1 * 2");
    assert_eq!(s.to_string(), "(* (- (- 1)) 2)");
    assert_eq!(r.to_string(), "(* (- (- 1)) 2)");

    let s = expr("--f . g");
    assert_eq!(s.to_string(), "(- (- (. f g)))");
    assert_eq!(r.to_string(), "(- (- (. f g)))");

    let s = expr("-9!");
    assert_eq!(s.to_string(), "(- (! 9))");
    assert_eq!(r.to_string(), "(- (! 9))");

    let s = expr("f . g !");
    assert_eq!(s.to_string(), "(! (. f g))");
    assert_eq!(r.to_string(), "(! (. f g))");

    let s = expr("(((0)))");
    assert_eq!(s.to_string(), "0");
    assert_eq!(r.to_string(), "0");

    let s = expr("x[0][1]");
    assert_eq!(s.to_string(), "([ ([ x 0) 1)");
    assert_eq!(r.to_string(), "([ ([ x 0) 1)");

    let s = expr(
        "a ? b :
         c ? d
         : e",
    );
    assert_eq!(s.to_string(), "(? a b (? c d e))");

    let s = expr("a = 0 ? b : c = d");
    assert_eq!(s.to_string(), "(= a (= (? 0 b c) d))");
    assert_eq!(r.to_string(), "(= a (= (? 0 b c) d))");
}

fn main() {
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let s = expr(&line);
        println!("{}", s)
    }
}
