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
* <p>@date:  2023/7/14 19:08
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 */

/**
the SYNTAX of xcsp3 is as follows:
```xml
<instance format="XCSP<sup>3</sup>" type="frameworkType">
  <variables>
    ( <var .../>
    | <array .../>
    )+
  </variables>
  <constraints>
    ( <constraint .../>
    | <metaConstraint .../>
    | <group .../>
    | <block .../>
    )*
  </constraints>
  [<objectives  [ combination="combinationType" ]>
    ( <minimize .../>
    | <maximize .../>
    )+
  </objectives>]
  [<annotations .../>]
</instance>
```
 */
pub mod xcsp3_xml {
    use crate::constraints::xconstraint_set::xcsp3_core::XConstraintSet;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use crate::xcsp_xml::constraint::xcsp3_xml::Constraint;
    use crate::xcsp_xml::constraint_type::xcsp3_xml::ConstraintType;
    use crate::xcsp_xml::objective::xcsp3_xml::{InstanceType, Objective};
    use crate::xcsp_xml::variable::xcsp3_xml::Variable;
    use crate::xcsp_xml::variable_type::xcsp3_xml::VariableType;
    use quick_xml::de::from_str;
    use quick_xml::DeError;
    use serde::Deserialize;
    use std::fs;
    use std::time::Instant;

    /// the instance of XCSP3
    #[derive(Deserialize)]
    pub struct XcspXmlModel {
        #[serde(rename = "@format")]
        format: String,
        #[serde(rename = "@type")]
        r#type: InstanceType,
        variables: Option<Variable>,
        constraints: Constraint,
        objectives: Option<Objective>,
        // #[serde(skip)]
        // variables_set: XVariableSet,
        // #[serde(skip)]
        // constraints_set: XConstraintSet<'a>,
    }

    impl XcspXmlModel {
        // pub fn build(&self) -> (XVariableSet,XConstraintSet) {
        //     let (v,c):(XVariableSet,XConstraintSet);
        //     v = self.build_variables();
        //     c = self.build_constraints(&v);
        //     (v,c)
        // }

        pub fn build_objectives(&self) {
            println!("{:?}", self.objectives)
        }

        /// read the instance from the xml file
        pub fn from_path(path: &str) -> Result<XcspXmlModel, DeError> {
            // let now = Instant::now();
            if !path.ends_with(".xml") {
                return Err(DeError::UnexpectedEof);
            }
            let xml = fs::read_to_string(path).unwrap();
            let r = from_str(&xml);
            // println!("read the instance named {} by {:?}.", path, now.elapsed());
            r
        }

        /// read the instance from string
        pub fn from_str(string: &str) -> Result<XcspXmlModel, DeError> {
            let now = Instant::now();
            let r = from_str(string);
            println!(
                "read the instance by {} microseconds",
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
                            let mut domain_for: Vec<&String> = vec![];
                            let mut domain_value: Vec<&String> = vec![];
                            for e in var_array_str.domains.iter() {
                                domain_value.push(&e.value);
                                domain_for.push(&e.r#for);
                            }
                            variables.build_variable_tree(
                                &var_array_str.id,
                                &var_array_str.size,
                                domain_for,
                                domain_value,
                            );
                        }
                    }
                }
            }
            variables
        }

        fn parse_constraint(con_type: &ConstraintType, set: &mut XConstraintSet) {
            match con_type {
                ConstraintType::Group(group) => {
                    // println!("{:?}", group.constraints);
                    XcspXmlModel::parse_constraint(&group.constraints[0], set);

                    match set.get_last_constraint() {
                        None => {}
                        Some(cc) => {
                            // println!("{}",cc.to_string())
                            set.build_group(cc, &group.args);
                        }
                    }
                }
                ConstraintType::Block(block) => {
                    for e in block.constraints.iter() {
                        // println!("{:?}", e);
                        XcspXmlModel::parse_constraint(e, set);
                    }
                }
                ConstraintType::AllDifferent {
                    vars,
                    list,
                    except,
                    matrix,
                } => {
                    if !vars.is_empty() {
                        set.build_all_different(vars)
                    } else if matrix.is_empty() {
                        if !except.is_empty() {
                            for e in list.iter() {
                                set.build_all_different_except(e, except);
                            }
                        } else {
                            for e in list.iter() {
                                set.build_all_different(e);
                            }
                        }
                    } else {
                        set.build_all_different_matrix(matrix);
                    }
                }
                ConstraintType::AllEqual { vars, list } => {
                    if !vars.is_empty() {
                        set.build_all_equal(vars);
                    } else {
                        for e in list.iter() {
                            set.build_all_equal(e);
                        }
                    }
                }
                ConstraintType::Circuit { .. } => {}
                ConstraintType::Ordered {
                    vars,
                    operator,
                    lengths,
                } => set.build_ordered(vars, lengths, operator),
                ConstraintType::Intension { value, function } => {
                    if !value.is_empty() {
                        set.build_intention(value);
                    } else if !function.is_empty() {
                        set.build_intention(function);
                    }
                }
                ConstraintType::Extension {
                    vars,
                    supports,
                    conflicts,
                } => {
                    if supports.is_empty() {
                        set.build_extension(vars, conflicts, false)
                    } else if conflicts.is_empty() {
                        set.build_extension(vars, supports, true)
                    } else {
                        eprintln!("can't build extension, conflicts or supports must be non empty.")
                    }
                }
                ConstraintType::Regular {
                    vars,
                    transitions,
                    start,
                    r#final,
                } => set.build_regular(vars, transitions, start, r#final),
                ConstraintType::Mdd { vars, transitions } => set.build_mdd(vars, transitions),
                ConstraintType::Sum {
                    vars,
                    condition,
                    coeffs,
                } => set.build_sum(vars, condition, coeffs),
                ConstraintType::Count { .. } => {}
                ConstraintType::NValues { .. } => {}
                ConstraintType::Cardinality { .. } => {}
                ConstraintType::Minimum { list, condition } => set.build_minimum(list, condition),
                ConstraintType::Maximum { list, condition } => set.build_maximum(list, condition),
                ConstraintType::Element { .. } => {}
                ConstraintType::Stretch { .. } => {}
                ConstraintType::NoOverlap { .. } => {}
                ConstraintType::Cumulative { .. } => {}
                ConstraintType::Instantiation { vars, values } => {
                    // println!("{}{:?}", vars, values);
                    set.build_instantiation(vars, values);
                }
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

        pub fn build_constraints<'a>(&'a self, set: &'a XVariableSet) -> XConstraintSet {
            let mut constraint: XConstraintSet = XConstraintSet::new(set);
            for con_type in self.constraints.constraints.iter() {
                XcspXmlModel::parse_constraint(con_type, &mut constraint);
            }
            constraint
        }
    }
}
