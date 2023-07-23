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
 * <p>@time: 13:47
 * </p>
 * <p>@this_file_name:XCSP3Variable
 * </p>
 */

pub mod xcsp3_core {
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_type::xcsp3_core::XVariableType;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    // use regex::Regex;
    use std::slice::Iter;
    /**
    the XVariableSet is a container that store all variables.
     */
    pub struct XVariableSet {
        variables: Vec<XVariableType>,
        id_to_index: HashMap<String, usize>, //store the id and the index of the variable
                                             // empty_domain: XDomainInteger,
    }

    impl Default for XVariableSet {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Display for XVariableSet {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret = String::default();
            for e in self.variables.iter() {
                ret = format!("{} \t{}\n", ret, e);
            }
            write!(f, "XVariableSet: \n{}", ret)
        }
    }

    impl XVariableSet {
        pub fn iter(&self) -> Iter<'_, XVariableType> {
            self.variables.iter()
        }

        // pub fn to_string(&self) -> String {
        //     let mut ret = String::from("XVariableSet: \n");
        //     for e in self.variables.iter() {
        //         ret = format!("{} \t{}\n", ret, e);
        //     }
        //     ret
        // }
        pub fn new() -> XVariableSet {
            XVariableSet {
                variables: vec![],
                id_to_index: HashMap::default(),
                // empty_domain: XDomainInteger::new(),
            }
        }

        pub fn build_variable_int(&mut self, id: &str, domain_string: &str, symbolic: &String) {
            if symbolic.eq("symbolic") {
                let domain = XDomainInteger::from_symbolic(domain_string);
                let var = XVariableType::new_int(id, domain);
                self.id_to_index.insert(var.get_id(), self.variables.len());
                self.variables.push(var);
            } else {
                match XDomainInteger::from_string(domain_string) {
                    Ok(domain) => {
                        let var = XVariableType::new_int(id, domain);
                        self.id_to_index.insert(var.get_id(), self.variables.len());
                        self.variables.push(var);
                    }
                    Err(e) => {
                        // eprintln!("{}", e.to_string());
                        self.variables.push(XVariableType::XVariableNone(e));
                    }
                }
            }
        }

        pub fn build_variable_int_as(&mut self, id: &str, as_str: &str) {
            match self.find_variable(as_str) {
                Ok(v) => {
                    if let XVariableType::XVariableInt(vv) = v {
                        let var = XVariableType::new_int(id, vv.domain.clone());
                        self.id_to_index.insert(var.get_id(), self.variables.len());
                        self.variables.push(var);
                    }
                }
                Err(e) => {
                    // eprintln!("{}", e.to_string());
                    self.variables.push(XVariableType::XVariableNone(e));
                }
            }
        }

        pub fn build_variable_array(&mut self, id: &str, sizes: &str, domain_string: &str) {
            match XDomainInteger::from_string(domain_string) {
                Ok(domain) => {
                    let array = XVariableType::new_array(id, sizes, domain);
                    match array {
                        XVariableType::XVariableArray(_) => {
                            self.id_to_index
                                .insert(array.get_id(), self.variables.len());
                            self.variables.push(array);
                        }
                        _ => {
                            self.variables.push(XVariableType::XVariableNone(
                                Xcsp3Error::get_variable_size_invalid_error(""),
                            ));
                        }
                    }
                }
                Err(e) => {
                    self.variables.push(XVariableType::XVariableNone(e));
                }
            };
        }

        pub fn build_variable_tree(
            &mut self,
            id: &str,
            sizes: &str,
            domain_for: Vec<&String>,
            domain_value: Vec<&String>,
        ) {
            let tree = XVariableType::new_tree(id, sizes, domain_for, domain_value);
            match tree {
                Ok(tree) => {
                    self.id_to_index.insert(tree.get_id(), self.variables.len());
                    self.variables.push(tree);
                }
                Err(e) => {
                    self.variables.push(XVariableType::XVariableNone(e));
                }
            }
        }

        pub fn find_variable(&self, id: &str) -> Result<&XVariableType, Xcsp3Error> {
            let name = match id.find('[') {
                None => id,
                Some(n) => &id[..n],
            };
            match self.id_to_index.get(name) {
                None => Err(Xcsp3Error::get_variable_not_found_error(
                    &("not find the variable named ".to_owned() + name),
                )),
                Some(v) => Ok(&self.variables[*v]),
            }
        }

        // pub fn exist_variables(&self, scope_str: &[String]) -> bool {
        //     for e in scope_str.iter() {
        //         if e.contains('%') {
        //             continue;
        //         } else {
        //             match self.find_variable(e) {
        //                 Ok(v) => match v {
        //                     XVariableType::XVariableNone(_) => return false,
        //                     XVariableType::XVariableArray(_) => {}
        //                     XVariableType::XVariableInt(_) => continue,
        //                     XVariableType::XVariableTree(_) => {}
        //                 },
        //                 Err(_) => return false,
        //             }
        //         }
        //     }
        //     true
        // }

        ///construct the scope from XVariableSet, when scope is equal to %x, where x is an i32 number, return empty tuple
        pub fn construct_scope(
            &self,
            scope_str: &[&String],
        ) -> Result<Vec<(String, &XDomainInteger)>, Xcsp3Error> {
            let mut ret: Vec<(String, &XDomainInteger)> = vec![];
            for e in scope_str.iter() {
                {
                    match self.find_variable(e) {
                        Ok(var_type) => match var_type {
                            XVariableType::XVariableArray(a) => match a.find_variable(e) {
                                Ok(mut vec) => {
                                    for (e1, e2) in vec.iter_mut() {
                                        ret.push((e1.to_string(), e2));
                                    }
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            },
                            XVariableType::XVariableInt(i) => ret.push((i.id.clone(), &i.domain)),
                            XVariableType::XVariableTree(t) => match t.find_variable(e) {
                                Ok(mut vec) => {
                                    for (e1, e2) in vec.iter_mut() {
                                        ret.push((e1.to_string(), e2));
                                    }
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            },
                            _ => {}
                        },
                        Err(_) => {
                            return Err(Xcsp3Error::get_variable_not_found_error(
                                "the scope not found, ",
                            ));
                        }
                    }
                }
            }
            Ok(ret)
        }
    }
}
