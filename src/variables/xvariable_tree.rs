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
* <p>@project_name: xcsp3-rust
* </p>
* <p>@author: luhan zhen
* </p>
* <p>@date:  2023/7/14 18:39
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
    use crate::utils::utils::xcsp3_utils::{
        get_all_variables_between_lower_and_upper, size_to_string, sizes_to_double_vec,
        sizes_to_vec,
    };
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;

    use crate::variables::xvariable_trait::xcsp3_core::XVariableTrait;

    #[derive(Clone)]
    pub struct XVariableTree {
        nodes: Vec<XVariableTreeNode>,
        others: XVariableTreeNode,
        pub(crate) id: String,
        sizes: Vec<usize>,
    }

    impl XVariableTree {
        /// return which node the variable belongs to
        fn get_node_by_vec(&self, v: &Vec<usize>) -> &XVariableTreeNode {
            for e in self.nodes.iter() {
                if e.belongs_to_this_node(&v) {
                    return &e;
                }
            }
            &self.others
        }

        pub fn find_variable(
            &self,
            id: &str,
        ) -> Result<Vec<(String, &XDomainInteger)>, Xcsp3Error> {
            let mut ret: Vec<(String, &XDomainInteger)> = vec![];
            // println!("{}", id);
            match id.find('[') {
                None => {
                    return Err(Xcsp3Error::get_variable_size_invalid_error(
                        "find_variable in XVariableTree error",
                    ));
                }
                Some(v) => match sizes_to_double_vec(&id[v..]) {
                    Ok((mut lower, mut upper)) => {
                        for i in 0..lower.len() {
                            if lower[i] == usize::MAX && upper[i] == usize::MAX {
                                lower[i] = 0;
                                upper[i] = self.sizes[i] - 1;
                            }
                        }
                        let all_variable = get_all_variables_between_lower_and_upper(lower, upper);
                        for size_vec in all_variable.iter() {
                            let node = self.get_node_by_vec(size_vec);
                            ret.push((size_to_string(&id[..v], size_vec), &node.domain));
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                },
            }

            //
            // if !id.contains("[]") {
            //     match id.find('[') {
            //         None => {}
            //         Some(v) => match sizes_to_vec(&id[v..]) {
            //             Ok((size_vec, _)) => {
            //                 let node = self.get_node_by_vec(size_vec);
            //                 ret.push((id.to_string(), &node.domain));
            //             }
            //             Err(e) => {
            //                 return Err(e);
            //             }
            //         },
            //     }
            // } else {
            //     let n = get_nth_square_of_name(id);
            //     for i in 0..self.sizes[n] {
            //         let mut s = id.to_string();
            //         s = s.replace("[]", format!("[{i}]").as_str());
            //         match s.find('[') {
            //             None => {}
            //             Some(v) => match sizes_to_vec(&s[v..]) {
            //                 Ok((size_vec, _)) => {
            //                     let node = self.get_node_by_vec(size_vec);
            //                     ret.push((s, &node.domain));
            //                 }
            //                 Err(e) => {
            //                     return Err(e);
            //                 }
            //             },
            //         }
            //     }
            // }
            return Ok(ret);
        }

        pub fn new(
            id: &str,
            sizes: &str,
            domain_for: Vec<&String>,
            domain_value: Vec<&String>,
        ) -> Result<Self, Xcsp3Error> {
            match sizes_to_vec(sizes) {
                Ok((size_vec, _)) => {
                    let mut nodes: Vec<XVariableTreeNode> = Vec::new();
                    // for dom in domain_vec.iter() {
                    let mut others_domain = Default::default();
                    for i in 0..domain_for.len() {
                        match XDomainInteger::from_string(domain_value[i]) {
                            Ok(domain) => {
                                if domain_for[i].eq("others") {
                                    others_domain = domain;
                                } else {
                                    let for_strs: Vec<&str> =
                                        domain_for[i].split_whitespace().collect();
                                    for e in for_strs.iter() {
                                        let for_str = e.to_string().replace(id, "");
                                        match sizes_to_double_vec(&for_str) {
                                            Ok((lower, upper)) => {
                                                nodes.push(XVariableTreeNode::new(
                                                    lower,
                                                    upper,
                                                    domain.clone(),
                                                ));
                                            }
                                            Err(e) => {
                                                eprintln!("{}", e.to_string());
                                                return Err(e);
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    Ok(XVariableTree {
                        id: id.to_string(),
                        sizes: size_vec,
                        others: XVariableTreeNode::new_other(others_domain),
                        nodes,
                    })
                }
                Err(e) => Err(e),
            }
        }
    }

    #[derive(Clone)]
    struct XVariableTreeNode {
        upper: Vec<usize>,
        lower: Vec<usize>,
        domain: XDomainInteger,
        is_other: bool,
    }

    impl XVariableTreeNode {
        pub fn belongs_to_this_node(&self, v: &Vec<usize>) -> bool {
            for (i, v) in v.iter().enumerate() {
                if self.lower[i] == usize::MAX && self.upper[i] == usize::MAX {
                    continue;
                } else if self.lower[i] <= *v && self.upper[i] >= *v {
                    continue;
                } else {
                    return false;
                }
            }
            true
        }
        pub fn to_string(&self, id: &str) -> String {
            let mut ret = format!("[for = {}", id);
            if self.is_other {
                ret.push_str("[others]..");
            } else {
                for i in 0..self.upper.len() {
                    ret.push('[');
                    if self.lower[i] == self.upper[i] {
                        if self.lower[i] == usize::MAX {
                            ret.push('*');
                        } else {
                            ret.push_str(&self.lower[i].to_string());
                        }
                    } else {
                        ret.push_str(&self.lower[i].to_string());
                        ret.push_str("..");
                        ret.push_str(&self.upper[i].to_string());
                    }

                    ret.push(']');
                }
            }

            ret.push_str("  domain = ");
            ret.push_str(&self.domain.to_string());
            ret
        }

        pub fn new(
            lower: Vec<usize>,
            upper: Vec<usize>,
            domain: XDomainInteger,
        ) -> XVariableTreeNode {
            XVariableTreeNode {
                upper,
                lower,
                domain,
                is_other: false,
            }
        }

        pub fn new_other(domain: XDomainInteger) -> XVariableTreeNode {
            XVariableTreeNode {
                upper: Vec::default(),
                lower: Vec::default(),
                domain,
                is_other: true,
            }
        }
    }

    impl XVariableTrait for XVariableTree {
        fn to_string(&self) -> String {
            let mut ret = format!("XVariableTree:  id = {}, sizes = ", self.id);
            for e in self.sizes.iter() {
                ret.push('[');
                ret.push_str(e.to_string().as_str());
                ret.push(']');
            }
            ret.push_str("nodes = ");
            for e in self.nodes.iter() {
                ret += &e.to_string(&self.id);
                ret += "]  ";
            }
            ret
        }
    }
}
