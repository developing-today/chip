// mod leaves;

// use crate::lang::tokens::Token;

// use self::leaves::Leaf;
// #[derive(Debug)]
// pub struct Tree(Leaf, Branch);

// #[derive(Debug)]
// pub enum Branch {
//     Leaf,
//     Branch(Leaf, Vec<Tree>),
// }
// impl Tree {
//     // pub fn new(tokens: &Vec<Token>) -> Tree {
//     //     Tree(
//     //         Leaf::new(tokens[0]),
//     //         Tree(Leaf::new(tokens[1]), Tree(tokens[2..])),
//     //     )
//     // } // lets keep going

//     pub(crate) fn new(tokens: Vec<Token>) -> Tree {
//         Tree(
//             Leaf::new(tokens[0]),
//             Branch::Branch(
//                 Leaf::new(tokens[1]),
//                 tokens[2..]
//                     .into_iter()
//                     .map(|token| Tree(Leaf::new(*token), Branch::Leaf))
//                     .collect(),
//             ),
//         )
//     }

//     // fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S {
//     //     let mut lhs = match lexer.next() {
//     //         Token::Atom(it) => {
//     //             print!("{} ", it);
//     //             S::Atom(it)
//     //         }
//     //         Token::Op('(') => {
//     //             let lhs = expr_bp(lexer, 0);
//     //             assert_eq!(lexer.next(), Token::Op(')'));
//     //             lhs
//     //         }
//     //         Token::Op(op) => {
//     //             let ((), r_bp) = prefix_binding_power(op);
//     //             let rhs = expr_bp(lexer, r_bp);
//     //             print!("{} ", op);
//     //             S::Cons(op, vec![rhs])
//     //         }
//     //         t => panic!("bad token: {:?}", t),
//     //     };

//     //     loop {
//     //         let op = match lexer.peek() {
//     //             Token::Eof => break,
//     //             Token::Op(op) => op,
//     //             t => panic!("bad token: {:?}", t),
//     //         };

//     //         if let Some((l_bp, ())) = postfix_binding_power(op) {
//     //             if l_bp < min_bp {
//     //                 break;
//     //             }
//     //             lexer.next();

//     //             lhs = if op == '[' {
//     //                 let rhs = expr_bp(lexer, 0);
//     //                 assert_eq!(lexer.next(), Token::Op(']'));
//     //                 S::Cons(op, vec![lhs, rhs])
//     //             } else {
//     //                 S::Cons(op, vec![lhs])
//     //             };
//     //             continue;
//     //         }

//     //         if let Some((l_bp, r_bp)) = infix_binding_power(op) {
//     //             if l_bp < min_bp {
//     //                 break;
//     //             }
//     //             lexer.next();

//     //             lhs = if op == '?' {
//     //                 let mhs = expr_bp(lexer, 0);
//     //                 assert_eq!(lexer.next(), Token::Op(':'));
//     //                 let rhs = expr_bp(lexer, r_bp);
//     //                 S::Cons(op, vec![lhs, mhs, rhs])
//     //             } else {
//     //                 let rhs = expr_bp(lexer, r_bp);
//     //                 S::Cons(op, vec![lhs, rhs])
//     //             };
//     //             continue;
//     //         }

//     //         break;
//     //     }

//     //     lhs
//     // }
// }
