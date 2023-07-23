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

    #[derive(Clone, Debug)]
    pub enum XObjectivesType {
        Minimize,
        Maximize,
    }
    impl XObjectivesType {
        pub fn get_objectives_type_by_str(op: &str) -> Option<Self> {
            match op {
                "minimize" => Some(Self::Minimize),
                "maximize" => Some(Self::Maximize),
                _ => None,
            }
        }
    }

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
