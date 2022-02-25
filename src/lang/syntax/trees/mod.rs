mod leafs;

use chiploxide::lang::tokens::{Token, Tokens, Unknown};

use self::leafs::{Leaf, Leafs};
#[derive(Debug, Clone, PartialEq)]
pub struct Tree(Leaf, Branch);
#[derive(Debug, Clone, PartialEq)]
pub struct Trees(pub(crate) Vec<Tree>);

// 1 + 2
// Tree(+, Branch[1, 2])
// (1 + 2) * (3 + 4)
/*
            9(
1
        2+
2
             ) <--- run 1+2 here (a)
          5*
            9(
3
        2+
4
             ) <--- run 3+4 here (b)
             <--- run a*b here
          */
//
//
// Tree(*, Branch[Tree(+, Branch[1, 2]), Tree(+, Branch[3, 4])])

#[derive(Debug, Clone, PartialEq)]
pub enum Branch {
    Leaf,
    Branch(Leaf, Trees),
}
/// A branch is_leaf if the Branch is a leaf
impl Branch {
    pub fn is_leaf(&self) -> bool {
        match self {
            Branch::Leaf => true,
            _ => false,
        }
    }
    pub fn is_pair(&self) -> bool {
        match self {
            Branch::Branch(_, vec) => vec.0.len() == 0, // a branch "is" in relation to its tree
            _ => false,
        }
    }

    pub(crate) fn empty() -> Branch {
        Branch::Leaf
    }
}
impl Tree {
    pub(crate) fn new(tokens: Tokens) -> Tree {
        tokens.into()
    }
    pub(crate) fn empty() -> Tree {
        Tree(Leaf::empty(), Branch::Leaf)
    }
}
impl Trees {
    pub(crate) fn empty() -> Trees {
        Tree::empty().into()
    }
}
impl From<Tree> for Trees {
    fn from(tree: Tree) -> Trees {
        Trees(vec![tree])
    }
}
impl From<Leafs> for TreeParse {
    fn from(leafs: Leafs) -> TreeParse {
        TreeParse::new(leafs)
    }
}
impl From<Tokens> for Tree {
    fn from(tokens: Tokens) -> Tree {
        match TreeParse::from(Leafs::from(tokens)).parse() {
            Wood::Tree(tree) => tree,
            Wood::Branch(branch) => branch.into(),
        }
    }
}
impl From<Leaf> for Tree {
    fn from(leaf: Leaf) -> Tree {
        Tree(leaf, Branch::Leaf)
    }
}
impl From<Leaf> for Branch {
    fn from(leaf: Leaf) -> Branch {
        Branch::Branch(leaf, Trees::empty())
    }
}
impl From<Branch> for Tree {
    fn from(branch: Branch) -> Tree {
        Tree(Leaf::empty(), branch)
    }
}
impl From<Tree> for Branch {
    fn from(tree: Tree) -> Branch {
        Branch::Branch(Leaf::empty(), tree.into())
    }
}
/// Returns the root node of the tree expression.
/// Leaf structs implement 3 methods
/// for expressing binding power:
/// 1. prefix_binding_power
/// 2. infix_binding_power
/// 3. postfix_binding_power
/// this was supposed to be a recursive lr pratt parser
// /// k = 1 (max lookahead depth)
// pub(crate) fn express_stickiness(
//     leaves: Leafs,
//     minimum_stickiness: u8,
//     empty_leaf: Branch,
// ) -> Tree {
//     let mut stack = Trees(Vec::new());
//     let current_stickiness = minimum_stickiness;
//     for leaf in leaves.0 {
//         if leaf.clone().prefix_stickiness().unwrap_or(0) > current_stickiness {
//             stack.0.push(Tree(leaf.clone(), empty_leaf.clone()).clone());
//         } else {
//             let mut new_stack = Trees(Vec::new());
//             while let Some(top) = stack.0.pop() {
//                 if top.1.is_leaf() {
//                     new_stack.0.push(top);
//                 } else {
//                     let mut children = Trees(Vec::new());
//                     children.0.push(top.clone());
//                     while let Some(top) = stack.0.pop() {
//                         if top.1.is_leaf() {
//                             children.0.push(top);
//                         } else {
//                             if top.0.clone().infix_binding_power().0
//                                 > top.0.clone().postfix_binding_power().unwrap_or(0)
//                             {
//                                 stack.0.push(top);
//                             } else {
//                                 children.0.push(top);
//                             }
//                         }
//                     }
//                     new_stack.0.push(Tree(
//                         leaf.clone(),
//                         Branch::Branch(
//                             top.0.clone(),
//                             Trees(children.0.into_iter().rev().collect()),
//                         )
//                         .clone(),
//                     ));
//                 }
//             }
//             stack = new_stack;
//         }
//     }

