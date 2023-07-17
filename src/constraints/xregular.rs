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
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;

    use crate::utils::xcsp3utils::xcsp3_core::{list_to_scope_ids, list_to_transitions};
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    #[derive(Clone)]
    pub struct XRegular<'a> {
        scope_vec_var: Vec<(String, &'a XDomainInteger)>,
        scope_vec_str: Vec<String>,
        start: String,
        r#final: Vec<String>,
        transitions: Vec<(String, i32, String)>,
    }

    impl XConstraintTrait for XRegular<'_> {
        fn to_string(&self) -> String {
            let mut ret = format!("XRegular: scope =  ",);
            for e in self.scope_vec_var.iter() {
                ret.push_str(e.0.as_str());
                ret.push_str("(");
                ret.push_str(e.1.to_string().as_str());
                ret.push_str("), ")
            }
            ret.push_str(&format!(
                " transitions = {:?}, start = {}, final = {:?},",
                self.transitions, self.start, self.r#final
            ));
            ret
        }
        fn get_scope_string(&self) -> &Vec<String> {
            &self.scope_vec_str
        }

        fn get_scope(&self) -> &Vec<(String, &XDomainInteger)> {
            &self.scope_vec_var
        }
    }

    impl<'a> XRegular<'a> {
        pub fn from_str(
            list: &str,
            transitions_str: &str,
            start_str: &str,
            final_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope_vec_str = list_to_scope_ids(list);

            match set.construct_scope(&scope_vec_str) {
                Ok(scope) => {
                    let mut finals: Vec<String> = vec![];
                    let t_final: Vec<&str> = final_str.split_whitespace().collect();
                    for s in t_final.iter() {
                        finals.push(s.to_string());
                    }
                    match list_to_transitions(transitions_str) {
                        Ok(transitions) => Ok(XRegular::new(
                            scope_vec_str,
                            scope,
                            start_str.to_string(),
                            finals,
                            transitions,
                        )),
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        }

        pub fn new(
            scope_vec_str: Vec<String>,
            scope_vec_var: Vec<(String, &'a XDomainInteger)>,
            start: String,
            r#final: Vec<String>,
            transitions: Vec<(String, i32, String)>,
        ) -> Self {
            XRegular {
                scope_vec_var,
                scope_vec_str,
                start,
                r#final,
                transitions,
            }
        }

        pub fn get_start(&self) -> &str {
            &self.start
        }

        pub fn get_final(&self) -> &Vec<String> {
            &self.r#final
        }

        pub fn get_transitions(&self) -> &Vec<(String, i32, String)> {
            &self.transitions
        }
    }
}
