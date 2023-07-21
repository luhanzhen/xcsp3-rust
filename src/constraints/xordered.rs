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
* <p>@date:  2023/7/15 14:56
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

    use crate::utils::utils_functions::xcsp3_utils::{list_to_scope_ids, list_to_values};
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    #[derive(Clone)]
    pub struct XOrdered<'a> {
        scope_vec_str: Vec<String>,
        scope_vec_var: Vec<(String, &'a XDomainInteger)>,
        lengths: Vec<i32>,
        operator: String,
    }

    impl XConstraintTrait for XOrdered<'_> {
        fn to_string(&self) -> String {
            let mut ret = "XOrdered: scope =  ".to_string();
            for e in self.scope_vec_var.iter() {
                ret.push_str(e.0.as_str());
                ret.push('(');
                ret.push_str(e.1.to_string().as_str());
                ret.push_str("), ")
            }
            ret.push_str(&format!(
                " lengths = {:?}, operator = {}",
                self.lengths, self.operator
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

    impl<'a> XOrdered<'a> {
        pub fn from_str(
            list: &str,
            lengths_str: &str,
            operator: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope_vec_str = list_to_scope_ids(list);
            match set.construct_scope(&scope_vec_str) {
                Ok(scope) => match list_to_values(lengths_str) {
                    Ok(lengths) => Ok(XOrdered::new(
                        scope_vec_str,
                        scope,
                        lengths,
                        operator.to_string(),
                    )),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
        }

        pub fn from_str_without_lengths(
            list: &str,
            operator: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope_vec_str = list_to_scope_ids(list);
            match set.construct_scope(&scope_vec_str) {
                Ok(scope) => Ok(XOrdered::new(
                    scope_vec_str,
                    scope,
                    vec![],
                    operator.to_string(),
                )),
                Err(e) => Err(e),
            }
        }

        pub fn new(
            scope_vec_str: Vec<String>,
            scope_vec_var: Vec<(String, &'a XDomainInteger)>,
            lengths: Vec<i32>,
            operator: String,
        ) -> Self {
            XOrdered {
                scope_vec_str,
                scope_vec_var,
                lengths,
                operator,
            }
        }
        pub fn get_lengths(&self) -> &Vec<i32> {
            &self.lengths
        }
        pub fn get_operator(&self) -> &str {
            &self.operator
        }
    }
}
