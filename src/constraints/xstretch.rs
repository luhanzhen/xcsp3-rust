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
 * <p>@date:  2023/7/31 12:42
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
    pub struct XStretch<'a> {
        scope: Vec<XVarVal>,
        map: HashMap<String, &'a XDomainInteger>,
        set: &'a XVariableSet,
        values: Vec<XVarVal>,
        widths: Vec<XVarVal>,
        patterns: Option<Vec<XVarVal>>,
    }

    impl<'a> XStretch<'a> {
        pub fn from_str(
            list: &str,
            value_str: &str,
            widths_str: &str,
            patterns_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope = match list_to_vec_var_val(list) {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            };
            let value = match list_to_vec_var_val(value_str) {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            };
            let widths = match list_to_vec_var_val(widths_str) {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            };
            let patterns = if patterns_str.is_empty() {
                None
            } else {
                match list_to_vec_var_val(patterns_str) {
                    Ok(n) => Some(n),
                    Err(e) => {
                        return Err(e);
                    }
                }
            };
            Ok(Self::new(scope, set, value, widths, patterns))
        }

        pub fn new(
            scope: Vec<XVarVal>,
            set: &'a XVariableSet,
            values: Vec<XVarVal>,
            widths: Vec<XVarVal>,
            patterns: Option<Vec<XVarVal>>,
        ) -> Self {
            Self {
                scope,
                map: Default::default(),
                set,
                values,
                widths,
                patterns,
            }
        }
        pub fn values(&self) -> &Vec<XVarVal> {
            &self.values
        }
        pub fn widths(&self) -> &Vec<XVarVal> {
            &self.widths
        }
        pub fn patterns(&self) -> &Option<Vec<XVarVal>> {
            &self.patterns
        }
    }

    impl XConstraintTrait for XStretch<'_> {
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
    impl Display for XStretch<'_> {
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
            ret.push_str("  widths = ");
            for e in self.widths.iter() {
                ret.push('(');
                ret.push_str(&e.to_string());
                ret.push_str("), ")
            }
            if let Some(pp) = &self.patterns {
                ret.push_str("  patterns = ");
                for e in pp.iter() {
                    ret.push('(');
                    ret.push_str(&e.to_string());
                    ret.push_str("), ")
                }
            }
            write!(f, "XStretch: list =  {} ", ret,)
        }
    }
}
