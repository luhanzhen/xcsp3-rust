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
 * <p>@date:  2023/7/29 16:48
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */
pub mod xcsp3_core {
    use crate::constraints::xconstraint_trait::xcsp3_core::XConstraintTrait;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    pub struct XCardinality<'a> {
        scope: Vec<XVarVal>,
        values: Vec<XVarVal>,
        occurs: Vec<XVarVal>,
        map: HashMap<String, &'a XDomainInteger>,
        set: &'a XVariableSet,
        closed: Option<bool>,
    }
    impl Display for XCardinality<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret = String::default();
            for e in self.scope.iter() {
                ret.push('(');
                ret.push_str(&e.to_string());
                ret.push_str("), ")
            }
            ret.push_str("  values = ");
            for e in self.values.iter() {
                ret.push('(');
                ret.push_str(&e.to_string());
                ret.push_str("), ")
            }
            if let Some(c) = self.closed {
                ret.push_str(&format!("  closed = {}", c));
            }
            ret.push_str("  occurs = ");
            for e in self.occurs.iter() {
                ret.push('(');
                ret.push_str(&e.to_string());
                ret.push_str("), ")
            }

            write!(f, "XCardinality: list =  {},  ", ret,)
        }
    }

    impl<'a> XCardinality<'a> {
        pub fn from_str(
            list: &str,
            values_str: &str,
            occurs_str: &str,
            closed_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope = match list_to_vec_var_val(list) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            let value = match list_to_vec_var_val(values_str) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            let occurs = match list_to_vec_var_val(occurs_str) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            let closed = if !closed_str.is_empty() {
                match closed_str.parse::<bool>() {
                    Ok(n) => Some(n),
                    Err(_) => {
                        return Err(Xcsp3Error::get_constraint_cardinality_error(
                            "parse cardinality  constraint closed error, ",
                        ));
                    }
                }
            } else {
                None
            };
            Ok(Self::new(scope, value, occurs, set, closed))
        }

        pub fn new(
            scope: Vec<XVarVal>,
            values: Vec<XVarVal>,
            occurs: Vec<XVarVal>,
            set: &'a XVariableSet,
            closed: Option<bool>,
        ) -> Self {
            Self {
                scope,
                values,
                occurs,
                map: Default::default(),
                set,
                closed,
            }
        }

        pub fn get_occurs(&self) -> &Vec<XVarVal> {
            &self.occurs
        }

        pub fn get_closed(&self) -> &Option<bool> {
            &self.closed
        }

        pub fn get_values(&self) -> &Vec<XVarVal> {
            &self.values
        }
    }

    impl XConstraintTrait for XCardinality<'_> {
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
}
