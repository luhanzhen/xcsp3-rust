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
 * <p>@date:  2023/8/3 18:29
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */
pub mod xcsp3_core {
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    pub struct XNoOverlapKDim<'a> {
        scope: Vec<Vec<XVarVal>>,
        lengths: Vec<Vec<XVarVal>>,
        map: HashMap<String, &'a XDomainInteger>,
        set: &'a XVariableSet,
        zero_ignored: Option<bool>,
    }

    impl<'a> XNoOverlapKDim<'a> {
        pub fn from_str(
            list: &str,
            lengths_str: &str,
            zero_ignored_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope: Vec<Vec<XVarVal>> =
                {
                    let mut sc = vec![];
                    let binding = list.replace(['(', ',',')']," ");
                    let spilt: Vec<&str> = binding.split_whitespace().collect();
                    for e in spilt.iter()
                    {
                        sc.push(
                        match list_to_vec_var_val(e) {
                            Ok(n) => n,
                            Err(e) => {
                                return Err(e);
                            }
                        })
                    }
                    sc
                };
            let lengths =
                {
                    let mut le = vec![];
                    let binding = lengths_str.replace(['(', ',',')']," ");
                    let spilt: Vec<&str> = binding.split_whitespace().collect();
                    for e in spilt.iter()
                    {
                        le.push(
                            match list_to_vec_var_val(e) {
                                Ok(n) => n,
                                Err(e) => {
                                    return Err(e);
                                }
                            })
                    }
                    le
                };
            let zero_ignored = if !zero_ignored_str.is_empty() {
                match zero_ignored_str.parse::<bool>() {
                    Ok(n) => Some(n),
                    Err(_) => {
                        return Err(Xcsp3Error::get_constraint_no_overlap_error(
                            "parse XNoOverlap constraint zero_ignored error, ",
                        ));
                    }
                }
            } else {
                None
            };
            Ok(Self::new(scope, lengths, set, zero_ignored))
        }
        pub fn new(
            scope: Vec<Vec<XVarVal>>,
            lengths: Vec<Vec<XVarVal>>,
            set: &'a XVariableSet,
            zero_ignored: Option<bool>,
        ) -> Self {
            Self {
                scope,
                lengths,
                map: Default::default(),
                set,
                zero_ignored,
            }
        }
        pub fn lengths(&self) -> &Vec<Vec<XVarVal>> {
            &self.lengths
        }
        pub fn zero_ignored(&self) -> Option<bool> {
            self.zero_ignored
        }

        fn get_scope_string(&self) -> &Vec<Vec<XVarVal>> {
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

    impl Display for XNoOverlapKDim<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret = String::default();
            for e in self.scope.iter() {
                ret.push('(');
                ret.push_str(&e.to_string());
                ret.push_str("), ")
            }
            ret.push_str("  lengths = ");
            for e in self.lengths.iter() {
                ret.push('(');
                ret.push_str(&e.to_string());
                ret.push_str("), ")
            }
            if let Some(n) = &self.zero_ignored {
                ret.push_str(&format!(" zeroIgnored = {}, ", n))
            }
            write!(f, "XNoOverlap: scope =  {}, ", ret, )
        }
    }
}
