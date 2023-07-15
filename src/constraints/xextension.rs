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
* <p>@date:  2023/7/14 18:54
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 **/

pub mod xcsp3_core {
    use crate::constraints::xconstraint_trait::xcsp3_core::XConstraintTrait;
    use crate::utils::xcsp3utils::xcsp3_core::{list_to_scope_ids, tuple_to_vector};
    use std::slice::Iter;

    #[derive(Clone)]
    pub struct XExtension {
        scope: Vec<String>,
        tuples: Vec<Vec<i32>>,
        is_support: bool,
    }

    impl XConstraintTrait for XExtension {
        fn to_string(&self) -> String {
            format!(
                "XExtension: scope = {:?}, tuples = {:?}, is_support = {}",
                self.scope, self.tuples, self.is_support
            )
        }

        fn get_scope(&self) -> &Vec<String> {
            &self.scope
        }
    }

    impl XExtension {
        /// construct the constraint from two strings and a bool
        pub fn from_str(list: &str, tuple: &str, is_support: bool) -> Option<XExtension> {
            if let Ok(tuples) = tuple_to_vector(tuple) {
                let scope = list_to_scope_ids(list);
                Some(XExtension {
                    scope,
                    tuples,
                    is_support,
                })
            } else {
                None
            }
        }

        ///return the iter of the supports tuples
        pub fn supports_iter(&self) -> Option<Iter<'_, Vec<i32>>> {
            if self.is_support {
                Some(self.tuples.iter())
            } else {
                None
            }
        }

        ///return the iter of the conflict tuples
        pub fn conflicts_iter(&self) -> Option<Iter<'_, Vec<i32>>> {
            if !self.is_support {
                Some(self.tuples.iter())
            } else {
                None
            }
        }
    }
}
