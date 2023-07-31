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
 * <p>@date: 2023/6/30
 * </p>
 * <p>@time: 13:44
 * </p>
 * <p>@this_file_name:XCSP3Constants
 * </p>
 */

// pub mod xcsp3_core {
//     use crate::constraints::xconstraint_set::xcsp3_core::XConstraintSet;
//     use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
//     use crate::objectives::xobjectives_set::xcsp3_core::XObjectivesSet;
//     use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
//     use crate::xcsp_xml::xcsp_xml_model::xcsp3_xml::{InstanceType, XcspXmlModel};
//
//     pub struct XCSP3<'a> {
//         model: XcspXmlModel,
//         variables: XVariableSet,
//         constraints: Option<XConstraintSet<'a>>,
//         objectives: Option<XObjectivesSet<'a>>,
//     }
//
//     // impl Drop for XCSP3<'_> {
//     //     fn drop(&mut self) {
//     //         drop(&self.objectives);
//     //         drop(&self.constraints);
//     //         drop(&self.variables);
//     //         drop(&self.model);
//     //     }
//     // }
//
//     impl<'a> XCSP3<'a> {
//         pub fn get_variables(&'a self) -> &'a XVariableSet {
//             &self.variables
//         }
//
//         pub fn from_path(xml_file: &str) -> Result<XCSP3<'a>, Xcsp3Error> {
//             match XcspXmlModel::from_path(xml_file) {
//                 Ok(model) => {
//                     let variable = model.build_variables();
//
//                     let mut a = XCSP3::new(model, variable);
//                     a.constraints = Some(a.model.build_constraints(&a.variables));
//                     // let obj = if let InstanceType::Cop = model.get_instance_type()
//                     // {
//                     //     Some(model.build_objectives(&variable))
//                     // } else {
//                     //     None
//                     // };
//                     Ok(a)
//                 }
//                 Err(e) => Err(Xcsp3Error::get_read_xml_error(e)),
//             }
//         }
//
//         pub fn new(
//             model: XcspXmlModel,
//             variables: XVariableSet,
//             // constraints: XConstraintSet<'a>,
//             // objectives: Option<XObjectivesSet<'a>>
//         ) -> XCSP3<'a> {
//             XCSP3 {
//                 model,
//                 variables,
//                 constraints: None,
//                 objectives: None,
//             }
//         }
//     }
// }