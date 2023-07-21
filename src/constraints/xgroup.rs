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
 * <p>@date:  2023/7/19 12:20
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

pub mod xcsp3_core {
    use crate::constraints::xconstraint_trait::xcsp3_core::XConstraintTrait;
    use crate::constraints::xconstraint_type::xcsp3_core::XConstraintType;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_scope_ids;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::fmt::{Display, Formatter};

    #[derive(Clone)]
    pub struct XGroup<'a> {
        args: Vec<Vec<(String, &'a XDomainInteger)>>,
        template: Box<XConstraintType<'a>>,

        /// redundant data, just for XConstraintTrait,
        scope_vec_str: Vec<String>,
        scope_vec_var: Vec<(String, &'a XDomainInteger)>,
    }

    impl Display for XGroup<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl XConstraintTrait for XGroup<'_> {
        // fn to_string(&self) -> String {
        //     let mut ret = "XGroup: [constraint =  [".to_string();
        //     ret.push_str(&self.template.to_string());
        //     ret.push_str("]] [args = ");
        //     for a in self.args.iter() {
        //         ret.push('[');
        //         for e in a.iter() {
        //             ret.push_str(e.0.as_str());
        //             ret.push('(');
        //             ret.push_str(e.1.to_string().as_str());
        //             ret.push_str("), ")
        //         }
        //         ret.push(']');
        //     }
        //     ret.push(']');
        //     ret
        // }

        fn get_scope_string(&self) -> &Vec<String> {
            &self.scope_vec_str
        }

        fn get_scope(&self) -> &Vec<(String, &XDomainInteger)> {
            &self.scope_vec_var
        }
    }

    impl<'a> XGroup<'a> {
        pub fn get_args(&self) -> &Vec<Vec<(String, &'a XDomainInteger)>> {
            &self.args
        }

        pub fn get_template(&self) -> &XConstraintType<'a> {
            &self.template
        }

        pub fn from_str(
            cc: XConstraintType<'a>,
            arg_str: &[String],
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let mut args: Vec<Vec<(String, &'a XDomainInteger)>> = vec![];
            for a in arg_str.iter() {
                let scope_vec_str = list_to_scope_ids(a);
                match set.construct_scope(&scope_vec_str) {
                    Ok(sc) => args.push(sc),
                    Err(e) => return Err(e),
                }
            }
            Ok(XGroup::new(args, Box::new(cc)))
        }

        pub fn new(
            args: Vec<Vec<(String, &'a XDomainInteger)>>,
            template: Box<XConstraintType<'a>>,
        ) -> Self {
            Self {
                args,
                template,
                scope_vec_str: vec![],
                scope_vec_var: vec![],
            }
        }
    }
}
