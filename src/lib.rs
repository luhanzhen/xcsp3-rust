pub mod xcsp3constants;

use xcsp3constants::*;

pub mod xcsp3variable;

use xcsp3variable::*;

mod xcsp3domain;

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
    use super::*;
    use crate::xcsp3domain::{XIntegerInterval, *};

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
        assert_ne!(domain.is_interval(), true);
        assert_eq!(domain.nb_values(), 12);

        let mut domain1: XDomainInteger = XDomainInteger::new();
        domain1.add_value(30);
        domain1.add_interval(40, 50);
        assert_eq!(domain.equals(&domain1), true);

        let mut domain2: XDomainInteger = XDomainInteger::new();
        domain2.add_value(30);
        domain2.add_interval(45, 55);
        assert_eq!(domain.equals(&domain2), false);
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
