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

pub mod xcsp3constants;
pub mod xcsp3domain;
pub mod xcsp3tree;
pub mod xcsp3treenode;
pub mod xcsp3utils;
pub mod xcsp3variable;

/**
 * <p>@project_name: XCSP3-Rust
 * <p/>
 * <p>@author: luhanzhen
 * <p/>
 * <p>@date: 2023/6/30
 * <p/>
 * <p>@time: 13:40
 * <p/>
 * <p>@this_file_name:lib.rs
 * <p/>
 */

#[cfg(test)]
mod test_xcsp3domain {

    use super::xcsp3domain::xcsp3_core::*;

    #[test]
    fn test_xinteger_value() {
        let value = XIntegerValue::new(42);

        assert_eq!(value.minimum(), 42);
        assert_eq!(value.width(), 1);
        assert_eq!(value.maximum(), 42);
        assert_eq!(value.to_string(), "42");
        assert!(value.equals(&XIntegerValue::new(42)));
        assert!(!value.equals(&XIntegerValue::new(2)));
        assert!(!value.equals(&XIntegerValue::new(4)));
    }

    #[test]
    fn test_xinteger_interval() {
        let interval = XIntegerInterval::new(40, 50);
        assert_eq!(interval.width(), 11);
        assert_eq!(interval.minimum(), 40);
        assert_eq!(interval.maximum(), 50);
        assert_eq!(interval.to_string(), "40..50");
        assert!(interval.equals(&XIntegerInterval::new(40, 50)));
        assert!(!interval.equals(&XIntegerInterval::new(41, 50)));
        assert!(!interval.equals(&XIntegerInterval::new(40, 49)));
        assert!(!interval.equals(&XIntegerInterval::new(41, 49)));
    }

    #[test]
    fn test_xdomain_integer() {
        let mut domain: XDomainInteger = XDomainInteger::new();

        // let interval = XIntegerInterval::new(40, 50);
        // let value = XIntegerValue::new(30);
        domain.add_value(30);
        domain.add_interval(40, 50);
        assert_eq!(domain.minimum(), 30);
        assert_eq!(domain.maximum(), 50);
        assert!(!domain.is_interval());
        assert_eq!(domain.nb_values(), 12);

        let mut domain1: XDomainInteger = XDomainInteger::new();
        domain1.add_value(30);
        domain1.add_interval(40, 50);
        assert!(domain.equals(&domain1));

        let mut domain2: XDomainInteger = XDomainInteger::new();
        domain2.add_value(30);
        domain2.add_interval(45, 55);
        assert!(!domain.equals(&domain2));
    }

    #[test]
    #[should_panic(expected = "not sequence domain")]
    fn test_xdomain_integer1() {
        let mut domain: XDomainInteger = XDomainInteger::new();
        domain.add_interval(40, 50);
        domain.add_value(30);
    }
}

#[cfg(test)]
mod test_xcsp3variable {
    #[test]
    fn test_xinteger_value() {}
}
