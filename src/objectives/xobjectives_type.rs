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
 * <p>@date:  2023/7/23 15:13
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
    use crate::objectives::xobjective_target::xcsp3_core::XObjectiveElement;
    use std::fmt::{Display, Formatter};
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;

    #[derive(Clone)]
    pub enum XObjectivesType {
        XObjectiveNone(Xcsp3Error),
        Minimize(XObjective),
        Maximize(XObjective),
    }

    impl Display for XObjectivesType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    XObjectivesType::Minimize(o) => {
                        format!("Minimize: {}", o.to_string())
                    }
                    XObjectivesType::Maximize(o) => {
                        format!("Maximize: {}", o.to_string())
                    }
                    XObjectivesType::XObjectiveNone(e) =>  format!(
                        "XObjectiveNone: there must be an error when parse this objective. Error is {}",e
                    ),
                }
            )
        }
    }

    #[derive(Clone)]
    pub enum XObjective {
        XObjectiveElement(XObjectiveElement),
        Expression(ExpressionTree),
    }

    impl Display for XObjective {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    XObjective::XObjectiveElement(e) => {
                        e.to_string()
                    }
                    XObjective::Expression(e) => {
                        e.to_string()
                    }
                }
            )
        }
    }

    // impl XObjectivesType {
    //     pub fn get_objectives_type_by_str(op: &str) -> Option<Self> {
    //         match op {
    //             "minimize" => Some(Self::Minimize),
    //             "maximize" => Some(Self::Maximize),
    //             _ => None,
    //         }
    //     }
    // }

    #[derive(Clone, Debug)]
    pub enum XObjectivesOperator {
        Sum,
        Product,
        Minimum,
        Maximum,
        NValues,
        Lex,
    }

    impl XObjectivesOperator {
        pub fn get_objectives_operator_by_str(op: &str) -> Option<Self> {
            match op {
                "Sum" => Some(Self::Sum),
                "Product" => Some(Self::Product),
                "minimize" => Some(Self::Minimum),
                "maximize" => Some(Self::Maximum),
                "nValues" => Some(Self::NValues),
                "lex" => Some(Self::Lex),
                _ => None,
            }
        }
    }
}
