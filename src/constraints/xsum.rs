/*
 * <p>@project_name: xcsp3-rust
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/7/20 14:52
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

pub mod xcsp3_core {
    use crate::constraints::xconstraint_trait::xcsp3_core::XConstraintTrait;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils::xcsp3_utils::{list_to_scope_ids, list_to_values};
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::collections::HashSet;
    use std::str::FromStr;

    #[derive(Clone, Debug)]
    pub enum Operator {
        Lt,
        Le,
        Ge,
        Gt,
        Eq,
        Ne,
        In,
        Notin,
    }

    impl Operator {
        pub fn get_operator_by_str(op: &str) -> Option<Self> {
            return match op {
                "lt" => Some(Self::Lt),
                "le" => Some(Self::Le),
                "ge" => Some(Self::Ge),
                "gt" => Some(Self::Gt),
                "ne" => Some(Self::Ne),
                "eq" => Some(Self::Eq),
                "in" => Some(Self::In),
                "notin" => Some(Self::Notin),
                _ => None,
            };
        }
    }

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
            } else {
                if s.len() != 1 {
                    None
                } else {
                    if s[0].contains("..") {
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
                    }
                }
            };
        }
    }

    #[derive(Clone)]
    pub struct XSum<'a> {
        scope_vec_str: Vec<String>,
        scope_vec_var: Vec<(String, &'a XDomainInteger)>,
        operator: Operator,
        operand: Operand,
        coeffs: Vec<i32>,
    }

    impl XConstraintTrait for XSum<'_> {
        fn to_string(&self) -> String {
            let mut ret = format!("XSum: scope =  ",);
            for e in self.scope_vec_var.iter() {
                ret.push_str(e.0.as_str());
                ret.push_str("(");
                ret.push_str(e.1.to_string().as_str());
                ret.push_str("), ")
            }
            ret.push_str(&format!(
                "coeffs = {:?}, Operator = {:?}, Operand = {:?}",
                self.coeffs, self.operator, self.operand
            ));
            ret
        }

        fn get_scope_string(&self) -> &Vec<String> {
            &self.scope_vec_str
        }

        fn get_scope(&self) -> &Vec<(String, &XDomainInteger)> {
            &self.scope_vec_var
        }
    }

    impl<'a> XSum<'a> {
        pub fn from_str(
            list: &str,
            condition: &str,
            coeffs: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope_vec_str = list_to_scope_ids(list);
            match set.construct_scope(&scope_vec_str) {
                Ok(scope) => {
                    let mut coe: Vec<i32>;
                    if coeffs.is_empty() {
                        coe = vec![];
                        for _ in 0..scope_vec_str.len() {
                            coe.push(1)
                        }
                    } else {
                        match list_to_values(coeffs) {
                            Ok(v) => coe = v,
                            Err(e) => return Err(e),
                        }
                    }
                    let condition = condition
                        .replace('(', " ")
                        .replace(')', " ")
                        .replace(',', " ");
                    let spilt: Vec<&str> = condition.split_whitespace().collect();
                    let ope: Operator;
                    match Operator::get_operator_by_str(spilt[0]) {
                        None => {
                            return Err(Xcsp3Error::get_constraint_sum_error(
                                "parse sum constraint error, ",
                            ))
                        }
                        Some(o) => ope = o,
                    }
                    let rand: Operand;

                    match Operand::get_operand_by_str(&spilt[1..], &ope) {
                        None => {
                            return Err(Xcsp3Error::get_constraint_sum_error(
                                "parse sum constraint error, ",
                            ))
                        }
                        Some(r) => rand = r,
                    }
                    Ok(Self::new(scope_vec_str, scope, ope, rand, coe))
                }
                Err(e) => Err(e),
            }
        }

        pub fn new(
            scope_vec_str: Vec<String>,
            scope_vec_var: Vec<(String, &'a XDomainInteger)>,
            operator: Operator,
            operand: Operand,
            coeffs: Vec<i32>,
        ) -> Self {
            Self {
                scope_vec_str,
                scope_vec_var,
                operator,
                operand,
                coeffs,
            }
        }

        pub fn get_coeffs(&self) -> &Vec<i32> {
            &self.coeffs
        }
        pub fn get_operand(&self) -> &Operand {
            &self.operand
        }

        pub fn get_operator(&self) -> &Operator {
            &self.operator
        }
    }
}
