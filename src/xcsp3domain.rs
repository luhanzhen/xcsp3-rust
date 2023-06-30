use std::ops::Deref;

/**
 * <p>@project_name: XCSP3-Rust
 * <p/>
 * <p>@author: luhanzhen
 * <p/>
 * <p>@date: 2023/6/30
 * <p/>
 * <p>@time: 13:55
 * <p/>
 * <p>@this_file_name:xcsp3domain
 * <p/>
 */

pub trait XIntegerEntity {
    fn width(&self) -> usize;

    fn minimum(&self) -> i32;

    fn maximum(&self) -> i32;

    fn print(&self);

    fn to_string(&self) -> String;

    fn drop(&self) {}

    fn equals(&self, arg: &dyn XIntegerEntity) -> bool;
    // return self.width() == arg.width();
}

pub struct XIntegerValue {
    value: i32,
}

impl XIntegerValue {
    pub fn new(v: i32) -> XIntegerValue {
        XIntegerValue { value: v }
    }

    // fn equals(&self, arg: &XIntegerValue) -> bool {
    //     return self.value == arg.value;
    // }
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

    fn drop(&self) {
        drop(self.value);
    }

    fn equals(&self, arg: &dyn XIntegerEntity) -> bool {
        self.value == arg.minimum()
    }
}

pub struct XIntegerInterval {
    max: i32,
    min: i32,
}

impl XIntegerInterval {
    pub fn new(max: i32, min: i32) -> XIntegerInterval {
        XIntegerInterval { max, min }
    }

    // fn equals(&self, arg: &XIntegerInterval) -> bool {
    //     return self.min == arg.min && self.max == arg.max;
    // }
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
        print!("{}..{}", self.min, self.max);
    }

    fn to_string(&self) -> String {
        format!(
            "{}..{}",
            self.minimum().to_string(),
            self.maximum().to_string()
        )
    }

    fn drop(&self) {}

    fn equals(&self, arg: &dyn XIntegerEntity) -> bool {
        return self.min == arg.minimum() && self.max == arg.maximum();
    }
}

pub struct XDomainInteger<'a, T: XIntegerEntity> {
    size: usize,
    top: i32,
    pub values: Vec<Box<&'a T>>,
}

impl<'a, T: XIntegerEntity> XDomainInteger<'a, T> {
    pub fn new() -> XDomainInteger<'a, T> {
        XDomainInteger {
            size: 0,
            top: i32::MIN,
            values: vec![],
        }
    }
    fn add_entity(&mut self, entity: &'a T) {
        self.size += entity.width();
        self.values.push(Box::new(entity));
    }

    pub fn equals(&self, arg: &XDomainInteger<T>) -> bool {
        if self.nb_values() != arg.nb_values() {
            return false;
        }
        if self.values.len() != arg.values.len() {
            return false;
        }
        for (i, e) in arg.values.iter().enumerate() {
            if !self.values[i].equals(e as &T) {
                return false;
            }
        }
        return true;
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

    pub fn drop(&mut self) {
        // for i in self.values.into_iter()
        // {
        //     drop(i)
        // }
    }
}

impl<'a, T: XIntegerEntity> Drop for XDomainInteger<'a, T> {
    fn drop(&mut self) {
        // std::mem::drop(self.values);
        // for (i,_) in self.values.iter().enumerate()
        // {
        //     drop(self.values[i].clone())
        // }
    }
}
