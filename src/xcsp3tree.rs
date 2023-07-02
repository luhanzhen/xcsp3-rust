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

    struct Tree<'a> {
        root: Option<Box<&'a dyn NodeTraits>>,
        expr: String,
        list_of_variables: Vec<String>,
    }

    impl<'a> Tree<'a> {
        pub fn from_string(e: String) -> Tree<'a> {
            Tree {
                root: None,
                expr: e,
                list_of_variables: vec![],
            }
        }

        pub fn from_node(node: &dyn NodeTraits) -> Tree {
            Tree {
                root: Some(Box::new(node)),
                expr: String::default(),
                list_of_variables: vec![],
            }
        }

        pub fn arity(&self) -> usize {
            self.list_of_variables.len()
        }
    }
}
