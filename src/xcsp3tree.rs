/**
 * <p>@project_name: XCSP3-Rust
 * <p/>
 * <p>@author: luhanzhen
 * <p/>
 * <p>@date: 2023/7/2
 * <p/>
 * <p>@time: 16:47
 * <p/>
 * <p>@this_file_name:xcsp3tree
 * <p/>
 */
#[allow(dead_code)]
pub mod xcsp3_core {
    use crate::xcsp3treenode::xcsp3_core::*;
    use std::collections::HashMap;

    struct Tree<'a> {
        root: Option<&'a dyn NodeTraits>,
        expr: String,
        list_of_variables: Vec<String>,
    }

    impl<'a> Tree<'a> {
        pub fn from_string(e: String) -> Tree<'a> {
            Tree {
                root: from_string_to_tree(&e),
                expr: e,
                list_of_variables: vec![],
            }
        }

        pub fn from_node(node: &dyn NodeTraits) -> Tree {
            Tree {
                root: Some(node),
                expr: String::default(),
                list_of_variables: vec![],
            }
        }
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            self.root.unwrap().evaluate(tuple)
        }
        pub fn arity(&self) -> usize {
            self.list_of_variables.len()
        }

        pub fn to_string(&self) -> String {
            self.root.unwrap().to_string()
        }

        pub fn canonize(&self) -> Option<&'a dyn NodeTraits> {
            self.root
        }
    }

    fn min<'a>(v1: &'a usize, v2: &'a usize, v3: &'a usize) -> &'a usize {
        std::cmp::min(std::cmp::min(v1, v2), v3)
    }

    fn from_string_to_tree<'a>(_string: &String) -> Option<&'a dyn NodeTraits> {
        // string.find()
        None
    }
}
