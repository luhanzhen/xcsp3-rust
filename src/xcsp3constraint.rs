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
    use crate::xcsp3error::xcsp3_core::Xcsp3Error;
    use std::slice::Iter;
    use std::str::FromStr;

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
            if let Ok(tuples) = tuple_to_vector(tuple) {
                self.constraints
                    .push(XConstraintType::XExtension(XExtension::new(
                        list_to_scope_ids(list),
                        tuples,
                        is_support,
                    )))
            }
        }
    }
    #[derive(Clone)]
    pub enum XConstraintType {
        None,
        XExtension(XExtension),
    }

    impl XConstraintType {
        pub fn to_string(&self) -> String {
            match self {
                XConstraintType::None => String::default(),
                XConstraintType::XExtension(c) => c.to_string(),
            }
        }
    }

    pub trait XConstraintTrait {
        fn to_string(&self) -> String;
    }

    #[derive(Clone)]
    pub struct XExtension {
        scope: Vec<String>,
        tuples: Vec<Vec<i32>>,
        is_support: bool,
    }

    impl XConstraintTrait for XExtension {
        fn to_string(&self) -> String {
            format!(
                "XExtension: scope = {:?}, tuples = {:?}, is_support = {}",
                self.scope, self.tuples, self.is_support
            )
        }
    }

    impl XExtension {
        pub fn new(scope: Vec<String>, tuples: Vec<Vec<i32>>, is_support: bool) -> XExtension {
            XExtension {
                scope,
                tuples,
                is_support,
            }
        }
    }

    fn list_to_scope_ids(list: &str) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        let lists: Vec<&str> = list.split_whitespace().collect();
        for e in lists.iter() {
            ret.push(e.to_string());
        }
        ret
    }

    fn tuple_to_vector(tuple: &str) -> Result<Vec<Vec<i32>>, Xcsp3Error> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let tuple = tuple.replace("(", " ");
        let tuple = tuple.replace(")", " ");
        let tuples_str: Vec<&str> = tuple.split_whitespace().collect();
        for ts in tuples_str.iter() {
            let tuple_str: Vec<&str> = ts.split(",").collect();
            let mut tt: Vec<i32> = Vec::new();
            for i in tuple_str.iter() {
                match i32::from_str(i) {
                    Ok(num) => {
                        tt.push(num);
                    }
                    Err(_) => {
                        return Err(Xcsp3Error::get_constraint_extension_error(
                            "parse the tuple of extension error",
                        ))
                    }
                }
            }
            ret.push(tt);
        }
        Ok(ret)
    }
}
