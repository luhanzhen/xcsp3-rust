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
 * <p>@project_name: XCSP3-Rust
 * </p>
 * <p>@author: luhanzhen
 * </p>
 * <p>@date: 2023/7/7
 * </p>
 * <p>@time: 18:35
 * </p>
 * <p>@this_file_name:xcsp3constraint
 * </p>
 */

pub mod xcsp3_core {
    use crate::constraints::xall_different::xcsp3_core::XAllDifferent;
    use crate::constraints::xall_different_except::xcsp3_core::XAllDifferentExcept;
    use crate::constraints::xall_equal::xcsp3_core::XAllEqual;
    use crate::constraints::xconstraint_type::xcsp3_core::XConstraintType;
    use crate::constraints::xextension::xcsp3_core::XExtension;
    use crate::constraints::xgroup::xcsp3_core::XGroup;
    use crate::constraints::xinstantiation::xcsp3_core::XInstantiation;
    use crate::constraints::xint_val_var::xcsp3_core::XVarVal;
    use crate::constraints::xintension::xcsp3_core::XIntention;
    use crate::constraints::xmax_min::xcsp3_core::XMaxMin;
    use crate::constraints::xmdd::xcsp3_core::XMdd;
    use crate::constraints::xordered::xcsp3_core::XOrdered;
    use crate::constraints::xregular::xcsp3_core::XRegular;
    use crate::constraints::xsum::xcsp3_core::XSum;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_matrix_ids;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::slice::Iter;

    /**
    the XConstraintSet is a container that stores all constraints.
     */
    pub struct XConstraintSet<'a> {
        constraints: Vec<XConstraintType<'a>>,
        set: &'a XVariableSet,
    }

    impl<'a> XConstraintSet<'a> {
        pub fn new(set: &'a XVariableSet) -> XConstraintSet<'a> {
            XConstraintSet {
                constraints: vec![],
                set,
            }
        }

        pub fn build_group(&mut self, cc: XConstraintType<'a>, args: &[String]) {
            // if let XConstraintType::XGroup(_) = &cc {
            //     // println!("group is in {}",c.to_string());
            //     self.constraints.push(XConstraintType::XConstraintNone(
            //         Xcsp3Error::get_constraint_group_error("the group is in group"),
            //     ))
            // }
            match XGroup::from_str(cc, args, self.set) {
                Ok(c) => {
                    self.constraints.push(XConstraintType::XGroup(c));
                }
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
            }
        }

        /// this function is designed for XGroup, parse the template for XGroup
        pub fn get_last_constraint(&mut self) -> Option<XConstraintType<'a>> {
            self.constraints.pop()
        }

        pub fn iter(&self) -> Iter<'_, XConstraintType> {
            self.constraints.iter()
        }

        pub fn build_minimum(&mut self, vars: &str, condition: &str) {
            match XMaxMin::from_str(vars, condition, true, self.set) {
                Ok(c) => {
                    self.constraints.push(XConstraintType::XMinimum(c));
                }
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
            }
        }
        pub fn build_maximum(&mut self, vars: &str, condition: &str) {
            match XMaxMin::from_str(vars, condition, false, self.set) {
                Ok(c) => {
                    self.constraints.push(XConstraintType::XMaximum(c));
                }
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
            }
        }

        pub fn build_sum(&mut self, vars: &str, condition: &str, coeffs: &str) {
            match XSum::from_str(vars, condition, coeffs, self.set) {
                Ok(c) => {
                    self.constraints.push(XConstraintType::XSum(c));
                }
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
            }
        }

        pub fn build_intention(&mut self, function: &str) {
            match XIntention::from_str_without_scope(function, self.set) {
                Ok(c) => {
                    self.constraints.push(XConstraintType::XIntention(c));
                }
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
            }
        }

        pub fn build_regular(
            &mut self,
            list: &str,
            transitions_str: &str,
            start_str: &str,
            final_str: &str,
        ) {
            match XRegular::from_str(list, transitions_str, start_str, final_str, self.set) {
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                Ok(c) => {
                    self.constraints.push(XConstraintType::XRegular(c));
                }
            }
        }

        pub fn build_mdd(&mut self, list: &str, transitions_str: &str) {
            match XMdd::from_str(list, transitions_str, self.set) {
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                Ok(c) => {
                    self.constraints.push(XConstraintType::XMdd(c));
                }
            }
        }

        pub fn build_ordered(&mut self, list: &str, lengths_str: &str, operator: &str) {
            if lengths_str.is_empty() {
                match XOrdered::from_str_without_lengths(list, operator, self.set) {
                    Ok(c) => {
                        self.constraints.push(XConstraintType::XOrdered(c));
                    }
                    Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                }
            } else {
                match XOrdered::from_str(list, lengths_str, operator, self.set) {
                    Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                    Ok(c) => {
                        self.constraints.push(XConstraintType::XOrdered(c));
                    }
                }
            }
        }

        pub fn build_instantiation(&mut self, list: &str, values: &str) {
            match XInstantiation::from_str(list, values, self.set) {
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                Ok(c) => {
                    self.constraints.push(XConstraintType::XInstantiation(c));
                }
            }
        }

        pub fn build_extension(&mut self, list: &str, tuple: &str, is_support: bool) {
            match XExtension::from_str(list, tuple, is_support, self.set) {
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                Ok(c) => {
                    self.constraints.push(XConstraintType::XExtension(c));
                }
            }
        }

        pub fn build_all_equal(&mut self, list: &str) {
            match XAllEqual::from_str(list, self.set) {
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                Ok(c) => {
                    self.constraints.push(XConstraintType::XAllEqual(c));
                }
            }
        }

        pub fn build_all_different(&mut self, list: &str) {
            match XAllDifferent::from_str(list, self.set) {
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                Ok(c) => {
                    self.constraints.push(XConstraintType::XAllDifferent(c));
                }
            }
        }

        pub fn build_all_different_except(&mut self, list: &str, except: &str) {
            match XAllDifferentExcept::from_str(list, except, self.set) {
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                Ok(c) => {
                    self.constraints
                        .push(XConstraintType::XAllDifferentExcept(c));
                }
            }
        }
        pub fn build_all_different_matrix(&mut self, list: &str) {
            let mat = list_to_matrix_ids(list);
            for line in mat.iter() {
                let mut scope: Vec<XVarVal> = vec![];
                for e in line {
                    scope.push(XVarVal::IntVar(e.clone()))
                }
                self.constraints
                    .push(XConstraintType::XAllDifferent(XAllDifferent::from_str_vec(
                        scope, self.set,
                    )))
            }
            for i in 0..mat[0].len() {
                let mut scope: Vec<XVarVal> = vec![];
                for m in mat.iter() {
                    scope.push(XVarVal::IntVar(m[i].clone()))
                }
                self.constraints
                    .push(XConstraintType::XAllDifferent(XAllDifferent::from_str_vec(
                        scope, self.set,
                    )))
            }
        }
    }
}
