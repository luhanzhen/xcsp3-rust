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
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use std::collections::HashSet;
    #[derive(Clone)]
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
    #[derive(Clone)]
    pub enum Operand {
        Integer(i32),
        Variable(String),
        Interval(i32, i32),
        SetInteger(HashSet<i32>),
    }

    #[derive(Clone)]
    pub struct XSum<'a> {
        scope_vec_str: Vec<String>,
        scope_vec_var: Vec<(String, &'a XDomainInteger)>,
        operator: Operator,
        operand: Operand,
    }

    impl XConstraintTrait for XSum<'_> {
        fn to_string(&self) -> String {
            let mut ret = format!("XSum: ",);
            ret
        }

        fn get_scope_string(&self) -> &Vec<String> {
            &self.scope_vec_str
        }

        fn get_scope(&self) -> &Vec<(String, &XDomainInteger)> {
            &self.scope_vec_var
        }
    }

    impl<'a> XSum<'a> {}
}
