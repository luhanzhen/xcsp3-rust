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
// #[allow(dead_code)]

pub mod xcsp3_core {
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use std::str::FromStr;

    #[derive(Clone)]
    pub enum XIntegerType {
        Empty,
        IntegerValue(XIntegerValue),
        IntegerInterval(XIntegerInterval),
        XIntegerSymbolic(XIntegerSymbolic),
    }

    impl XIntegerType {
        pub fn equals(&self, arg: &XIntegerType) -> bool {
            match self {
                XIntegerType::IntegerValue(iv) => match arg {
                    XIntegerType::IntegerValue(iiv) => iv.equals(iiv),
                    _ => false,
                },
                XIntegerType::IntegerInterval(ii) => match arg {
                    XIntegerType::IntegerInterval(iii) => ii.equals(iii),
                    _ => false,
                },
                XIntegerType::XIntegerSymbolic(ii) => match arg {
                    XIntegerType::XIntegerSymbolic(iii) => ii.equals(iii),
                    _ => false,
                },
                _ => false,
            }
        }

        pub fn to_string(&self) -> String {
            match self {
                XIntegerType::IntegerValue(iv) => iv.to_string(),
                XIntegerType::IntegerInterval(ii) => ii.to_string(),
                XIntegerType::XIntegerSymbolic(ii) => ii.to_string(),
                _ => "empty".to_string(),
            }
        }
        pub fn maximum(&self) -> i32 {
            match self {
                XIntegerType::IntegerValue(iv) => iv.maximum(),
                XIntegerType::IntegerInterval(ii) => ii.maximum(),
                XIntegerType::XIntegerSymbolic(ii) => ii.maximum(),
                _ => 2_147_483_647i32,
                //i32::MAX, my ide named clion tell me i32::MAX is private constant, but rustc can compile it...
            }
        }

        pub fn minimum(&self) -> i32 {
            match self {
                XIntegerType::IntegerValue(iv) => iv.minimum(),
                XIntegerType::IntegerInterval(ii) => ii.minimum(),
                XIntegerType::XIntegerSymbolic(ii) => ii.minimum(),
                _ => -2_147_483_648i32, //i32::MIN,
            }
        }
    }

    pub trait XIntegerEntity {
        fn width(&self) -> usize;

        fn minimum(&self) -> i32;

        fn maximum(&self) -> i32;

        fn print(&self);

        fn to_string(&self) -> String;

        fn equals(&self, arg: &dyn XIntegerEntity) -> bool;
    }

    #[derive(Copy, Clone)]
    pub struct XIntegerValue {
        value: i32,
    }

    impl From<i32> for XIntegerValue {
        fn from(value: i32) -> Self {
            XIntegerValue { value }
        }
    }

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

