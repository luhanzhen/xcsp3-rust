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
#[allow(dead_code)]
pub mod xcsp3_core {

    use crate::xcsp3domain::xcsp3_core::*;

    #[derive(Clone)]
    pub enum XVariableType {
        XVariableArray(XVariableInt),
        XVariableInt(XVariableArray),
    }

    #[derive(Clone)]
    pub struct XVariableInt {
        classes: String,
        domain: Vec<XDomainInteger>,
        id: String,
    }

    #[derive(Clone)]
    pub struct XVariableArray {
        classes: String,
        variables: Vec<XVariableType>,
        sizes: Vec<i32>,
        id: String,
    }

    impl XVariableArray {
        pub fn from_sizes(lid: String, szs: &Vec<i32>) -> Self {
            XVariableArray {
                id: lid,
                classes: String::default(),
                variables: vec![],
                sizes: szs.clone(),
            }
        }
    }
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
