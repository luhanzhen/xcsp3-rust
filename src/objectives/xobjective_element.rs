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
 * <p>@date:  2023/7/23 15:36
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
    use std::collections::HashMap;

    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::fmt::{Display, Formatter};

    #[derive(Clone, Debug)]
    pub enum XElementOperator {
        Sum,
        Product,
        Minimum,
        Maximum,
        NValues,
        Lex,
    }

    impl XElementOperator {
        pub fn get_objectives_operator_by_str(op: &str) -> Option<Self> {
            match op {
                "sum" => Some(Self::Sum),
                "product" => Some(Self::Product),
                "minimum" => Some(Self::Minimum),
                "maximum" => Some(Self::Maximum),
                "nValues" => Some(Self::NValues),
                "lex" => Some(Self::Lex),
                _ => None,
            }
        }
    }

    #[derive(Clone)]
    pub struct XObjectiveElement<'a> {
        operator: XElementOperator,
        scope: Vec<XVarVal>,
        coeffs: Vec<XVarVal>,
        set: &'a XVariableSet,
        map: HashMap<String, &'a XDomainInteger>,
    }

    impl<'a> XObjectiveElement<'a> {
        pub fn get_scope_string(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn get_scope(&mut self) -> Vec<(&String, &XDomainInteger)> {
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

        pub fn new(
            operator: XElementOperator,
            scope: Vec<XVarVal>,
            coeffs: Vec<XVarVal>,
            set: &'a XVariableSet,
        ) -> Self {
            Self {
                operator,
                scope,
                coeffs,
                set,
                map: Default::default(),
            }
        }

        pub fn from_str(
            list_str: &str,
            coeffs_str: &str,
            ope_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            match list_to_vec_var_val(list_str) {
                Ok(scope_vec_str) => match list_to_vec_var_val(coeffs_str) {
                    Ok(coef_vec_str) => {
                        match XElementOperator::get_objectives_operator_by_str(ope_str) {
                            None => Err(Xcsp3Error::get_objective_target_error(
                                "parse objective type error, ",
                            )),
                            Some(v) => Ok(Self::new(v, scope_vec_str, coef_vec_str, set)),
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
            // Err(Xcsp3Error::get_objective_target_error("e"))
        }

        pub fn get_operator(&self) -> &XElementOperator {
            &self.operator
        }
        pub fn get_coeffs(&self) -> &Vec<XVarVal> {
            &self.coeffs
        }
    }

    impl Display for XObjectiveElement<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret1 = String::default();
            for e in self.scope.iter() {
                ret1.push('(');
                ret1.push_str(&e.to_string());
                ret1.push_str("), ")
            }
            let mut ret2 = String::default();
            for e in self.coeffs.iter() {
                ret2.push('(');
                ret2.push_str(&e.to_string());
                ret2.push_str("), ")
            }
            write!(
                f,
                "operator = {:?} scope =  {}, coeffs = {}",
                self.operator, ret1, ret2
            )
        }
    }
}
