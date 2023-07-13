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
 * <p>@time: 9:57
 * </p>
 * <p>@this_file_name:xcsp3skeleton
 * </p>
 **/
#[allow(dead_code)]
pub mod xcsp3_core {
    use crate::xcsp3constraint::xcsp3_core::XConstraintSet;
    use crate::xcsp3domain::xcsp3_core::XDomainInteger;
    use crate::xcsp3variable::xcsp3_core::{XVariableSet, XVariableType};
    use quick_xml::de::from_str;
    use quick_xml::DeError;
    use serde::Deserialize;
    use std::fs;
    use std::time::Instant;

    #[derive(Deserialize, Debug)]
    pub struct VariableDomain {
        #[serde(rename = "@for")]
        pub r#for: String,
        #[serde(rename = "$value")]
        pub value: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct VariableArray {
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@type", default)]
        pub type_: String,
        #[serde(rename = "@startIndex", default)]
        pub start_index: String,
        #[serde(rename = "@note", default)]
        pub note: String,
        #[serde(rename = "@size", default)]
        pub size: String,
        #[serde(rename = "$value", default)]
        pub value: String,
        #[serde(rename = "domain", default)]
        pub domains: Vec<VariableDomain>,
    }

    #[derive(Deserialize, Debug)]
    pub struct VariableVar {
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@type", default)]
        pub type_: String,
        #[serde(rename = "$value", default)]
        pub value: String,
        #[serde(rename = "@as", default)]
        pub r#as: String,
    }

    #[derive(Deserialize, Debug)]
    pub enum VariableType {
        #[serde(rename = "var")]
        Var(VariableVar),
        #[serde(rename = "array")]
        Array(VariableArray),
    }

    #[derive(Deserialize, Debug)]
    pub struct Variable {
        #[serde(rename = "$value", default)]
        pub variables: Vec<VariableType>,
    }

    #[derive(Deserialize, Debug)]
    pub struct ConstraintGroup {
        #[serde(rename = "@class")]
        pub class: Option<String>,
        #[serde(rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "$value", default)]
        pub constraints: Vec<ConstraintType>,
        #[serde(rename = "args", default)]
        pub args: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct ConstraintBlock {
        #[serde(rename = "@id", default)]
        pub id: String,
        #[serde(rename = "@note", default)]
        pub note: String,
        #[serde(rename = "@class", default)]
        pub r#type: Option<String>,
        #[serde(rename = "$value", default)]
        pub constraints: Vec<ConstraintType>,
    }

    #[derive(Deserialize, Debug)]
    pub enum ConstraintType {
        // #[serde(rename = "metaConstraint")]
        // eta_constraint(ConstraintMetaConstraint),
        #[serde(rename = "group")]
        Group(ConstraintGroup),
        #[serde(rename = "block")]
        Block(ConstraintBlock),
        #[serde(rename = "allDifferent")]
        AllDifferent {
            #[serde(rename = "$value", default)]
            vars: String,
            #[serde(rename = "list", default)]
            list: Vec<String>,
            #[serde(rename = "except", default)]
            except: String,
            #[serde(rename = "matrix", default)]
            matrix: String,
        },
        #[serde(rename = "allEqual")]
        AllEqual {
            #[serde(rename = "$value")]
            vars: String,
        },
        #[serde(rename = "circuit")]
        Circuit {
            #[serde(rename = "$value")]
            vars: String,
        },
        #[serde(rename = "ordered")]
        Ordered {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "operator", default)]
            operator: String,
            #[serde(rename = "lengths", default)]
            lengths: String,
        },

