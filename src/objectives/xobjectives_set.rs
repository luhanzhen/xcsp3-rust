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
 * <p>@date:  2023/7/23 15:11
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

pub mod xcsp3_core {

    use crate::data_structs::expression_tree::xcsp3_utils::ExpressionTree;
    use crate::objectives::xobjectives_type::xcsp3_core::{XObjective, XObjectivesType};
    use std::slice::Iter;
    use crate::objectives::xobjective_expression::xcsp3_core::XObjectiveExpression;


    pub struct XObjectivesSet {
        objectives: Vec<XObjectivesType>,
    }

    impl Default for XObjectivesSet {
        fn default() -> Self {
            Self::new()
        }
    }

    impl XObjectivesSet {
        pub fn build_max(&mut self, list: &str, coeffs: &str, expression: &str, type_str: &str) {
            // println!("{} {} {} {}", list, coeffs, expression, type_str);
            if !expression.is_empty() {
                match XObjectiveExpression::from_expr(expression) {
                    Ok(xoe) => {
                        self.objectives.push(XObjectivesType::Maximize(XObjective::XObjectiveExpression(xoe)));
                    }
                    Err(e) => self.objectives.push(XObjectivesType::XObjectiveNone(e)),
                }
            }
        }

        pub fn build_min(&mut self, list: &str, coeffs: &str, expression: &str, type_str: &str) {
            // println!("{} {} {} {}", list, coeffs, expression, type_str)
               if !expression.is_empty() {
                match XObjectiveExpression::from_expr(expression) {
                    Ok(xoe) => {
                        self.objectives.push(XObjectivesType::Minimize(XObjective::XObjectiveExpression(xoe)));
                    }
                    Err(e) => self.objectives.push(XObjectivesType::XObjectiveNone(e)),
                }
            }
        }


        pub fn iter(&self) -> Iter<'_, XObjectivesType> {
            self.objectives.iter()
        }
        pub fn new() -> Self {
            Self { objectives: vec![] }
        }
    }
}
