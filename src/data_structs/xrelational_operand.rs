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

/*
 * <p>@project_name: xcsp3-rust
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/7/21 10:38
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

pub mod xcsp3_core {
    use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
    use std::collections::HashSet;
    use std::str::FromStr;

    #[derive(Clone, Debug)]
    pub enum Operand {
        Integer(i32),
        Variable(String),
        Interval(i32, i32),
        SetInteger(HashSet<i32>),
    }

    impl Operand {
        pub fn get_operand_by_str(s: &[&str], op: &Operator) -> Option<Self> {
            let mut is_set: bool = false;
            match op {
                Operator::In => is_set = true,
                Operator::Notin => is_set = true,
                _ => {}
            }
            return if is_set {
                let mut ret: HashSet<i32> = HashSet::new();
                for l in s.iter() {
                    match i32::from_str(l) {
                        Ok(n) => {
                            ret.insert(n);
                        }
                        Err(_) => {
                            return None;
                        }
                    }
                }
                Some(Operand::SetInteger(ret))
            } else if s.len() != 1 {
                None
            } else if s[0].contains("..") {
                let interval: Vec<&str> = s[0].split("..").collect();
                if interval.len() == 2 {
                    match i32::from_str(interval[0]) {
                        Ok(l) => match i32::from_str(interval[1]) {
                            Ok(r) => {
                                if l <= r {
                                    Some(Operand::Interval(l, r))
                                } else {
                                    None
                                }
                            }
                            Err(_) => None,
                        },
                        Err(_) => None,
                    }
                } else {
                    None
                }
            } else {
                match i32::from_str(s[0]) {
                    Ok(n) => Some(Operand::Integer(n)),
                    Err(_) => Some(Operand::Variable(s[0].to_string())),
                }
            };
        }
    }
}
