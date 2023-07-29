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
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::objectives::xobjective_element::xcsp3_core::XObjectiveElement;
    use crate::objectives::xobjective_expression::xcsp3_core::XObjectiveExpression;
    use std::fmt::{Display, Formatter};

    #[derive(Clone)]
    pub enum XObjectivesType<'a> {
        XObjectiveNone(Xcsp3Error),
        Minimize(XObjective<'a>),
        Maximize(XObjective<'a>),
    }

    impl Display for XObjectivesType<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    XObjectivesType::Minimize(o) => {
                        format!("Minimize: {}", o)
                    }
                    XObjectivesType::Maximize(o) => {
                        format!("Maximize: {}", o)
                    }
                    XObjectivesType::XObjectiveNone(e) =>  format!(
                        "XObjectiveNone: there must be an error when parse this objective. Error is {}",e
                    ),
                }
            )
        }
    }

    #[derive(Clone)]
    pub enum XObjective<'a> {
        XObjectiveElement(XObjectiveElement<'a>),
        XObjectiveExpression(XObjectiveExpression<'a>),
    }

    impl Display for XObjective<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    XObjective::XObjectiveElement(e) => {
                        e.to_string()
                    }
                    XObjective::XObjectiveExpression(e) => {
                        e.to_string()
                    }
                }
            )
        }
    }
}
