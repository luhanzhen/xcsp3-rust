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
 * <p>@date:  2023/7/29 16:46
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

    pub struct XChannel<'a> {
        scope: Vec<XVarVal>,
        map: HashMap<String, &'a XDomainInteger>,
        set: &'a XVariableSet,
        start_index: Option<i32>,
        value: Option<XVarVal>,
    }

    impl XConstraintTrait for XChannel<'_> {
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

    impl<'a> XChannel<'a> {
        pub fn get_start_index(&self) -> &Option<i32> {
            &self.start_index
        }

        pub fn get_value(&self) -> &Option<XVarVal> {
            &self.value
        }

        pub fn from_str(
            list: &str,
            start_index_str: &str,
            value_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            match list_to_vec_var_val(list) {
                Ok(scope_vec_str) => {
                    let start_index = if start_index_str.is_empty() {
                        None
                    } else {
                        match start_index_str.parse::<i32>() {
                            Ok(n) => Some(n),
                            Err(_) => {
                                return Err(Xcsp3Error::get_constraint_channel_error(
                                    "parse channel constraint start_index error, ",
                                ));
                            }
                        }
                    };
                    let value = if value_str.is_empty() {
                        None
                    } else {
                        match XVarVal::from_string(value_str) {
                            None => {
                                return Err(Xcsp3Error::get_constraint_channel_error(
                                    "parse channel constraint value error, ",
                                ));
                            }
                            Some(v) => Some(v),
                        }
                    };
                    Ok(Self::new(scope_vec_str, set, start_index, value))
                }
                Err(e) => Err(e),
            }
        }
        pub fn new(
            scope: Vec<XVarVal>,
            set: &'a XVariableSet,
            start_index: Option<i32>,
            value: Option<XVarVal>,
        ) -> Self {
            Self {
                scope,
                map: Default::default(),
                set,
                start_index,
                value,
            }
        }
    }

    impl Display for XChannel<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret = String::default();
            for e in self.scope.iter() {
                ret.push('(');
                ret.push_str(&e.to_string());
                ret.push_str("), ")
            }
            if let Some(XVarVal::IntVar(e)) = &self.value {
                ret.push_str(&format!(" value = {}, ", e))
            }
            if let Some(n) = &self.start_index {
                ret.push_str(&format!(" start_index = {}, ", n))
            }
            write!(f, "XChannel: scope =  {}", ret,)
        }
    }
}
