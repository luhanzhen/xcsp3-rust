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
* <p>@date:  2023/7/21 10:42
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
    use crate::constraints::xrelational_operand::xcsp3_core::Operand;
    use crate::constraints::xrelational_operator::xcsp3_core::Operator;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils::xcsp3_utils::list_to_scope_ids;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    #[derive(Clone)]
    pub struct XMaxMin<'a> {
        scope_vec_str: Vec<String>,
        scope_vec_var: Vec<(String, &'a XDomainInteger)>,
        operator: Operator,
        operand: Operand,
        is_maximum_or_minimum: bool, // true if maximum, false if minimum
    }

    impl XConstraintTrait for XMaxMin<'_> {
        fn to_string(&self) -> String {
            let mut ret: String;
            if self.is_maximum() {
                ret = format!("XMaximum: scope =  ",);
            } else {
                ret = format!("XMinimum: scope =  ",);
            }
            for e in self.scope_vec_var.iter() {
                ret.push_str(e.0.as_str());
                ret.push_str("(");
                ret.push_str(e.1.to_string().as_str());
                ret.push_str("), ")
            }
            ret.push_str(&format!(
                " Operator = {:?}, Operand = {:?}",
                self.operator, self.operand
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

    impl<'a> XMaxMin<'a> {
        pub fn from_str(
            list: &str,
            condition: &str,
            is_maximum_or_minimum: bool,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope_vec_str = list_to_scope_ids(list);
            match set.construct_scope(&scope_vec_str) {
                Ok(scope) => {
                    let condition = condition
                        .replace('(', " ")
                        .replace(')', " ")
                        .replace(',', " ");
                    let spilt: Vec<&str> = condition.split_whitespace().collect();
                    let ope: Operator;
                    match Operator::get_operator_by_str(spilt[0]) {
                        None => {
                            return Err(Xcsp3Error::get_constraint_sum_error(
                                "parse sum constraint error, ",
                            ))
                        }
                        Some(o) => ope = o,
                    }
                    let rand: Operand;

                    match Operand::get_operand_by_str(&spilt[1..], &ope) {
                        None => {
                            return Err(Xcsp3Error::get_constraint_sum_error(
                                "parse sum constraint error, ",
                            ))
                        }
                        Some(r) => rand = r,
                    }
                    Ok(Self::new(
                        scope_vec_str,
                        scope,
                        ope,
                        rand,
                        is_maximum_or_minimum,
                    ))
                }
                Err(e) => Err(e),
            }
        }
        pub fn new(
            scope_vec_str: Vec<String>,
            scope_vec_var: Vec<(String, &'a XDomainInteger)>,
            operator: Operator,
            operand: Operand,
            is_maximum_or_minimum: bool,
        ) -> Self {
            Self {
                scope_vec_str,
                scope_vec_var,
                operator,
                operand,
                is_maximum_or_minimum,
            }
        }
        pub fn is_maximum(&self) -> bool {
            return self.is_maximum_or_minimum;
        }

        pub fn is_minimum(&self) -> bool {
            return !self.is_maximum_or_minimum;
        }

        pub fn get_operand(&self) -> &Operand {
            &self.operand
        }

        pub fn get_operator(&self) -> &Operator {
            &self.operator
        }
    }
}