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

/**
* <p>@project_name: xcsp3-rust
* </p>
* <p>@author: luhan zhen
* </p>
* <p>@date:  2023/7/15 13:23
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 **/

pub mod xcsp3_core {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ParseConstraintError {
        pub msg: String,
        pub r#type: ConstraintError,
    }

    impl ParseConstraintError {
        pub(crate) fn to_string(&self) -> String {
            format!("{:?}:{}", self.r#type, self.msg)
        }

        pub(crate) fn get_scope_not_found_error(s: &str) -> ParseConstraintError {
            const WEBSITE: &str =
                " please visit http://xcsp.org/specifications/constraints/";
            ParseConstraintError {
                msg: (s.to_owned() + WEBSITE),
                r#type: ConstraintError::ScopeNotFoundError,
            }
        }

        pub(crate) fn get_extension_error(s: &str) -> ParseConstraintError {
            const WEBSITE: &str =
                " please visit http://xcsp.org/specifications/constraints/generic/extension/";
            ParseConstraintError {
                msg: (s.to_owned() + WEBSITE),
                r#type: ConstraintError::ExtensionError,
            }
        }

        pub(crate) fn get_list_of_values_error(s: &str) -> ParseConstraintError {
            const WEBSITE: &str =
                " please visit http://xcsp.org/specifications/constraints";
            ParseConstraintError {
                msg: (s.to_owned() + WEBSITE),
                r#type: ConstraintError::ExtensionError,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ConstraintError {
        ExtensionError,
        ScopeNotFoundError,
    }
}
