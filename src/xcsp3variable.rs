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
 */
#[allow(dead_code)]
pub mod xcsp3_core {

    use crate::xcsp3domain::xcsp3_core::*;

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

    pub trait XVariableTrait {
        fn to_string(&self) -> String;
        fn fake(&mut self);
    }

    pub struct XInteger {
        classes: String,
        domain: Option<XDomainInteger>,
        value: i32,
        id: String,
    }

    impl XInteger {
        pub fn from(id: String, dom: XDomainInteger) -> Self {
            let e = XEntity::from(id);
            XVariable {
                entity: e,
                classes: String::default(),
                domain: dom,
            }
        }

        pub fn from_with_index(mut id: String, dom: XDomainInteger, indexes: &[i32]) -> Self {
            for i in indexes.iter() {
                id = format!("{}[{}]", id, i);
            }
            XVariable {
                entity: XEntity::from(id),
                classes: String::default(),
                domain: dom,
            }
        }
    }
}