//     stack.0.pop().unwrap_or(Tree::empty()).clone()
// }
#[derive(Debug, Clone, PartialEq)]
struct TreeParse(usize, Leafs, Tree);
impl TreeParse {
    pub(crate) fn new(leafs: Leafs) -> TreeParse {
        TreeParse(0, leafs, Tree::empty())
    }
    //   # (Left-to-right, leftmost derivation)
    //   # (Left-to-right, Rightmost derivation in reverse)

    //   fn AtToken(&self, token_type):
    //     """Test if we are looking at a token."""
    //     # ...

    fn next(&mut self) {
        self.0 += 1;
    }
    fn peek(&self) -> Option<&Leaf> {
        self.1 .0.get(self.0)
    }
    fn peek_next(&self) -> Option<&Leaf> {
        self.1 .0.get(self.0 + 1)
    }
    //   fn Eat(&self, val):
    //     """Assert the value of the current token, then move to the next token."""
    //     # ...
    fn eat(&mut self, val: &Leaf) -> bool {
        if let Some(leaf) = self.peek() {
            if leaf == val {
                self.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn eat_then_parse_until(mut self, leaf: &Leaf, rbp: u8) -> Branch {
        Branch::Branch(
            leaf.clone(),
            Tree::from(|| -> Branch {
                if self.eat(leaf) {
                    return self.parse_until(rbp);
                } else {
                    return Branch::empty().into();
                }
            }())
            .into(),
        )
    }
    //   fn ParseUntil(&self, rbp):
    //     """
    //     Parse to the right, eating tokens until we encounter a token with binding
    //     power LESS THAN OR EQUAL TO rbp.
    //     """
    //     # ...

    //   fn Parse(&self):
    //     """Initial entry point."""
    //     return self.ParseUntil(0)
    // Purpose of Wood reserved for later
    fn parse(self) -> Wood {
        Wood::Branch(self.parse_until(0))
    }
    // Parses Tokens Into Nested Tree
    // One can also parse into a single flat tree of trees
    // Write both, from/into the ot_nexther. Can run either directly.
    fn parse_until(self, minimum_stickiness: u8) -> Branch {
        match self.clone().peek() {
            Some(leaf) => {
                match leaf.clone().prefix_stickiness() {
                    Some(prefix_stickiness) => {
                        return self.eat_then_parse_until(leaf, prefix_stickiness);
                        // if prefix_stickiness > minimum_stickiness {
                        //     || -> Branch {
                        //         if let Some(grouping) = leaf.clone().grouping() {
                        //             while !self.eat(&grouping.into()) {
                        //                 return self.eat_then_parse_until(leaf, prefix_stickiness);
                        //             }
                        //         }
                        //     }();
                        // }
                    }
                    _ => (),
                }
                loop {
                    let stickiness = leaf.clone().stickiness();
                    if stickiness.0 .1 > minimum_stickiness {
                        return self.eat_then_parse_until(leaf, stickiness.0 .1);
                    } else {
                        return Branch::from(leaf.clone().clone());
                    }
                }
            }
            None => Branch::empty(),
        }
    }
}

// Why is this made, some sort of generic match instead?
enum Wood {
    Tree(Tree),
    Branch(Branch),
}
// Why not just call the variant directly?
impl Wood {
    fn branch(branch: Branch) -> Wood {
        Wood::Branch(branch)
    }
    fn tree(tree: Tree) -> Wood {
        Wood::Tree(tree)
    }
}
