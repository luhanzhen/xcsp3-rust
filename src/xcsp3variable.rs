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
 * <p>@date: 2023/6/30
 * </p>
 * <p>@time: 13:47
 * </p>
 * <p>@this_file_name:XCSP3Variable
 * </p>
 **/
// #[allow(dead_code)]
pub mod xcsp3_core {
    use crate::xcsp3domain::xcsp3_core::*;
    use crate::xcsp3error::xcsp3_core::Xcsp3Error;
    use std::collections::HashMap;
    use std::slice::Iter;
    use std::str::FromStr;

    pub struct XVariableSet {
        variables: Vec<XVariableType>,
        id_to_index: HashMap<String, usize>, //store the id and the index of the variable
    }

    impl XVariableSet {
        pub fn iter(&self) -> Iter<'_, XVariableType> {
            return self.variables.iter();
        }

        pub fn to_string(&self) -> String {
            let mut ret = String::from("XVariableSet: \n");
            for e in self.variables.iter() {
                ret = format!("{} \t{}\n", ret, e.to_string());
            }
            ret
        }
        pub fn new() -> XVariableSet {
            XVariableSet {
                variables: vec![],
                id_to_index: HashMap::default(),
            }
        }

        pub fn build_variable_int(&mut self, id: &str, domain: XDomainInteger) {
            let var = XVariableType::new_int(id, domain);
            self.id_to_index.insert(var.get_id(), self.variables.len());
            self.variables.push(var);
        }

        pub fn build_variable_array(&mut self, id: &str, sizes: &str, domain: XDomainInteger) {
            let array = XVariableType::new_array(id, sizes, domain);
            self.id_to_index
                .insert(array.get_id(), self.variables.len());
            self.variables.push(array);
        }

