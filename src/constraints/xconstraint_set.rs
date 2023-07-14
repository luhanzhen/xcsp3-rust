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
 **/

pub mod xcsp3_core {

    use crate::constraints::xall_different::xcsp3_core::XAllDifferent;
    use crate::constraints::xall_different_except::xcsp3_core::XAllDifferentExcept;
    use crate::constraints::xconstraint_type::xcsp3_core::XConstraintType;
    use crate::constraints::xextension::xcsp3_core::XExtension;
    use crate::utils::xcsp3utils::xcsp3_core::list_to_matrix_ids;
    use std::slice::Iter;

    pub struct XConstraintSet {
        constraints: Vec<XConstraintType>,
    }

    impl XConstraintSet {
        pub fn new() -> XConstraintSet {
            XConstraintSet {
                constraints: vec![],
            }
        }
        pub fn iter(&self) -> Iter<'_, XConstraintType> {
            self.constraints.iter()
        }

        pub fn build_extension(&mut self, list: &str, tuple: &str, is_support: bool) {
            match XExtension::from_str(list, tuple, is_support) {
                None => self.constraints.push(XConstraintType::XConstraintNone),
                Some(c) => {
                    self.constraints.push(XConstraintType::XExtension(c));
                }
            }
        }

        pub fn build_all_different(&mut self, list: &str) {
            match XAllDifferent::from_str(list) {
                None => self.constraints.push(XConstraintType::XConstraintNone),
                Some(c) => {
                    self.constraints.push(XConstraintType::XAllDifferent(c));
                }
            }
        }

        pub fn build_all_different_except(&mut self, list: &str, except: &str) {
            match XAllDifferentExcept::from_str(list, except) {
                None => self.constraints.push(XConstraintType::XConstraintNone),
                Some(c) => {
                    self.constraints
                        .push(XConstraintType::XAllDifferentExcept(c));
                }
            }
        }
        pub fn build_all_different_matrix(&mut self, list: &str) {
            let mat = list_to_matrix_ids(list);
            for line in mat.iter() {
                self.constraints
                    .push(XConstraintType::XAllDifferent(XAllDifferent::new(
                        line.clone(),
                    )))
            }
            for i in 0..mat[0].len() {
                let mut column: Vec<String> = vec![];
                for j in 0..mat.len() {
                    column.push(mat[j][i].clone());
                }
                self.constraints
                    .push(XConstraintType::XAllDifferent(XAllDifferent::new(column)))
            }
        }
    }

    impl Default for XConstraintSet {
        fn default() -> Self {
            Self::new()
        }
    }
}
