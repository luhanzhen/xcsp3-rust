/*=============================================================================
* parser for CSP instances represented in XCSP3 Format
*
* Copyright (c) 2023 xcsp.org (contact @ xcsp.org)
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in
* all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
* THE SOFTWARE.
*=============================================================================
*/

/*
 * <p>@project_name: xcsp3-rust
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/7/19 12:15
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */
pub mod xcsp3_utils {
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::expression_tree::xcsp3_utils::Operator::*;
    use std::str::FromStr;

    #[derive(Debug, Clone)]
    pub enum Operator {
        Add,
        Neg,
        Abs,
        Sub,
        Mul,
        Div,
        Mod,
        Sqr,
        Pow,
        Min,
        Max,
        Dist,
        Lt,
        Le,
        Ge,
        Gt,
        Ne,
        Eq,
        And,
        Not,
        Or,
        Xor,
        Iff,
        Imp,
        If,
    }

    impl Operator {
        pub fn string_is_operator(op: &str) -> Option<Self> {
            return match op {
                "add" => Some(Add),
                "neg" => Some(Neg),
                "abs" => Some(Abs),
                "sub" => Some(Sub),
                "mul" => Some(Mul),
                "div" => Some(Div),
                "mod" => Some(Mod),
                "sqr" => Some(Sqr),
                "pow" => Some(Pow),
                "min" => Some(Min),
                "max" => Some(Max),
                "dist" => Some(Dist),
                "lt" => Some(Lt),
                "le" => Some(Le),
                "ge" => Some(Ge),
                "gt" => Some(Gt),
                "ne" => Some(Ne),
                "eq" => Some(Eq),
                "and" => Some(And),
                "not" => Some(Not),
                "or" => Some(Or),
                "xor" => Some(Xor),
                "iff" => Some(Iff),
                "imp" => Some(Imp),
                "if" => Some(If),
                _ => None,
            };
        }
    }

    #[derive(Clone)]
    pub enum TreeNode {
        RightBracket,
        Constant(i32),
        Argument(i32),
        Variable(String),
        Operator(Operator, Vec<TreeNode>),
    }

    impl TreeNode {
        pub fn to_string(&self) -> String {
            match self {
                TreeNode::Constant(i) => i.to_string(),
                TreeNode::Variable(v) => v.to_string(),
                TreeNode::Argument(a) => format!("%{}", a),
                TreeNode::Operator(o, v) => {
                    let mut ret = format!("{:?}", o);
                    ret.push_str("(");
                    for (i, n) in v.iter().enumerate() {
                        ret.push_str(&n.to_string());
                        if i != v.len() - 1 {
                            ret.push_str(", ")
                        }
                    }
                    ret.push_str(")");
                    ret
                }
                // TreeNode::Root => "".to_string(),
                _ => "".to_string(),
            }
        }
    }

    #[derive(Clone)]
    pub struct ExpressionTree {
        root: TreeNode,
        expression: String,
    }

    impl ExpressionTree {
        pub fn to_string(&self) -> String {
            format!(
                " expression = {}, expression_str = {}, ",
                self.root.to_string(),
                self.expression,
            )
        }
        pub fn from_str(expression: &str) -> Result<Self, Xcsp3Error> {
            match ExpressionTree::parse(expression) {
                Ok(e) => Ok(ExpressionTree {
                    root: e,
                    expression: expression.to_string(),
                }),
                Err(e) => Err(e),
            }
        }

        fn operator(exp: &str, stack: &mut Vec<TreeNode>) -> Option<Xcsp3Error> {
            let expression: String = exp.chars().rev().collect();
            // expression = expression.replace("(", "").replace(")", "");
            match Operator::string_is_operator(&expression) {
                None => {
                    if expression.contains("%") {
                        match i32::from_str(&expression[1..]) {
                            Ok(n) => stack.push(TreeNode::Argument(n)),
                            Err(_) => {
                                return Some(Xcsp3Error::get_constraint_expression_error(
                                    "parse the expression error!!",
                                ));
                            }
                        }
                    } else {
                        match i32::from_str(&expression[..]) {
                            Ok(n) => stack.push(TreeNode::Constant(n)),
                            Err(_) => stack.push(TreeNode::Variable(expression)),
                        }
                    }
                }
                Some(ope) => {
                    let mut nodes = vec![];
                    loop {
                        let top = stack.pop();
                        match top {
                            None => {
                                return Some(Xcsp3Error::get_constraint_expression_error(
                                    "parse the expression error!!",
                                ));
                            }
                            Some(n) => match n {
                                TreeNode::RightBracket => {
                                    stack.push(TreeNode::Operator(ope, nodes));
                                    break;
                                }
                                _ => {
                                    nodes.push(n);
                                }
                            },
                        }
                    }
                }
            }

            None
        }

        fn parse(expression: &str) -> Result<TreeNode, Xcsp3Error> {
            let mut stack: Vec<TreeNode> = vec![];
            let exp: String = expression.chars().filter(|c| !c.is_whitespace()).collect();

            let rev_exp: String = exp.chars().rev().collect();
            // println!("{rev_exp}");
            let mut i = 0;
            let mut last = 0;
            while i < rev_exp.len() {
                if &rev_exp[i..i + 1] == ")" {
                    stack.push(TreeNode::RightBracket);
                    last = i;
                } else {
                    if &rev_exp[i..i + 1] == "," || &rev_exp[i..i + 1] == "(" {
                        match ExpressionTree::operator(&rev_exp[last + 1..i], &mut stack) {
                            Some(e) => {
                                return Err(e);
                            }
                            _ => {}
                        }
                        last = i;
                    } else if i == rev_exp.len() - 1 {
                        match ExpressionTree::operator(&rev_exp[last + 1..i + 1], &mut stack) {
                            Some(e) => {
                                return Err(e);
                            }
                            _ => {}
                        }
                    }
                }
                i += 1
            }

            Ok(stack.pop().unwrap())
        }
        pub fn iter(&self) -> ExpressionIter {
            ExpressionIter {}
        }
    }

    pub struct ExpressionIter {}

    impl Iterator for ExpressionIter {
        type Item = TreeNode;

        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }
}