        #[serde(rename = "intension")]
        Intension {
            #[serde(rename = "$value")]
            value: String,
        },
        #[serde(rename = "extension")]
        Extension {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "supports", default)]
            supports: String,
            #[serde(rename = "conflicts", default)]
            conflicts: String,
        },
        #[serde(rename = "regular")]
        Regular {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "transitions", default)]
            transitions: String,
            #[serde(rename = "start", default)]
            start: String,
            #[serde(rename = "final", default)]
            r#final: String,
        },
        #[serde(rename = "mdd")]
        Mdd {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "transitions", default)]
            transitions: String,
        },
        #[serde(rename = "sum")]
        Sum {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "coeffs", default)]
            coeffs: String,
        },
        #[serde(rename = "count")]
        Count {
            #[serde(rename = "@id", default)]
            id: String,
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "nValues")]
        NValues {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "except", default)]
            except: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "cardinality")]
        Cardinality {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
            #[serde(rename = "occurs", default)]
            occurs: String,
        },
        #[serde(rename = "minimum")]
        Minimum {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "maximum")]
        Maximum {
            #[serde(rename = "list", default)]
            list: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "element")]
        Element {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            value: String,
            #[serde(rename = "index", default)]
            index: String,
        },
        #[serde(rename = "stretch")]
        Stretch {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
            #[serde(rename = "widths", default)]
            widths: String,
            #[serde(rename = "patterns", default)]
            patterns: String,
        },
        #[serde(rename = "noOverlap")]
        NoOverlap {
            #[serde(rename = "origins", default)]
            origins: String,
            #[serde(rename = "lengths", default)]
            lengths: String,
        },
        #[serde(rename = "cumulative")]
        Cumulative {
            #[serde(rename = "origins", default)]
            origins: String,
            #[serde(rename = "lengths", default)]
            lengths: String,
            #[serde(rename = "heights", default)]
            heights: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "ends", default)]
            ends: String,
            #[serde(rename = "machines", default)]
            machines: String,
        },
        #[serde(rename = "instantiation")]
        Instantiation {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
        },
        #[serde(rename = "slide")]
        Slide {
            #[serde(rename = "@id", default)]
            id: String,
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "$value", default)]
            constraints: Vec<ConstraintType>,
        },
        #[serde(rename = "channel")]
        Channel {
            #[serde(rename = "list", default)]
            vars: Vec<String>,
            #[serde(rename = "value", default)]
            value: String,
        },
        #[serde(rename = "allDistant")]
        AllDistant {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "precedence")]
        Precedence {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
        },
        #[serde(rename = "balance")]
        Balance {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "values", default)]
            values: String,
        },
        #[serde(rename = "spread")]
        Spread {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "total", default)]
            total: String,
        },
        #[serde(rename = "deviation")]
        Deviation {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "total", default)]
            total: String,
        },

        #[serde(rename = "binPacking")]
        BinPacking {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "sizes", default)]
            sizes: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "lex")]
        Lex {
            #[serde(rename = "matrix", default)]
            matrix: String,
            #[serde(rename = "operator", default)]
            operator: String,
        },
        #[serde(rename = "clause")]
        Clause {
            #[serde(rename = "$value", default)]
            vars: String,
            #[serde(rename = "list", default)]
            list: Vec<String>,
        },
    }

    #[derive(Deserialize, Debug)]
    pub struct Constraint {
        // #[serde(rename = "constraint", default)]
        #[serde(rename = "$value", default)]
        pub constraints: Vec<ConstraintType>,
    }

    #[derive(Deserialize, Debug)]
    pub enum ObjectiveTarget {
        #[serde(rename = "sum")]
        Sum,
        #[serde(rename = "product")]
        Product,
        #[serde(rename = "minimum")]
        Minimum,
        #[serde(rename = "maximum")]
        Maximum,
        #[serde(rename = "nvalues")]
        Nvalues,
        #[serde(rename = "lex")]
        Lex,
    }

    #[derive(Deserialize, Debug)]
    pub struct MaximizeMinimize {
        #[serde(rename = "id", default)]
        pub id: String,
        #[serde(rename = "type")]
        pub r#type: Option<ObjectiveTarget>,
        #[serde(rename = "list", default)]
        pub list: String,
        #[serde(rename = "coeffs", default)]
        pub coeffs: String,
        #[serde(rename = "$value", default)]
        pub value: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Objective {
        #[serde(rename = "minimize", default)]
        pub minimize: Vec<MaximizeMinimize>,
        #[serde(rename = "maximize", default)]
        pub maximize: Vec<MaximizeMinimize>,
    }

    #[derive(Deserialize, Debug)]
    pub enum InstanceType {
        CSP,
        COP,
    }

    /// the instance of XCSP3
    #[derive(Deserialize, Debug)]
    pub struct Instance {
        #[serde(rename = "@format")]
        format: String,

        #[serde(rename = "@type")]
        r#type: InstanceType,
        variables: Option<Variable>,
        constraints: Constraint,
        objectives: Option<Objective>,
    }

    impl Instance {
        /// read the instance from the xml file
        pub fn from_path(path: &str) -> Result<Instance, DeError> {
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
                            let ret = XDomainInteger::from_string(&var_string.value);
                            if let Ok(domain) = ret {
                                variables.build_variable_int(&var_string.id, domain);
                            } else if let Err(e) = ret {
                                eprintln!("[{:?}] {}", &var_string, e.to_string());
                            }
                        } else {
                            match variables.find_variable_int(&var_string.r#as) {
                                Ok(v) => {
                                    if let XVariableType::XVariableInt(vv) = v {
                                        variables
                                            .build_variable_int(&var_string.id, vv.clone_domain());
                                    }
                                }
                                Err(e) => {
                                    eprintln!("[{:?}] {}", &var_string, e.to_string());
                                }
                            }
                        }
                    }
                    VariableType::Array(var_array_str) => {
                        // println!("var_array {:?}", var_array)
                        if var_array_str.domains.is_empty() {
                            match XDomainInteger::from_string(&var_array_str.value) {
                                Ok(domain) => {
                                    variables.build_variable_array(
                                        &var_array_str.id,
                                        &var_array_str.size,
                                        domain,
                                    );
                                }
                                Err(e) => {
                                    eprintln!("[{:?}] {}", var_array_str, e.to_string());
                                }
                            };
                        } else {
                            for dom in var_array_str.domains.iter() {
                                let ret = XDomainInteger::from_string(&dom.value);
                                if let Ok(_domain) = ret {
                                    // println!("var_array domain: {}", domain.to_string())
                                } else if let Err(e) = ret {
                                    eprintln!("[{:?}] {}", &var_array_str, e.to_string());
                                }
                            }
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
                        vars,
                        list: _,
                        except: _,
                        matrix: _,
                    } => {
                        println!("{}", vars)
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
