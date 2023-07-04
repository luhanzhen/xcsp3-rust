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
 * <p/>
 * <p>@author: luhanzhen
 * <p/>
 * <p>@date: 2023/6/30
 * <p/>
 * <p>@time: 13:47
 * <p/>
 * <p>@this_file_name:XCSP3Variable
 * <p/>
 **/
#[allow(dead_code)]
pub mod xcsp3_core {
    use crate::xcsp3domain::xcsp3_core::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    #[derive(Default)]
    pub struct XInterval {
        min: i32,
        max: i32,
    }

    impl XInterval {
        pub fn new(min: i32, max: i32) -> XInterval {
            XInterval { min, max }
        }

        // pub fn to_string(&self) -> String {
        //     format!("[{},{}]", self.min, self.max)
        // }
    }

    pub trait XEntityTrait {
        fn fake(&self);
        fn get_type(&self) -> &EntityType;
    }

    pub trait XVariableTrait: XEntityTrait {
        fn to_string(&self) -> String;
    }

    pub enum EntityType {
        XInteger,
        XTree,
        XEInterval,
        XParameterVariable,
        XVariableArray,
    }

    pub struct XInteger {
        classes: String,
        domain: Option<XDomainInteger>,
        value: i32,
        id: String,
        entity_type: EntityType,
    }

    impl XEntityTrait for XInteger {
        fn fake(&self) {}

        fn get_type(&self) -> &EntityType {
            &self.entity_type
        }
    }

    impl XVariableTrait for XInteger {
        fn to_string(&self) -> String {
            format!("XInteger {}:{}", self.id, self.value.to_string())
        }
    }

    impl XInteger {
        pub fn from(lid: String, n: i32) -> Self {
            XInteger {
                id: lid,
                classes: String::default(),
                domain: None,
                value: n,
                entity_type: EntityType::XInteger,
            }
        }

        // pub fn from_with_index(mut id: String, dom: XDomainInteger, indexes: &[i32]) -> Self {
        //     for i in indexes.iter() {
        //         id = format!("{}[{}]", id, i);
        //     }
        //     XVariable {
        //         entity: XEntity::from(id),
        //         classes: String::default(),
        //         domain: dom,
        //     }
        // }
    }

    pub struct XTree {
        classes: String,
        domain: Option<XDomainInteger>,
        id: String,
        entity_type: EntityType,
    }

    impl XTree {
        pub fn from(lid: String) -> Self {
            XTree {
                id: lid,
                classes: String::default(),
                domain: None,
                entity_type: EntityType::XTree,
            }
        }
    }

    impl XEntityTrait for XTree {
        fn fake(&self) {}

        fn get_type(&self) -> &EntityType {
            &self.entity_type
        }
    }

    impl XVariableTrait for XTree {
        fn to_string(&self) -> String {
            format!("XTree {}", self.id)
        }
    }

    pub struct XEInterval {
        classes: String,
        domain: Option<XDomainInteger>,
        id: String,
        entity_type: EntityType,
        max: i32,
        min: i32,
    }

    impl XEInterval {
        pub fn from(lid: String, mn: i32, mx: i32) -> Self {
            XEInterval {
                id: lid,
                classes: String::default(),
                domain: None,
                entity_type: EntityType::XEInterval,
                max: mx,
                min: mn,
            }
        }
    }

    impl XEntityTrait for XEInterval {
        fn fake(&self) {}

        fn get_type(&self) -> &EntityType {
            &self.entity_type
        }
    }

    impl XVariableTrait for XEInterval {
        fn to_string(&self) -> String {
            format!(
                "XEInterval {} :[min: {}, max:{}]",
                self.id, self.min, self.max
            )
        }
    }

    pub fn is_integer(object: &dyn XEntityTrait) -> bool {
        match object.get_type() {
            EntityType::XInteger => true,
            _ => false,
        }
    }

    pub fn is_interval(object: &dyn XEntityTrait) -> bool {
        match object.get_type() {
            EntityType::XInteger => true,
            _ => false,
        }
    }

    pub struct XParameterVariable {
        classes: String,
        domain: Option<XDomainInteger>,
        id: String,
        entity_type: EntityType,
        number: i32,
    }

    static XPARAMETER_VARIABLE_MAX: AtomicI32 = AtomicI32::new(i32::MIN);

    impl XParameterVariable {
        pub fn from(lid: String) -> Self {
            let n: i32;
            if lid.as_str().bytes().nth(1) == '.'.to_string().bytes().nth(0) {
                n = -1;
            } else {
                let nn = &lid.as_str()[1..];
                n = nn.to_string().parse().unwrap();
            }
            XPARAMETER_VARIABLE_MAX.fetch_max(n, Ordering::Relaxed);
            XParameterVariable {
                id: lid,
                classes: String::default(),
                domain: None,
                entity_type: EntityType::XParameterVariable,
                number: n,
            }
        }
    }

    impl XEntityTrait for XParameterVariable {
        fn fake(&self) {}

        fn get_type(&self) -> &EntityType {
            &self.entity_type
        }
    }

    impl XVariableTrait for XParameterVariable {
        fn to_string(&self) -> String {
            format!("XParameterVariable {}", self.id)
        }
    }

    struct XVariableArray<'a> {
        classes: String,
        entity_type: EntityType,
        variables: Vec<&'a dyn XVariableTrait>,
        sizes: Vec<i32>,
        id: String,
    }
    impl<'a> XVariableArray<'a> {
        pub fn from_sizes(lid: String, szs: &Vec<i32>) -> Self {
            XVariableArray {
                id: lid,
                classes: String::default(),
                entity_type: EntityType::XVariableArray,
                variables: vec![],
                sizes: szs.clone(),
            }
        }
        pub fn from_array(lid: String, variables: &Vec<&'a dyn XVariableTrait>) -> Self {
            XVariableArray {
                id: lid,
                classes: String::default(),
                entity_type: EntityType::XVariableArray,
                variables: vec![],
                sizes: szs.clone(),
            }
        }
    }

    impl<'a> XEntityTrait for XVariableArray<'a> {
        fn fake(&self) {}

        fn get_type(&self) -> &EntityType {
            &self.entity_type
        }
    }
}
