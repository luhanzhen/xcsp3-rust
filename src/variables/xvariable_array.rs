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
* <p>@date:  2023/7/14 18:40
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 **/

pub mod xcsp3_core {
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::{
        get_all_variables_between_lower_and_upper, size_to_string, sizes_to_double_vec,
        sizes_to_vec,
    };
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;

    use crate::variables::xvariable_trait::xcsp3_core::XVariableTrait;

    #[derive(Clone)]
    pub struct XVariableArray {
        pub(crate) id: String,
        sizes: Vec<usize>,
        domain: XDomainInteger,
    }

    impl XVariableTrait for XVariableArray {
        fn to_string(&self) -> String {
            let mut ret: String = String::from("XVariableArray: id = ");
            ret.push_str(self.id.as_str());
            ret.push_str("  size = ");
            for e in self.sizes.iter() {
                ret.push('[');
                ret.push_str(e.to_string().as_str());
                ret.push(']');
            }
            ret.push_str(" domain = ");
            ret.push_str(self.domain.to_string().as_str());
            ret
        }
    }

    impl XVariableArray {
        pub fn find_variable(
            &self,
            id: &str,
        ) -> Result<Vec<(String, &XDomainInteger)>, Xcsp3Error> {
            // println!("{}", id);
            let mut ret: Vec<(String, &XDomainInteger)> = vec![];
            // println!("{}", id);
            match id.find('[') {
                None => {
                    return Err(Xcsp3Error::get_variable_size_invalid_error(
                        "find_variable in XVariableArray error",
                    ));
                }
                Some(v) => match sizes_to_double_vec(&id[v..]) {
                    Ok((mut lower, mut upper)) => {
                        for i in 0..lower.len() {
                            if lower[i] == usize::MAX && upper[i] == usize::MAX {
                                lower[i] = 0;
                                upper[i] = self.sizes[i] - 1;
                            }
                            if lower[i] > upper[i] || upper[i] >= self.sizes[i] {
                                return Err(Xcsp3Error::get_variable_size_invalid_error(
                                    "find_variable in XVariableArray error",
                                ));
                            }
                        }
                        let all_variable = get_all_variables_between_lower_and_upper(lower, upper);
                        for size_vec in all_variable.iter() {
                            ret.push((size_to_string(&id[..v], size_vec), &self.domain));
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                },
            }
            Ok(ret)
            // let mut ret: Vec<(String, &XDomainInteger)> = vec![];
            // if id.contains("[]") {
            //     let n = get_nth_square_of_name(id);
            //     for i in 0..self.sizes[n] {
            //         let mut s = id.to_string();
            //         s = s.replace("[]", format!("[{i}]").as_str());
            //         ret.push((s, &self.domain));
            //     }
            // } else {
            //     match id.find('[') {
            //         None => {}
            //         Some(v) => match sizes_to_vec(&id[v..]) {
            //             Ok((size_vec, _)) => {
            //                 if size_vec.len() == self.sizes.len() {
            //                     for (i, s) in size_vec.iter().enumerate() {
            //                         if *s >= self.sizes[i] {
            //                             return Err(Xcsp3Error::get_variable_size_invalid_error(
            //                                 "parse the size of variable error",
            //                             ));
            //                         }
            //                     }
            //                 } else {
            //                     return Err(Xcsp3Error::get_variable_size_invalid_error(
            //                         "parse the size of variable error",
            //                     ));
            //                 }
            //             }
            //             Err(e) => {
            //                 return Err(e);
            //             }
            //         },
            //     }
            //     ret.push((id.to_string(), &self.domain));
            // }
            // return Ok(ret);
        }

        pub fn new(id: &str, sizes: &str, domain: XDomainInteger) -> Result<Self, Xcsp3Error> {
            match sizes_to_vec(sizes) {
                Ok((size_vec, _)) => Ok(XVariableArray {
                    id: id.to_string(),
                    sizes: size_vec,
                    domain,
                }),
                Err(e) => Err(e),
            }
        }
    }
}

// println!("dim : {:?}", dim);
// for i in 0..size {
//     let mut iid = String::from(id);
//     let mut remain = i;
//
//     for d in dim.iter() {
//         // iid.push_str(format!("[{}]", remain / d).as_str());
//         iid.push_str("[");
//         iid.push_str((remain/d).to_string().as_str());
//         iid.push_str("]");
//         remain %= d;
//     }
// ret = format!("{}]", ret);
