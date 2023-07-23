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
* <p>@date:  2023/7/14 18:54
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
    use crate::utils::utils_functions::xcsp3_utils::{list_to_vec_var_val, tuple_to_vector};
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::time_interval::xcsp3_utils::TimeInterval;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::slice::Iter;

    #[derive(Clone)]
    pub struct XExtension<'a> {
        scope: Vec<XVarVal>,
        map: HashMap<String, &'a XDomainInteger>,
        set: &'a XVariableSet,
        ///if the  value in tuples is i32::MAX, then it is the star
        tuples: Vec<Vec<i32>>,
        is_support: bool,
    }

    impl Display for XExtension<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret = String::default();
            for e in self.scope.iter() {
                ret.push('(');
                ret.push_str(&e.to_string());
                ret.push_str("), ")
            }
            ret.push_str(&format!(
                "tuples = {:?}, is_support = {}",
                self.tuples, self.is_support
            ));
            write!(f, "XExtension: scope = {}", ret)
        }
    }

    impl XConstraintTrait for XExtension<'_> {
        fn get_scope_string(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        fn get_scope(&mut self) -> Vec<(&String, &XDomainInteger)> {
            for e in &self.scope {
                if let XVarVal::IntVar(s) = e {
                    if !self.map.contains_key(s) {
                        if let Ok(vec) = self.set.construct_scope(&[s]) {
                            for (vs, vv) in vec.into_iter() {
                                self.map.insert(vs, vv);
                            }
                        }
                    }
                }
            }
            let mut scope_vec_var: Vec<(&String, &XDomainInteger)> = vec![];
            for e in self.map.iter() {
                scope_vec_var.push((e.0, e.1))
            }
            scope_vec_var
        }
    }

    impl<'a> XExtension<'a> {
        /// construct the constraint from two strings and a bool
        pub fn from_str(
            list: &str,
            tuple: &str,
            is_support: bool,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            // let tt= TimeInterval::new();
            let a = match list_to_vec_var_val(list) {
                Ok(scope_vec_str) => {
                    match tuple_to_vector(tuple, !list.contains("..") && scope_vec_str.len() == 1) {
                        Ok(tuples) => Ok(XExtension::new(scope_vec_str, set, tuples, is_support)),
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            };
            // println!("{:?}",tt.get());
            a
        }

        pub fn new(
            scope: Vec<XVarVal>,
            set: &'a XVariableSet,
            tuples: Vec<Vec<i32>>,
            is_support: bool,
        ) -> Self {
            XExtension {
                scope,
                map: Default::default(),
                set,
                tuples,
                is_support,
            }
        }
        ///return the iter of the supports tuples, if the value is i32::MAX, then it is the star
        pub fn supports_iter(&self) -> Option<Iter<'_, Vec<i32>>> {
            if self.is_support {
                Some(self.tuples.iter())
            } else {
                None
            }
        }

        ///return the iter of the conflict tuples, if the value is i32::MAX, then it is the star
        pub fn conflicts_iter(&self) -> Option<Iter<'_, Vec<i32>>> {
            if !self.is_support {
                Some(self.tuples.iter())
            } else {
                None
            }
        }
    }
}
