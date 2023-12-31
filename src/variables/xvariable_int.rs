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
* <p>@date:  2023/7/14 18:40
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 **/

pub mod xcsp3_core {
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use std::fmt::{Display, Formatter};
    // use crate::variables::xvariable_trait::xcsp3_core::XVariableTrait;

    #[derive(Clone)]
    pub struct XVariableInt {
        pub(crate) domain: XDomainInteger,
        pub(crate) id: String,
    }

    impl XVariableInt {
        pub fn new(id: String, domain: XDomainInteger) -> XVariableInt {
            XVariableInt { id, domain }
        }

        pub fn clone_domain(&self) -> XDomainInteger {
            self.domain.clone()
        }
    }

    impl Display for XVariableInt {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "XVariableInt: id = {}, domain = {}",
                self.id, self.domain
            )
        }
    }

    // impl XVariableTrait for XVariableInt {
    //     fn to_string(&self) -> String {
    //         format!(
    //             "XVariableInt: id = {}, domain = {}",
    //             self.id,
    //             self.domain.to_string()
    //         )
    //     }
    // }
}
