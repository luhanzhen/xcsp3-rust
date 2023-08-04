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
 * <p>@date:  2023/7/29 16:45
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
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    pub struct XSlide<'a> {
        args: Vec<XVarVal>,
        map: HashMap<String, &'a XDomainInteger>,
        set: &'a XVariableSet,
        template: Box<XConstraintType<'a>>,
        circular: bool,
        offset: i32,
    }

    impl<'a> XSlide<'a> {
        pub fn get_offset(&self) -> i32 {
            self.offset
        }
        pub fn get_circular(&self) -> bool {
            self.circular
        }
        pub fn get_scope(&mut self) -> Vec<(&String, &XDomainInteger)> {
            for e in &self.args {
                if let XVarVal::IntVar(s) = e {
                    if !self.map.contains_key(s) {
                        if let Ok(vec) = self.set.construct_scope(&[s]) {
                            for (vs, vv) in vec.into_iter() {
                                self.map.insert(vs, vv);
                            }
                        }
                    }
                }
            }
            let mut scope_vec_var: Vec<(&String, &XDomainInteger)> = vec![];
            for e in self.map.iter() {
                scope_vec_var.push((e.0, e.1))
            }
            scope_vec_var
        }

        pub fn get_args(&self) -> &Vec<XVarVal> {
            &self.args
        }

        pub fn get_template(&self) -> &XConstraintType<'a> {
            &self.template
        }

        pub fn from_str(
            cc: XConstraintType<'a>,
            arg_str: &str,
            offset_str: &str,
            circular_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            match list_to_vec_var_val(arg_str) {
                Ok(scope_vec_str) => {
                    let offset = if offset_str.is_empty() {
                        0
                    } else {
                        match offset_str.parse::<i32>() {
                            Ok(o) => o,
                            Err(_) => {
                                return Err(Xcsp3Error::get_constraint_slide_error(
                                    "parse the offset error, ",
                                ));
                            }
                        }
                    };
                    let circular = if circular_str.is_empty() {
                        false
                    } else {
                        match circular_str.parse::<bool>() {
                            Ok(c) => c,
                            Err(_) => {
                                return Err(Xcsp3Error::get_constraint_slide_error(
                                    "parse the circular error, ",
                                ))
                            }
                        }
                    };
                    Ok(Self::new(
                        scope_vec_str,
                        set,
                        offset,
                        circular,
                        Box::new(cc),
                    ))
                }
                Err(e) => Err(e),
            }

            // match list_to_vec_var_val(arg_str) {
            //     Ok(scope_vec_str) => match offset_str.parse::<i32>() {
            //         Ok(offset) => match circular_str.parse::<bool>() {
            //             Ok(circular) => Ok(Self::new(
            //                 scope_vec_str,
            //                 set,
            //                 offset,
            //                 circular,
            //                 Box::new(cc),
            //             )),
            //             Err(_) => Err(Xcsp3Error::get_constraint_slide_error(
            //                 "parse the circular error, ",
            //             )),
            //         },
            //         Err(_) => Err(Xcsp3Error::get_constraint_slide_error(
            //             "parse the offset error, ",
            //         )),
            //     },
            //     Err(e) => Err(e),
            // }
        }

        pub fn new(
            args: Vec<XVarVal>,
            set: &'a XVariableSet,
            offset: i32,
            circular: bool,
            template: Box<XConstraintType<'a>>,
        ) -> Self {
            Self {
                args,
                map: Default::default(),
                set,
                template,
                circular,
                offset,
            }
        }
    }

    impl Display for XSlide<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret = String::default();
            for a in self.args.iter() {
                ret.push_str(a.to_string().as_str());
                ret.push_str(", ")
            }
            write!(
                f,
                "XSlide: [constraint = {} [ list =  {}]]",
                &self.template.to_string(),
                ret
            )
        }
    }
}
