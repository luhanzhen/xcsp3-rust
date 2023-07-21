/*
 * <p>@project_name: xcsp3-rust
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/7/21 15:26
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

pub mod xcsp3_core {
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use std::fmt::{Display, Formatter};
    use std::str::FromStr;
    use regex::Regex;

    #[derive(Clone)]
    pub enum XVarVal<'a> {
        IntVar(Variable<'a>),
        IntVal(i32),
        IntInterval(i32,i32),
        IntArgument(i32),
        IntStart,
    }

    impl Display for XVarVal<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
           write!(f,"{}", match self
           {
               XVarVal::IntVar(v) => {v.to_string()}
               XVarVal::IntVal(v) => {v.to_string()}
               XVarVal::IntInterval(l, r) => {format!("{}..{}",l,r)}
               XVarVal::IntArgument(e) => {format!("%{}",e)}
               XVarVal::IntStart => {"%...".to_string()}
           })
        }
    }

    impl  XVarVal<'_> {
        pub fn from_string(s:&str)-> Option<Self> {
            match i32::from_str(s) {
                Ok(e) => {
                    Some(XVarVal::IntVal(e))
                }
                Err(_) => {

                    if Regex::new(r"%(0|[1-9][0-9]*)").unwrap().is_match(s) //%num
                    {

                        match i32::from_str(&s[1..]) {
                            Ok(e) => { Some(XVarVal::IntArgument(e))}
                            Err(_) => {
                                None
                            }
                        }
                    }else {
                        if  Regex::new(r"%([.]*)").unwrap().is_match(s) //%...
                        {
                            Some(XVarVal::IntStart)
                        }else {
                            if s.contains("..") //n1..n2
                            { let interval: Vec<&str> = s.split("..").collect();
                                match i32::from_str(interval[0]) {
                                    Ok(l) => match i32::from_str(interval[1]) {
                                        Ok(r) => {
                                            if l <= r {
                                                Some(XVarVal::IntInterval(l, r))
                                            } else {
                                                None
                                            }
                                        }
                                        Err(_) => {
                                            None
                                        }
                                    },
                                    Err(_) => {
                                        None
                                    }
                                }
                            }else {
                                Some(XVarVal::IntVar(Variable::new(s)))

                            }

                        }
                    }



                }
            }
        }

    }

    #[derive(Clone)]
    pub struct Variable<'a> {
        pub(crate) id: String,
        pub(crate) domain: Option<&'a XDomainInteger>,
    }

    impl<'a> Variable<'a> {
        pub fn new(id: &str)->Self {
            Self
            {
                id:id.to_owned(),
            domain:None,
            }
        }
        pub fn set_domain(&mut self,domain : &'a XDomainInteger) {
            self.domain = Some(domain);
        }

        pub fn get_domain(&self) -> &Option<&XDomainInteger> {
            &self.domain
        }

        pub fn get_id(&self) -> &String {
            &self.id
        }

        pub fn set_id(&self) -> &String {
            &self.id
        }
    }

    impl Display for Variable<'_> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}({})", self.id,  match self.domain
            {
                None => {"".to_string()}
                Some(d) => {d.to_string()}
            })
        }
    }
}
