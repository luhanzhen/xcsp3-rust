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
    use crate::constraints::xconstraint_type::xcsp3_core::XConstraintType;
    use crate::constraints::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::time_interval::xcsp3_utils::TimeInterval;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    #[derive(Clone)]
    pub struct XGroup<'a> {
        args: Vec<Vec<XVarVal>>,
        map: Vec<HashMap<String, &'a XDomainInteger>>,
        set: &'a XVariableSet,
        template: Box<XConstraintType<'a>>,
    }

    impl Display for XGroup<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret = String::default();
            for a in self.args.iter() {
                ret.push('[');
                for e in a.iter() {
                    ret.push('(');
                    ret.push_str(e.to_string().as_str());
                    ret.push_str("), ")
                }
                ret.push(']');
            }
            ret.push(']');
            write!(
                f,
                "XGroup: [constraint = {} [  {} ]] [args = ",
                &self.template.to_string(),
                ret
            )
        }
    }

    // impl XConstraintTrait for XGroup<'_> {
    //
    //
    //     fn get_scope_string(&self) -> &Vec<XVarVal>
    //     {
    //         &self.scope
    //     }
    //
    //     fn get_scope(&mut self) -> Vec<(&String, &XDomainInteger)> {
    //         for e in self.scope
    //         {
    //             if let XVarVal::IntVar(s) = e
    //             {
    //                 if !self.map.contains_key(&s)
    //                 {
    //                     if let Ok(vec) = self.set.construct_scope(&vec![s])
    //                     {
    //                         for (vs,vv) in vec.into_iter()
    //                         {
    //                             self.map.insert(vs, vv);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         let mut scope_vec_var: Vec<(&String, & XDomainInteger)>= vec![];
    //         for e in self.map.iter()
    //         {
    //             scope_vec_var.push((e.0,e.1))
    //         }
    //         scope_vec_var
    //     }
    // }

    impl<'a> XGroup<'a> {
        pub fn get_args(&self) -> &Vec<Vec<XVarVal>> {
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
            let tt = TimeInterval::new();
            let mut args: Vec<Vec<XVarVal>> = vec![];
            let mut map: Vec<HashMap<String, &'a XDomainInteger>> = vec![];
            args.reserve(arg_str.len());
            map.reserve(arg_str.len());
            for a in arg_str.iter() {
                match list_to_vec_var_val(a) {
                    Ok(scope_vec_str) => {
                        args.push(scope_vec_str);
                        // map.push(HashMap::default())
                    }
                    Err(e) => return Err(e),
                }
            }
            println!("this group cost {:?}", tt.get());
            Ok(XGroup::new(args, map, set, Box::new(cc)))
        }

        pub fn new(
            args: Vec<Vec<XVarVal>>,
            map: Vec<HashMap<String, &'a XDomainInteger>>,
            set: &'a XVariableSet,
            template: Box<XConstraintType<'a>>,
        ) -> Self {
            Self {
                args,
                map,
                set,
                template,
            }
        }
    }
}