        pub fn find_variable_int(&self, id: &str) -> Result<&XVariableType, Xcsp3Error> {
            match self.id_to_index.get(id) {
                None => {
                    Err(Xcsp3Error::get_variable_not_found_error("not find the variable, please visit http://xcsp.org/specifications/variables/integer/"))
                }
                Some(v) => {
                    Ok(&self.variables[*v])
                }
            }
        }
    }

    #[derive(Clone)]
    pub enum XVariableType {
        None,
        XVariableArray(XVariableArray),
        XVariableInt(XVariableInt),
        XVariableTree(XVariableTree),
    }

    impl XVariableType {
        pub fn new_int(id: &str, domain: XDomainInteger) -> XVariableType {
            XVariableType::XVariableInt(XVariableInt::new(id.to_string(), domain))
        }

        pub fn new_array(id: &str, sizes: &str, domain: XDomainInteger) -> XVariableType {
            if let Some(e) = XVariableArray::from_sizes_one_domain(id, sizes, domain) {
                XVariableType::XVariableArray(e)
            } else {
                XVariableType::None
            }
        }

        pub fn get_id(&self) -> String {
            match self {
                XVariableType::XVariableArray(v) => v.id.clone(),
                XVariableType::XVariableInt(v) => v.id.clone(),
                XVariableType::XVariableTree(v) => v.id.clone(),
                _ => String::default(),
            }
        }
        pub fn to_string(&self) -> String {
            match self {
                XVariableType::XVariableArray(a) => a.to_string(),
                XVariableType::XVariableInt(i) => i.to_string(),
                XVariableType::XVariableTree(t) => t.to_string(),
                _ => String::default(),
            }
        }
    }

    pub trait XVariableTrait {
        fn to_string(&self) -> String;
    }

    #[derive(Clone)]
    pub struct XVariableTree {
        domain: XDomainInteger,
        id: String,
    }

    impl XVariableTrait for XVariableTree {
        fn to_string(&self) -> String {

        }
    }

    #[derive(Clone)]
    pub struct XVariableInt {
        domain: XDomainInteger,
        id: String,
    }

    impl XVariableInt {
        pub fn new(id: String, domain: XDomainInteger) -> XVariableInt {
            XVariableInt { id, domain }
        }

        pub fn clone_domain(&self) -> XDomainInteger {
            self.domain.clone()
        }
    }

    impl XVariableTrait for XVariableInt {
        fn to_string(&self) -> String {
            format!(
                "XVariableInt: id = {}, domain = {}",
                self.id,
                self.domain.to_string()
            )
        }
    }

    #[derive(Clone)]
    pub struct XVariableArray {
        id: String,
        sizes: Vec<usize>,
        size: usize,
        domain: XDomainInteger,
    }

    impl XVariableTrait for XVariableArray {
        fn to_string(&self) -> String {
            let mut ret: String = String::from("XVariableArray: id = ");
            ret.push_str(self.id.as_str());
            ret.push_str("  size = ");
            // {}]: [, self.id);
            // for e in self.variables.iter() {
            //     ret.push_str(format!(" \t{}", e.to_string()).as_str());
            // }

            for e in self.sizes.iter() {
                ret.push_str("[");
                ret.push_str(e.to_string().as_str());
                ret.push_str("]");
            }
            ret.push_str(" domain = ");
            ret.push_str(self.domain.to_string().as_str());
            // println!("dim : {:?}", dim);
            // for i in 0..size {
            //     let mut iid = String::from(id);
            //     let mut remain = i;
            //
            //     for d in dim.iter() {
            //         // iid.push_str(format!("[{}]", remain / d).as_str());
            //         iid.push_str("[");
            //         iid.push_str((remain/d).to_string().as_str());
            //         iid.push_str("]");
            //         remain %= d;
            //     }
            // ret = format!("{}]", ret);
            ret
        }
    }

    impl XVariableArray {
        pub fn find_variable(&self, id: &str) -> Result<(String, &XDomainInteger), Xcsp3Error> {
            return if let Ok((size_vec, size)) = sizes_to_vec(id) {
                if size > self.size {
                    Err(Xcsp3Error::get_variable_size_invalid_error("parse the size of variable error, please visit http://xcsp.org/specifications/variables/arrays/"))
                } else {
                    Ok((id.to_string(), &self.domain))
                }
            } else {
                Err(Xcsp3Error::get_variable_size_invalid_error("parse the size of variable error, please visit http://xcsp.org/specifications/variables/arrays/"))
            };
        }

        pub fn from_sizes_one_domain(
            id: &str,
            sizes: &str,
            domain: XDomainInteger,
        ) -> Option<Self> {
            if let Ok((size_vec, size)) = sizes_to_vec(sizes) {
                Some(XVariableArray {
                    id: id.to_string(),
                    sizes: size_vec,
                    size,
                    domain,
                })
            } else {
                None
            }
        }
    }
    // #[derive(Clone)]
    // pub struct XVariableArray {
    //     variables: Vec<XVariableType>,
    //     id: String,
    //     id_to_index: HashMap<String, usize>, //store the id and the index of the variable
    // }

    /// transform the string size to vector sizes
    /// eg:  [2][3][4] -> ([2,3,4], 24)
    fn sizes_to_vec(sizes: &str) -> Result<(Vec<usize>, usize), Xcsp3Error> {
        let mut ret: Vec<usize> = vec![];
        let mut sz: usize = 1;
        // sizes.find()
        let sizes = sizes.replace("[", " ");
        let sizes = sizes.replace("]", " ");

        let nums: Vec<&str> = sizes.split_whitespace().collect();
        for n in nums.iter() {
            match usize::from_str(n) {
                Ok(v) => {
                    ret.push(v);
                    sz *= v;
                }
                Err(_) => {
                    return Err(Xcsp3Error::get_variable_size_invalid_error("parse the size of variable error, please visit http://xcsp.org/specifications/variables/integer/"));
                }
            };
        }

        Ok((ret, sz))
    }

    // impl XVariableArray {
    //     pub fn find_variable(&self, id: &str) -> Result<&XVariableType, Xcsp3Error> {
    //         match self.id_to_index.get(id) {
    //             None => {
    //                 Err(Xcsp3Error::get_variable_not_found_error("not find the variable, please visit http://xcsp.org/specifications/variables/integer/"))
    //             }
    //             Some(v) => {
    //                 Ok(&self.variables[*v])
    //             }
    //         }
    //     }
    //
    //     pub fn from_sizes_one_domain(
    //         id: &str,
    //         sizes: &str,
    //         domain: XDomainInteger,
    //     ) -> Option<Self> {
    //         if let Ok((size_vec, size)) = sizes_to_vec(sizes) {
    //             let mut vars: Vec<XVariableType> = vec![];
    //             let mut index_usize: HashMap<String, usize> = HashMap::new();
    //             let mut dim: Vec<usize> = vec![];
    //             let mut remain = size;
    //             for e in size_vec.iter() {
    //                 remain /= e;
    //                 dim.push(remain);
    //             }
    //             // println!("dim : {:?}", dim);
    //             for i in 0..size {
    //                 let mut iid = String::from(id);
    //                 let mut remain = i;
    //
    //                 for d in dim.iter() {
    //                     // iid.push_str(format!("[{}]", remain / d).as_str());
    //                     iid.push_str("[");
    //                     iid.push_str((remain/d).to_string().as_str());
    //                     iid.push_str("]");
    //                     remain %= d;
    //                 }
    //                 index_usize.insert(iid.clone(), i);
    //                 vars.push(XVariableType::XVariableInt(XVariableInt::new(
    //                     iid,
    //                     domain.clone(),
    //                 )));
    //                 // print!("  iid : {:?}", iid);
    //             }
    //             Some(XVariableArray {
    //                 id: id.to_string(),
    //                 variables: vars,
    //                 id_to_index: index_usize,
    //             })
    //         } else {
    //             None
    //         }
    //     }
    //
    //     pub fn to_string(&self) -> String {
    //         let mut ret = format!("XVariableArray[{}]: [", self.id);
    //         for e in self.variables.iter() {
    //             ret.push_str(format!(" \t{}", e.to_string()).as_str());
    //         }
    //         ret = format!("{}]", ret);
    //         ret
    //     }
    // }
}

