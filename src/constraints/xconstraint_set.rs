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
    use crate::constraints::xinstantiation::xcsp3_core::XInstantiation;
    use crate::constraints::xmdd::xcsp3_core::XMdd;
    use crate::constraints::xordered::xcsp3_core::XOrdered;
    use crate::constraints::xregular::xcsp3_core::XRegular;
    use crate::utils::xcsp3utils::xcsp3_core::list_to_matrix_ids;
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
        pub fn iter(&self) -> Iter<'_, XConstraintType> {
            self.constraints.iter()
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
            match XMdd::from_str(list, transitions_str) {
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                Ok(c) => {
                    self.constraints.push(XConstraintType::XMdd(c));
                }
            }
        }

        pub fn build_ordered(&mut self, list: &str, lengths_str: &str, operator: &str) {
            if lengths_str.is_empty() {
                self.constraints.push(XConstraintType::XOrdered(
                    XOrdered::from_str_without_lengths(list, operator).unwrap(),
                ));
            } else {
                match XOrdered::from_str(list, lengths_str, operator) {
                    Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                    Ok(c) => {
                        self.constraints.push(XConstraintType::XOrdered(c));
                    }
                }
            }
        }

        pub fn build_instantiation(&mut self, list: &str, values: &str) {
            match XInstantiation::from_str(list, values) {
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                Ok(c) => {
                    self.constraints.push(XConstraintType::XInstantiation(c));
                }
            }
        }

        pub fn build_extension(&mut self, list: &str, tuple: &str, is_support: bool) {
            match XExtension::from_str(list, tuple, is_support) {
                Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                Ok(c) => {
                    self.constraints.push(XConstraintType::XExtension(c));
                }
            }
        }

        pub fn build_all_equal(&mut self, list: &str) {
            match XAllEqual::from_str(list) {
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
            match XAllDifferentExcept::from_str(list, except) {
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
                match XAllDifferent::from_str_vec(line.clone(), self.set) {
                    Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                    Ok(c) => self.constraints.push(XConstraintType::XAllDifferent(c)),
                }
            }
            for i in 0..mat[0].len() {
                let mut column: Vec<String> = vec![];
                for m in mat.iter() {
                    column.push(m[i].clone());
                }
                match XAllDifferent::from_str_vec(column, self.set) {
                    Err(e) => self.constraints.push(XConstraintType::XConstraintNone(e)),
                    Ok(c) => self.constraints.push(XConstraintType::XAllDifferent(c)),
                }
            }
        }
    }
}
