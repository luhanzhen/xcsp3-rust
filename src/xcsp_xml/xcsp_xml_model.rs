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
* <p>@date:  2023/7/14 19:08
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 **/

pub mod xcsp3_xml {
    use crate::constraints::xconstraint_set::xcsp3_core::XConstraintSet;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use crate::xcsp_xml::constraint::xcsp3_xml::Constraint;
    use crate::xcsp_xml::constraint_type::xcsp3_xml::ConstraintType;
    use crate::xcsp_xml::objective::xcsp3_xml::{InstanceType, Objective};
    use crate::xcsp_xml::variable::xcsp3_xml::{Variable, VariableType};
    use quick_xml::de::from_str;
    use quick_xml::DeError;
    use serde::Deserialize;
    use std::fs;
    use std::time::Instant;

    /// the instance of XCSP3
    #[derive(Deserialize, Debug)]
    pub struct XcspXmlModel {
        #[serde(rename = "@format")]
        format: String,

        #[serde(rename = "@type")]
        r#type: InstanceType,
        variables: Option<Variable>,
        constraints: Constraint,
        objectives: Option<Objective>,
    }

    impl XcspXmlModel {
        pub fn build_objectives(&self) {
            println!("{:?}", self.objectives)
        }
        /// read the instance from the xml file
        pub fn from_path(path: &str) -> Result<XcspXmlModel, DeError> {
            let now = Instant::now();
            if !path.ends_with(".xml") {
                return Err(DeError::UnexpectedEof);
            }
            let xml = fs::read_to_string(path).unwrap();
            let r = from_str(&xml);
            println!(
                "read the instance named {} by {} microseconds",
                path,
                now.elapsed().as_micros()
            );
            r
        }

        /// get the format of the instance: "XCSP3"
        pub fn get_format(&self) -> String {
            self.format.clone()
        }

        /// get the type of the instance:  COP or CSP
        pub fn get_instance_type(&self) -> &InstanceType {
            &self.r#type
        }

        /// build the variables
        pub fn build_variables(&self) -> XVariableSet {
            // let mut variables: Vec<XVariableType> = vec![];
            let mut variables: XVariableSet = XVariableSet::new();

            for var_type in self.variables.as_ref().unwrap().variables.iter() {
                match var_type {
                    VariableType::Var(var_string) => {
                        if var_string.r#as.is_empty() {
                            variables.build_variable_int(
                                &var_string.id,
                                &var_string.value,
                                &var_string.r#type,
                            );
                        } else {
                            variables.build_variable_int_as(&var_string.id, &var_string.r#as);
                        }
                    }
                    VariableType::Array(var_array_str) => {
                        // println!("var_array {:?}", var_array)
                        if var_array_str.domains.is_empty() {
                            variables.build_variable_array(
                                &var_array_str.id,
                                &var_array_str.size,
                                &var_array_str.value,
                            );
                        } else {
                            variables.build_variable_tree(
                                &var_array_str.id,
                                &var_array_str.size,
                                &var_array_str.domains,
                            );
                        }
                    }
                }
            }
            // println!("{}", variables.to_string());

            variables
        }

        pub fn build_constraints(&self) -> XConstraintSet {
            let mut constraint: XConstraintSet = XConstraintSet::new();
            for con_type in self.constraints.constraints.iter() {
                match con_type {
                    ConstraintType::Group(_) => {}
                    ConstraintType::Block(_) => {}
                    ConstraintType::AllDifferent {
                        vars: _,
                        list: _,
                        except: _,
                        matrix: _,
                    } => {
                        // println!("{}", vars)
                    }
                    ConstraintType::AllEqual { .. } => {}
                    ConstraintType::Circuit { .. } => {}
                    ConstraintType::Ordered { .. } => {}
                    ConstraintType::Intension { .. } => {}
                    ConstraintType::Extension {
                        vars,
                        supports,
                        conflicts,
                    } => {
                        if supports.is_empty() {
                            constraint.build_extension(vars, conflicts, false)
                        } else if conflicts.is_empty() {
                            constraint.build_extension(vars, supports, true)
                        } else {
                            eprintln!(
                                "can't build extension, conflicts or supports must be non empty."
                            )
                        }
                    }
                    ConstraintType::Regular { .. } => {}
                    ConstraintType::Mdd { .. } => {}
                    ConstraintType::Sum { .. } => {}
                    ConstraintType::Count { .. } => {}
                    ConstraintType::NValues { .. } => {}
                    ConstraintType::Cardinality { .. } => {}
                    ConstraintType::Minimum { .. } => {}
                    ConstraintType::Maximum { .. } => {}
                    ConstraintType::Element { .. } => {}
                    ConstraintType::Stretch { .. } => {}
                    ConstraintType::NoOverlap { .. } => {}
                    ConstraintType::Cumulative { .. } => {}
                    ConstraintType::Instantiation { .. } => {}
                    ConstraintType::Slide { .. } => {}
                    ConstraintType::Channel { .. } => {}
                    ConstraintType::AllDistant { .. } => {}
                    ConstraintType::Precedence { .. } => {}
                    ConstraintType::Balance { .. } => {}
                    ConstraintType::Spread { .. } => {}
                    ConstraintType::Deviation { .. } => {}
                    ConstraintType::BinPacking { .. } => {}
                    ConstraintType::Lex { .. } => {}
                    ConstraintType::Clause { .. } => {}
                }
            }
            constraint
        }
    }
}
