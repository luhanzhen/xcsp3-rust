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
 * <p>@date:  2023/7/23 15:48
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */
pub mod xcsp3_core {
    use std::fmt::{Display, Formatter};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ObjectiveError {
        UnknownScope,
        UnknownCoeffs,
        UnknownType,
        UnknownTarget,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ParseObjectivesError {
        pub msg: String,
        pub r#type: ObjectiveError,
    }

    impl Display for ParseObjectivesError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}: {}", self.r#type, self.msg)
        }
    }

    impl ParseObjectivesError {
        pub(crate) fn get_scope_error(s: &str) -> ParseObjectivesError {
            const WEBSITE: &str = " please visit http://xcsp.org/specifications/objectives/";
            ParseObjectivesError {
                msg: (s.to_owned() + WEBSITE),
                r#type: ObjectiveError::UnknownScope,
            }
        }

        pub(crate) fn get_coeffs_error(s: &str) -> ParseObjectivesError {
            const WEBSITE: &str = " please visit http://xcsp.org/specifications/objectives/";
            ParseObjectivesError {
                msg: (s.to_owned() + WEBSITE),
                r#type: ObjectiveError::UnknownCoeffs,
            }
        }

        pub(crate) fn get_type_error(s: &str) -> ParseObjectivesError {
            const WEBSITE: &str = " please visit http://xcsp.org/specifications/objectives/";
            ParseObjectivesError {
                msg: (s.to_owned() + WEBSITE),
                r#type: ObjectiveError::UnknownType,
            }
        }

        pub(crate) fn get_target_error(s: &str) -> ParseObjectivesError {
            const WEBSITE: &str = " please visit http://xcsp.org/specifications/objectives/";
            ParseObjectivesError {
                msg: (s.to_owned() + WEBSITE),
                r#type: ObjectiveError::UnknownTarget,
            }
        }
    }
}
