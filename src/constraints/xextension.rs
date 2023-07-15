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
* <p>@date:  2023/7/14 18:54
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 */

pub mod xcsp3_core {
    use crate::constraints::xconstraint_trait::xcsp3_core::XConstraintTrait;
    use crate::utils::xcsp3utils::xcsp3_core::{list_to_scope_ids, tuple_to_vector};
    use crate::variables::xdomain_integer::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::slice::Iter;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;

    #[derive(Clone)]
    pub struct XExtension<'a> {
        scope_vec_str: Vec<String>,
        scope_vec_var: Vec<(String, &'a XDomainInteger)>,
        tuples: Vec<Vec<i32>>,
        is_support: bool,
    }

    impl XConstraintTrait for XExtension<'_> {
        fn to_string(&self) -> String {
            format!(
                "XExtension: scope = {:?}, tuples = {:?}, is_support = {}",
                self.scope_vec_str, self.tuples, self.is_support
            )
        }

        fn get_scope_string(&self) -> &Vec<String> {
            &self.scope_vec_str
        }

        fn get_scope(&self) -> &Vec<(String, &XDomainInteger)> {
            &self.scope_vec_var
        }
    }

    impl XExtension<'_> {
        /// construct the constraint from two strings and a bool
        pub fn from_str<'a>(list: &str, tuple: &str, is_support: bool) -> Option<XExtension<'a>> {
            let scope = list_to_scope_ids(list);
            if let Ok(tuples) = tuple_to_vector(tuple, scope.len() == 1) {
                Some(XExtension::new(scope, tuples, is_support))
            } else {
                None
            }
        }



        pub fn new<'a>(
            scope: Vec<String>,
            tuples: Vec<Vec<i32>>,
            is_support: bool,
        ) -> XExtension<'a> {
            XExtension {
                scope_vec_str: scope,
                scope_vec_var: vec![],
                tuples,
                is_support,
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
