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
* <p>@date:  2023/7/14 18:35
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 **/

pub mod xcsp3_core {
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_array::xcsp3_core::XVariableArray;
    use crate::variables::xvariable_int::xcsp3_core::XVariableInt;
    use std::fmt::{Display, Formatter};
    // use crate::variables::xvariable_trait::xcsp3_core::XVariableTrait;
    use crate::variables::xvariable_tree::xcsp3_core::XVariableTree;

    #[derive(Clone)]
    pub enum XVariableType {
        XVariableNone(Xcsp3Error),
        XVariableArray(XVariableArray),
        XVariableInt(XVariableInt),
        XVariableTree(XVariableTree),
    }

    impl XVariableType {
        pub fn new_int(id: &str, domain: XDomainInteger) -> XVariableType {
            XVariableType::XVariableInt(XVariableInt::new(id.to_string(), domain))
        }

        pub fn new_array(id: &str, sizes: &str, domain: XDomainInteger) -> XVariableType {
            match XVariableArray::new(id, sizes, domain) {
                Ok(array) => XVariableType::XVariableArray(array),
                Err(e) => XVariableType::XVariableNone(e),
            }
        }

        pub fn new_tree(
            id: &str,
            sizes: &str,
            domain_for: Vec<&String>,
            domain_value: Vec<&String>,
        ) -> Result<Self, Xcsp3Error> {
            match XVariableTree::new(id, sizes, domain_for, domain_value) {
                Ok(t) => Ok(XVariableType::XVariableTree(t)),
                Err(e) => Err(e),
            }
        }

        pub fn get_id(&self) -> String {
            match self {
                XVariableType::XVariableArray(v) => v.id.clone(),
                XVariableType::XVariableInt(v) => v.id.clone(),
                XVariableType::XVariableTree(v) => v.id.clone(),
                _ => String::default(),
            }
        }
    }

    impl Display for XVariableType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    XVariableType::XVariableArray(a) => a.to_string(),
                    XVariableType::XVariableInt(i) => i.to_string(),
                    XVariableType::XVariableTree(t) => t.to_string(),
                    _ => {
                        String::from(
                            "XVariableNone: there must be an error when parse this variable.",
                        )
                    }
                }
            )
        }
    }
}
