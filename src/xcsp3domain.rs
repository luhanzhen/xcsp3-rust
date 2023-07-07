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
 * <p>@time: 13:55
 * </p>
 * <p>@this_file_name:xcsp3domain
 * </p>
 */
#[allow(dead_code)]
pub mod xcsp3_core {
    use std::ops::Deref;
    pub enum XIntegerType {
        IntegerValue,
        IntegerInterval,
    }

    pub trait XIntegerEntity {
        fn width(&self) -> usize;

        fn minimum(&self) -> i32;

        fn maximum(&self) -> i32;

        fn print(&self);

        fn to_string(&self) -> String;

        fn drop(&self) {}

        fn equals(&self, arg: &dyn XIntegerEntity) -> bool;

        fn get_type(&self) -> &XIntegerType;
        // return self.width() == arg.width();
    }

    pub struct XIntegerValue {
        value: i32,
        integer_type: XIntegerType,
    }

    impl XIntegerValue {
        pub fn new(v: i32) -> XIntegerValue {
            XIntegerValue {
                value: v,
                integer_type: XIntegerType::IntegerValue,
            }
        }

        // fn equals(&self, arg: &XIntegerValue) -> bool {
        //     return self.value == arg.value;
        // }
    }

    // impl Clone for XIntegerValue {
    //     fn clone(&self) -> Self {
    //         XIntegerValue {
    //             value: self.value,
    //             integer_type: XIntegerType::IntegerValue,
    //         }
    //     }
    // }

    impl XIntegerEntity for XIntegerValue {
        fn width(&self) -> usize {
            1
        }

        fn minimum(&self) -> i32 {
            self.value
        }

        fn maximum(&self) -> i32 {
            self.value
        }

        fn print(&self) {
            print!("{}", self.value);
        }

        fn to_string(&self) -> String {
            self.value.to_string()
        }

        fn drop(&self) {
            // drop(self.value);
        }

        fn equals(&self, arg: &dyn XIntegerEntity) -> bool {
            self.value == arg.minimum()
        }

        fn get_type(&self) -> &XIntegerType {
            &self.integer_type
        }

        // fn clone(&self) -> Self {
        //     XIntegerValue { value: self.value }
        // }
    }

    pub struct XIntegerInterval {
        max: i32,
        min: i32,
        integer_type: XIntegerType,
    }

    impl XIntegerInterval {
        pub fn new(min: i32, max: i32) -> XIntegerInterval {
            XIntegerInterval {
                max,
                min,
                integer_type: XIntegerType::IntegerValue,
            }
        }

        // fn equals(&self, arg: &XIntegerInterval) -> bool {
        //     return self.min == arg.min && self.max == arg.max;
        // }
    }

    // impl Clone for XIntegerInterval {
    //     fn clone(&self) -> Self {
    //         XIntegerInterval {
    //             max:self.max,
    //             min:self.min,
    //             integer_type: XIntegerType::IntegerValue,
    //         }
    //     }
    // }

    impl XIntegerEntity for XIntegerInterval {
        fn width(&self) -> usize {
            (self.max - self.min + 1) as usize
        }

        fn minimum(&self) -> i32 {
            self.min
        }

        fn maximum(&self) -> i32 {
            self.max
        }

        fn print(&self) {
            print!("{}..{}", self.min, self.max);
        }

        fn to_string(&self) -> String {
            format!("{}..{}", self.minimum(), self.maximum())
        }

        fn drop(&self) {}

        fn equals(&self, arg: &dyn XIntegerEntity) -> bool {
            self.min == arg.minimum() && self.max == arg.maximum()
        }

        fn get_type(&self) -> &XIntegerType {
            &self.integer_type
        }
    }

    // #[derive(Clone, Debug)]
    pub struct XDomainInteger {
        size: usize,
        top: i32,
        values: Vec<Box<dyn XIntegerEntity>>,
    }

    impl Clone for XDomainInteger {
        fn clone(&self) -> Self {
            let mut v = Vec::new();
            for e in self.values.iter() {
                let ee: Box<dyn XIntegerEntity> = match e.get_type() {
                    XIntegerType::IntegerValue => Box::new(XIntegerValue::new(e.deref().minimum())),
                    XIntegerType::IntegerInterval => Box::new(XIntegerInterval::new(
                        e.deref().minimum(),
                        e.deref().minimum(),
                    )),
                };
                v.push(ee);
            }
            XDomainInteger {
                size: self.size,
                top: self.top,
                values: v,
            }
        }
    }

    impl Default for XDomainInteger {
        fn default() -> Self {
            Self::new()
        }
    }

    impl XDomainInteger {
        pub fn new() -> XDomainInteger {
            XDomainInteger {
                size: 0,
                top: i32::MIN,
                values: vec![],
            }
        }

        fn add_entity(&mut self, entity: Box<dyn XIntegerEntity>) {
            self.size += entity.width();
            self.values.push(entity);
        }

        pub fn equals(&self, arg: &XDomainInteger) -> bool {
            if self.nb_values() != arg.nb_values() {
                return false;
            }
            if self.values.len() != arg.values.len() {
                return false;
            }
            for (i, e) in arg.values.iter().enumerate() {
                if !self.values[i].equals(e.deref()) {
                    return false;
                }
            }
            true
        }

        pub fn add_value(&mut self, value: i32) {
            if value > self.top {
                let b = Box::new(XIntegerValue::new(value));
                self.add_entity(b);
                self.top = value;
            } else {
                panic!("not sequence domain");
            }
        }

        pub fn add_interval(&mut self, min: i32, max: i32) {
            if min > self.top && min <= max {
                let b = Box::new(XIntegerInterval::new(min, max));
                self.add_entity(b);
                self.top = max;
            } else {
                panic!("not sequence domain");
            }
        }

        pub fn nb_values(&self) -> usize {
            self.size
        }

        pub fn minimum(&self) -> i32 {
            self.values[0].minimum()
        }

        pub fn maximum(&self) -> i32 {
            self.values[self.values.len() - 1].maximum()
        }

        pub fn is_interval(&self) -> bool {
            self.size == (self.maximum() - self.minimum() + 1) as usize
        }
    }
}
