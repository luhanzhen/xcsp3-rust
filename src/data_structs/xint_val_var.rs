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

    use regex::Regex;
    use std::fmt::{Display, Formatter};
    use std::str::FromStr;

    #[derive(Clone, Debug)]
    pub enum XVarVal {
        // IntVar(Variable<'a>),
        IntVar(String),
        IntVal(i32),
        IntInterval(i32, i32),
        IntArgument(i32),
        IntStart,
        IntNone,
    }

    impl Display for XVarVal {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    XVarVal::IntVar(v) => {
                        v.clone()
                    }
                    XVarVal::IntVal(v) => {
                        v.to_string()
                    }
                    XVarVal::IntInterval(l, r) => {
                        format!("{}..{}", l, r)
                    }
                    XVarVal::IntArgument(e) => {
                        format!("%{}", e)
                    }
                    XVarVal::IntStart => {
                        "%...".to_string()
                    }
                    XVarVal::IntNone => {
                        "".to_string()
                    }
                }
            )
        }
    }

    impl XVarVal {
        pub fn from_string(s: &str) -> Option<Self> {
            if s.contains('[') {
                Some(XVarVal::IntVar(String::from(s)))
            } else if s.is_empty() {
                None
            } else if s.contains('%') {
                if Regex::new(r"%(0|[1-9][0-9]*)").unwrap().is_match(s)
                //%num
                {
                    match i32::from_str(&s[1..]) {
                        Ok(e) => Some(XVarVal::IntArgument(e)),
                        Err(_) => None,
                    }
                } else if Regex::new(r"%([.]*)").unwrap().is_match(s)
                //%...
                {
                    Some(XVarVal::IntStart)
                } else {
                    None
                }
            } else {
                match i32::from_str(s) {
                    Ok(e) => Some(XVarVal::IntVal(e)),
                    Err(_) => Some(XVarVal::IntVar(String::from(s))),
                }
                // match i32::from_str(s) {
                //     Ok(e) => Some(XVarVal::IntVal(e)),
                //     Err(_) => {
                //         if Regex::new(r"%(0|[1-9][0-9]*)").unwrap().is_match(s)
                //         //%num
                //         {
                //             match i32::from_str(&s[1..]) {
                //                 Ok(e) => Some(XVarVal::IntArgument(e)),
                //                 Err(_) => None,
                //             }
                //         } else {
                //             if Regex::new(r"%([.]*)").unwrap().is_match(s)
                //             //%...
                //             {
                //                 Some(XVarVal::IntStart)
                //             } else {
                //                 Some(XVarVal::IntVar(String::from(s)))
                //
                //                 // }
                //             }
                //         }
                //     }
                // }
            }
        }
    }

    // #[derive(Clone)]
    // pub struct Variable<'a> {
    //     pub(crate) id: String,
    //     pub(crate) domain: Option<&'a XDomainInteger>,
    // }
    //
    // impl<'a> Variable<'a> {
    //     pub fn new(id: &str)->Self {
    //         Self
    //         {
    //             id:id.to_owned(),
    //         domain:None,
    //         }
    //     }
    //     pub fn set_domain(&mut self,domain : &'a XDomainInteger) {
    //         self.domain = Some(domain);
    //     }
    //
    //     pub fn get_domain(&self) -> &Option<&XDomainInteger> {
    //         &self.domain
    //     }
    //
    //     pub fn get_id(&self) -> &String {
    //         &self.id
    //     }
    //
    //     pub fn set_id(&self) -> &String {
    //         &self.id
    //     }
    // }
    //
    // impl Display for Variable<'_> {
    //     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    //         write!(f, "{}({})", self.id,  match self.domain
    //         {
    //             None => {"".to_string()}
    //             Some(d) => {d.to_string()}
    //         })
    //     }
    // }
}
