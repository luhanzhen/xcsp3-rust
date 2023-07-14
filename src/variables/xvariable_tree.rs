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
    use crate::utils::xcsp3utils::xcsp3_core::{sizes_to_double_vec, sizes_to_vec};
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;

    use crate::variables::xvariable_trait::xcsp3_core::XVariableTrait;
    use crate::xcsp_xml::variable_domain::xcsp3_xml::VariableDomain;

    #[derive(Clone)]
    pub struct XVariableTree {
        nodes: Vec<XVariableTreeNode>,
        pub(crate) id: String,
        sizes: Vec<usize>,
        _size: usize,
    }

    impl XVariableTree {
        pub fn new(id: &str, sizes: &str, domain_vec: &[VariableDomain]) -> Option<Self> {
            if let Ok((size_vec, _size)) = sizes_to_vec(sizes) {
                let mut nodes: Vec<XVariableTreeNode> = Vec::new();
                for dom in domain_vec.iter() {
                    let ret = XDomainInteger::from_string(&dom.value);
                    if let Ok(domain) = ret {
                        if dom.r#for.eq("others") {
                            nodes.push(XVariableTreeNode::new_other(domain));
                        } else {
                            let for_strs: Vec<&str> = dom.r#for.split_whitespace().collect();
                            for e in for_strs.iter() {
                                let mut for_str = e.to_string();
                                for_str = for_str.replace(id, "");
                                for_str = for_str.replace("[]", "[*]");
                                match sizes_to_double_vec(for_str) {
                                    Ok((lower, upper)) => {
                                        nodes.push(XVariableTreeNode::new(
                                            lower,
                                            upper,
                                            domain.clone(),
                                        ));
                                    }
                                    Err(e) => {
                                        eprintln!("{}", e.to_string());
                                        return None;
                                    }
                                }
                            }
                        }
                    } else if ret.is_err() {
                        return None;
                    }
                }
                Some(XVariableTree {
                    id: id.to_string(),
                    sizes: size_vec,
                    _size,
                    nodes,
                })
            } else {
                None
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
        pub fn to_string(&self, id: &String) -> String {
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