// impl<'a> XVariableInt<'a> {
//     pub fn from(lid: String, dom: &'a XDomainInteger) -> Self {
//         XVariableInt {
//             id: lid,
//             classes: String::default(),
//             domain: Some(dom),
//             entity_type: EntityType::XVariableInt,
//         }
//     }
//
//     pub fn from_with_index(mut lid: String, dom: &'a XDomainInteger, indexes: &[i32]) -> Self {
//         for e in indexes.into_iter() {
//             lid = format!("{} [{}]", lid, e.to_string());
//         }
//         XVariableInt {
//             classes: String::default(),
//             domain: Some(dom),
//             id: lid,
//             entity_type: EntityType::XVariableInt,
//         }
//     }
// }

// use std::sync::atomic::{AtomicI32, Ordering};

// #[derive(Default)]
// pub struct XInterval {
//     min: i32,
//     max: i32,
// }
//
// impl XInterval {
//     pub fn new(min: i32, max: i32) -> XInterval {
//         XInterval { min, max }
//     }
//
//     // pub fn to_string(&self) -> String {
//     //     format!("[{},{}]", self.min, self.max)
//     // }
// }

// pub trait XEntityTrait {
//     fn fake(&self);
//     fn get_type(&self) -> &EntityType;
// }

// pub trait XVariableTrait {
//     fn to_string(&self) -> String;
//     fn get_domain(&self) -> &Option<&XDomainInteger>;
//     fn get_type(&self) -> &EntityType;
// }

// impl<'a> XVariableTrait for XVariableInt<'a> {
//     fn to_string(&self) -> String {
//         format!("XVariable {}:{}", self.id, self.classes)
//     }
//
//     fn get_domain(&self) -> &Option<&XDomainInteger> {
//         &self.domain
//     }
//     fn get_type(&self) -> &EntityType {
//         &self.entity_type
//     }
// }

// initialize a XVariableArray from a existing XVariableArray.
// pub fn from_array(lid: String, var_array: &XVariableArray) -> Self {
//     let mut index: Vec<i32> = vec![];
//     index.resize(var_array.sizes.len(), 0);
//     let mut variable: Vec<Box<dyn XVariableTrait>> = vec![];
//     for i in 0..var_array.variables.len() {
//         let var = XVariable::from_with_index(
//             lid.clone(),
//             var_array.variables[i].get_domain().unwrap(),
//             &index,
//         );
//         variable.push(Box::new(var));
//     }
//     XVariableArray {
//         id: lid,
//         classes: String::default(),
//         entity_type: EntityType::XVariableArray,
//         variables: variable,
//         sizes: index,
//     }
// }

