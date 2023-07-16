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
* <p>@date:  2023/7/15 15:13
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 */

pub mod xcsp3_core {
    use crate::constraints::xconstraint_trait::xcsp3_core::XConstraintTrait;
    use crate::utils::xcsp3utils::xcsp3_core::{list_to_scope_ids, list_to_transitions};
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;

    #[derive(Clone)]
    pub struct XMdd<'a> {
        scope_vec_str: Vec<String>,
        transitions: Vec<(String, i32, String)>,
        scope_vec_var: Vec<(String, &'a XDomainInteger)>,
    }

    impl XConstraintTrait for XMdd<'_> {
        fn to_string(&self) -> String {
            format!(
                "XMdd: scope = {:?},  transitions = {:?}",
                self.scope_vec_str, self.transitions,
            )
        }

        fn get_scope_string(&self) -> &Vec<String> {
            &self.scope_vec_str
        }

        fn get_scope(&self) -> &Vec<(String, &XDomainInteger)> {
            &self.scope_vec_var
        }
    }

    impl XMdd<'_> {
        pub fn from_str(list: &str, transitions_str: &str) -> Option<Self> {
            let scope = list_to_scope_ids(list);
            match list_to_transitions(transitions_str) {
                Ok(transitions) => Some(XMdd::new(scope, transitions)),
                Err(_) => None,
            }
        }

        pub fn get_transitions(&self) -> &Vec<(String, i32, String)> {
            &self.transitions
        }
        pub fn new(scope_vec_str: Vec<String>, transitions: Vec<(String, i32, String)>) -> Self {
            XMdd {
                scope_vec_str,
                transitions,
                scope_vec_var: vec![],
            }
        }
    }
}