        fn equals(&self, arg: &dyn XIntegerEntity) -> bool {
            self.value == arg.minimum()
        }
    }

    #[derive(Clone)]
    pub struct XIntegerSymbolic {
        values: Vec<i32>,
        symbolic: Vec<String>,
    }

    impl XIntegerSymbolic {
        pub fn new(domain: &str) -> XIntegerSymbolic {
            let dds: Vec<&str> = domain.split_whitespace().collect();
            let mut symbolic: Vec<String> = vec![];
            let mut values: Vec<i32> = vec![];
            for (i, s) in dds.iter().enumerate() {
                values.push(i as i32);
                symbolic.push(s.to_string());
            }
            XIntegerSymbolic { symbolic, values }
        }
    }

    impl XIntegerEntity for XIntegerSymbolic {
        fn width(&self) -> usize {
            self.values.len()
        }

        fn minimum(&self) -> i32 {
            self.values[0]
        }

        fn maximum(&self) -> i32 {
            self.values[self.values.len() - 1]
        }

        fn print(&self) {
            todo!()
        }

        fn to_string(&self) -> String {
            // format!("{}..{}", self.minimum(), self.maximum())
            let mut ret = String::default();
            for i in 0..self.symbolic.len() {
                let s = format!("{}({}), ", self.values[i], self.symbolic[i]);
                ret.push_str(&s);
            }
            ret
        }

        fn equals(&self, arg: &dyn XIntegerEntity) -> bool {
            self.minimum() == arg.minimum() && self.minimum() == arg.maximum()
        }
    }

    #[derive(Copy, Clone)]
    pub struct XIntegerInterval {
        max: i32,
        min: i32,
    }

    impl XIntegerInterval {
        pub fn new(min: i32, max: i32) -> XIntegerInterval {
            XIntegerInterval { max, min }
        }
    }

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
            print!("{}", self.to_string());
        }

        fn to_string(&self) -> String {
            format!("{}..{}", self.minimum(), self.maximum())
        }

        fn equals(&self, arg: &dyn XIntegerEntity) -> bool {
            self.min == arg.minimum() && self.max == arg.maximum()
        }
    }

    #[derive(Clone)]
    pub struct XDomainInteger {
        size: usize,
        top: i32,
        values: Vec<XIntegerType>,
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

        pub fn from_symbolic(domain: &str) -> XDomainInteger {
            let entity = XIntegerType::XIntegerSymbolic(XIntegerSymbolic::new(domain));
            XDomainInteger {
                size: 0,
                top: 0,
                values: vec![entity],
            }
        }

        pub fn from_string(domain: &str) -> Result<XDomainInteger, Xcsp3Error> {
            let mut ret: XDomainInteger = XDomainInteger::new();
            let domains: Vec<&str> = domain.split_whitespace().collect();

            for d in domains.iter() {
                if d.contains("..") {
                    let interval: Vec<&str> = d.split("..").collect();
                    if interval.len() == 2 {
                        let left = i32::from_str(interval[0]);
                        let right = i32::from_str(interval[1]);
                        match left {
                            Ok(l) => match right {
                                Ok(r) => {
                                    ret.add_interval(l, r);
                                }
                                Err(_) => {
                                    return Err(Xcsp3Error::get_domain_interval_error(
                                        "parse the domain error",
                                    ));
                                }
                            },
                            Err(_) => {
                                return Err(Xcsp3Error::get_domain_interval_error(
                                    "parse the domain error",
                                ));
                            }
                        }
                    }
                } else {
                    match i32::from_str(d) {
                        Ok(v) => ret.add_value(v),
                        Err(_) => {
                            // ret.add_value(i as i32)
                            return Err(Xcsp3Error::get_domain_integer_error(
                                "parse the domain error",
                            ));
                        }
                    };
                }
            }

            Ok(ret)
        }

        fn add_entity(&mut self, entity: XIntegerType) {
            match entity {
                XIntegerType::IntegerValue(iv) => {
                    self.size += iv.width();
                }
                XIntegerType::IntegerInterval(ii) => {
                    self.size += ii.width();
                }
                _ => {}
            }

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
                if !self.values[i].equals(e) {
                    return false;
                }
            }
            true
        }

        pub fn add_value(&mut self, value: i32) {
            if value > self.top {
                let b = XIntegerType::IntegerValue(XIntegerValue::from(value));
                self.add_entity(b);
                self.top = value;
            } else {
                panic!("not sequence domain");
            }
        }

        pub fn add_interval(&mut self, min: i32, max: i32) {
            if min > self.top && min <= max {
                let b = XIntegerType::IntegerInterval(XIntegerInterval::new(min, max));
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

        pub fn to_string(&self) -> String {
            let mut s = String::new();
            for e in self.values.iter() {
                s = format!("{} {}", s, e.to_string());
            }
            // for e in self.iter() {
            //     s = format!("{} {}", s, e.to_string());
            // }
            s
        }

        pub fn iter(&self) -> XDomainIter {
            XDomainIter {
                values: &self.values,
                current: 0,
                current1: i32::MAX,
            }
        }
    }

    pub struct XDomainIter<'a> {
        values: &'a Vec<XIntegerType>,
        current: usize,
        current1: i32,
    }

    impl Iterator for XDomainIter<'_> {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            let mut ret: Option<Self::Item> = None;
            if self.current >= self.values.len() {
                return ret;
            }
            for cc in self.current..self.values.len() {
                match &self.values[cc] {
                    XIntegerType::XIntegerSymbolic(s) => {
                        if self.current1 == i32::MAX {
                            self.current1 = s.minimum();
                            ret = Some(self.current1);
                            self.current1 += 1;
                            break;
                        } else if self.current1 > s.maximum() {
                            self.current += 1;
                            continue;
                        } else {
                            ret = Some(self.current1);
                            self.current1 += 1;
                            break;
                        }
                    }
                    XIntegerType::Empty => {
                        self.current += 1;
                        continue;
                    }
                    XIntegerType::IntegerValue(v) => {
                        self.current += 1;
                        ret = Some(v.value);
                        break;
                    }
                    XIntegerType::IntegerInterval(i) => {
                        if self.current1 == i32::MAX {
                            self.current1 = i.minimum();
                            ret = Some(self.current1);
                            self.current1 += 1;
                            break;
                        } else if self.current1 > i.maximum() {
                            self.current1 = i32::MAX;
                            self.current += 1;
                            continue;
                        } else {
                            ret = Some(self.current1);
                            self.current1 += 1;
                            break;
                        }
                    }
                }
            }
            ret
        }
    }
}