// pub struct XInteger<'a> {
//     classes: String,
//     domain: Option<&'a XDomainInteger>,
//     value: i32,
//     id: String,
//     entity_type: EntityType,
// }
//
// impl<'a> XEntityTrait for XInteger<'a> {
//     fn fake(&self) {}
//
//     fn get_type(&self) -> &EntityType {
//         &self.entity_type
//     }
// }
//
// impl<'a> XVariableTrait for XInteger<'a> {
//     fn to_string(&self) -> String {
//         format!("XInteger {}:{}", self.id, self.value.to_string())
//     }
//     fn get_domain(&self) -> &Option<&XDomainInteger> {
//         &self.domain
//     }
// }
//
// impl<'a> XInteger<'a> {
//     pub fn from(lid: String, n: i32) -> Self {
//         XInteger {
//             id: lid,
//             classes: String::default(),
//             domain: None,
//             value: n,
//             entity_type: EntityType::XInteger,
//         }
//     }
// }
//
// pub struct XTree<'a> {
//     classes: String,
//     domain: Option<&'a XDomainInteger>,
//     id: String,
//     entity_type: EntityType,
// }
//
// impl<'a> XTree<'a> {
//     pub fn from(lid: String) -> Self {
//         XTree {
//             id: lid,
//             classes: String::default(),
//             domain: None,
//             entity_type: EntityType::XTree,
//         }
//     }
// }
//
// impl<'a> XEntityTrait for XTree<'a> {
//     fn fake(&self) {}
//
//     fn get_type(&self) -> &EntityType {
//         &self.entity_type
//     }
// }
//
// impl<'a> XVariableTrait for XTree<'a> {
//     fn to_string(&self) -> String {
//         format!("XTree {}", self.id)
//     }
//     fn get_domain(&self) -> &Option<&XDomainInteger> {
//         &self.domain
//     }
// }
//
// pub struct XEInterval<'a> {
//     classes: String,
//     domain: Option<&'a XDomainInteger>,
//     id: String,
//     entity_type: EntityType,
//     max: i32,
//     min: i32,
// }
//
// impl<'a> XEInterval<'a> {
//     pub fn from(lid: String, mn: i32, mx: i32) -> Self {
//         XEInterval {
//             id: lid,
//             classes: String::default(),
//             domain: None,
//             entity_type: EntityType::XEInterval,
//             max: mx,
//             min: mn,
//         }
//     }
// }
//
// impl<'a> XEntityTrait for XEInterval<'a> {
//     fn fake(&self) {}
//
//     fn get_type(&self) -> &EntityType {
//         &self.entity_type
//     }
// }
//
// impl<'a> XVariableTrait for XEInterval<'a> {
//     fn to_string(&self) -> String {
//         format!(
//             "XEInterval {} :[min: {}, max:{}]",
//             self.id, self.min, self.max
//         )
//     }
//     fn get_domain(&self) -> &Option<&XDomainInteger> {
//         &self.domain
//     }
// }

// pub fn is_integer(object: &dyn XEntityTrait) -> bool {
//     match object.get_type() {
//         EntityType::XInteger => true,
//         _ => false,
//     }
// }
//
// pub fn is_interval(object: &dyn XEntityTrait) -> bool {
//     match object.get_type() {
//         EntityType::XInteger => true,
//         _ => false,
//     }
// }

// pub struct XParameterVariable<'a> {
//     classes: String,
//     domain: Option<&'a XDomainInteger>,
//     id: String,
//     entity_type: EntityType,
//     number: i32,
// }
//
// static XPARAMETER_VARIABLE_MAX: AtomicI32 = AtomicI32::new(i32::MIN);
//
// impl<'a> XParameterVariable<'a> {
//     pub fn from(lid: String) -> Self {
//         let n: i32;
//         if lid.as_str().bytes().nth(1) == '.'.to_string().bytes().nth(0) {
//             n = -1;
//         } else {
//             let nn = &lid.as_str()[1..];
//             n = nn.to_string().parse().unwrap();
//         }
//         XPARAMETER_VARIABLE_MAX.fetch_max(n, Ordering::Relaxed);
//         XParameterVariable {
//             id: lid,
//             classes: String::default(),
//             domain: None,
//             entity_type: EntityType::XParameterVariable,
//             number: n,
//         }
//     }
// }
//
// impl<'a> XEntityTrait for XParameterVariable<'a> {
//     fn fake(&self) {}
//
//     fn get_type(&self) -> &EntityType {
//         &self.entity_type
//     }
// }
//
// impl<'a> XVariableTrait for XParameterVariable<'a> {
//     fn to_string(&self) -> String {
//         format!("XParameterVariable {}", self.id)
//     }
//     fn get_domain(&self) -> &Option<&XDomainInteger> {
//         &self.domain
//     }
// }
