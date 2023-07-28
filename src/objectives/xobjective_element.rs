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
    use crate::objectives::xobjectives_type::xcsp3_core::*;
    use std::fmt::{Display, Formatter};

    #[derive(Clone, Debug)]
    pub struct XObjectiveElement {
        // target: XObjectivesType,
        operator: XObjectivesOperator,
        scope: Vec<XVarVal>,
        coeffs: Vec<XVarVal>,
    }

    impl XObjectiveElement {
        pub fn new(
            // target: XObjectivesType,
            operator: XObjectivesOperator,
            scope: Vec<XVarVal>,
            coeffs: Vec<XVarVal>,
        ) -> Self {
            Self {
                // target,
                operator,
                scope,
                coeffs,
            }
        }

        pub fn from_str() -> Result<Self, Xcsp3Error> {
            Err(Xcsp3Error::get_objective_target_error("e"))
        }

        // pub fn target(&self) -> &XObjectivesType {
        //     &self.target
        // }
        pub fn operator(&self) -> &XObjectivesOperator {
            &self.operator
        }
        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }
        pub fn coeffs(&self) -> &Vec<XVarVal> {
            &self.coeffs
        }
    }

    impl Display for XObjectiveElement {
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
                "XObjective:  operator = {:?} scope =  {}, coeffs = {}",
                self.operator, ret1, ret2
            )
        }
    }
}
