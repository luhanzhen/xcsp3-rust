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
 * <p>@project_name: XCSP3-Rust
 * </p>
 * <p>@author: luhanzhen
 * </p>
 * <p>@date: 2023/6/30
 * </p>
 * <p>@time: 13:55
 * </p>
 * <p>@this_file_name:xcsp3domain
 * </p>
 */

pub mod xcsp3_core {

    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use std::str::FromStr;

    /// return the list of scopes,
    /// eg str"x[1], x[3], x[5]" - > vec[x[1], x[3], x[5]]
    pub fn list_to_scope_ids(list: &str) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        let lists: Vec<&str> = list.split_whitespace().collect();
        for e in lists.iter() {
            ret.push(e.to_string());
        }
        ret
    }

    /// return the matrix,
    /// eg str"(x1,x2,x3,x4,x5)(y1,y2,y3,y4,y5)(z1,z2,z3,z4,z5)" - > vec[[x1,x2,x3,x4,x5][y1,y2,y3,y4,y5][z1,z2,z3,z4,z5]]
    pub fn list_to_matrix_ids(list: &str) -> Vec<Vec<String>> {
        let mut ret: Vec<Vec<String>> = Vec::new();
        let mut list = list.to_string();
        list = list.replace(')', "@");
        list = list.replace('\n', "");
        list = list.replace("(", " ");
        let lists: Vec<&str> = list.split("@").collect();
        for e in lists.iter() {
            if !e.is_empty() {
                let ss = e.replace(',', " ");
                ret.push(list_to_scope_ids(&ss));
            }
        }
        ret
    }

    /// return the list of values,
    /// eg str"1 3 5 76" -> vec[1,3,5,76],
    pub fn list_to_values(list: &str) -> Result<Vec<i32>, Xcsp3Error> {
        let mut ret: Vec<i32> = Vec::new();
        let lists: Vec<&str> = list.split_whitespace().collect();
        for e in lists.iter() {
            match i32::from_str(e) {
                Ok(n) => ret.push(n),
                Err(_) => {
                    return Err(Xcsp3Error::get_constraint_list_of_values_error(
                        "parse the list of values error. ",
                    ));
                }
            }
        }
        Ok(ret)
    }

    /// return the list of values,
    /// eg str"(1, 3, 5, 76)" -> vec[1,3,5,76],
    pub fn list_with_bracket_comma_to_values(list: &str) -> Result<Vec<i32>, Xcsp3Error> {
        let mut ret: Vec<i32> = Vec::new();
        let mut list = list.to_string();
        list = list.replace('(', " ");
        list = list.replace(')', " ");
        list = list.replace(',', " ");
        let lists: Vec<&str> = list.split_whitespace().collect();

        for e in lists.iter() {
            match i32::from_str(e) {
                Ok(n) => ret.push(n),
                Err(_) => {
                    return Err(Xcsp3Error::get_constraint_list_of_values_error(
                        "parse the list of values error. ",
                    ));
                }
            }
        }
        Ok(ret)
    }

    ///return the tuples by given string,
    /// eg (0,0,1)(0,1,0)(1,0,0)(1,1,1) -> [[0,0,1],[0,1,0],[1,0,0],[1,1,1]]
    pub fn tuple_to_vector(tuple: &str) -> Result<Vec<Vec<i32>>, Xcsp3Error> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let tuple = tuple.replace('(', " ");
        let tuple = tuple.replace(')', " ");
        let tuples_str: Vec<&str> = tuple.split_whitespace().collect();
        for ts in tuples_str.iter() {
            let tuple_str: Vec<&str> = ts.split(',').collect();
            let mut tt: Vec<i32> = Vec::new();
            for i in tuple_str.iter() {
                match i32::from_str(i) {
                    Ok(num) => {
                        tt.push(num);
                    }
                    Err(_) => {
                        return Err(Xcsp3Error::get_constraint_extension_error(
                            "parse the tuple of extension error",
                        ));
                    }
                }
            }
            ret.push(tt);
        }
        Ok(ret)
    }

    /// transform the string size to vector sizes
    /// eg:  [2][3..4][4..8] -> ([2,3,4],[2][4][8])
    pub fn sizes_to_double_vec(sizes: String) -> Result<(Vec<usize>, Vec<usize>), Xcsp3Error> {
        let mut lower: Vec<usize> = vec![];
        let mut upper: Vec<usize> = vec![];

        let mut sizes = sizes.replace('[', " ");
        sizes = sizes.replace(']', " ");
        let nums: Vec<&str> = sizes.split_whitespace().collect();
        for n in nums.iter() {
            if n.find('*').is_some() {
                lower.push(usize::MAX);
                upper.push(usize::MAX);
            } else if n.contains("..") {
                let interval: Vec<&str> = n.split("..").collect();
                if interval.len() == 2 {
                    let low = usize::from_str(interval[0]);
                    let up = usize::from_str(interval[1]);
                    match low {
                        Ok(l) => match up {
                            Ok(u) => {
                                lower.push(l);
                                upper.push(u);
                            }
                            Err(_) => {
                                return Err(Xcsp3Error::get_domain_for_error(
                                    "parse the domain for error",
                                ));
                            }
                        },
                        Err(_) => {
                            return Err(Xcsp3Error::get_domain_for_error(
                                "parse the domain for error",
                            ));
                        }
                    }
                }
            } else {
                match usize::from_str(n) {
                    Ok(v) => {
                        lower.push(v);
                        upper.push(v);
                    }
                    Err(_) => {
                        return Err(Xcsp3Error::get_domain_for_error("parse the domain error"));
                    }
                };
            }
        }
        Ok((lower, upper))
    }

    /// transform the string size to vector sizes
    /// eg:  [2][3][4] -> ([2,3,4], 24)
    pub fn sizes_to_vec(sizes: &str) -> Result<(Vec<usize>, usize), Xcsp3Error> {
        let mut ret: Vec<usize> = vec![];
        let mut sz: usize = 1;
        let mut sizes = sizes.replace('[', " ");
        sizes = sizes.replace(']', " ");
        let nums: Vec<&str> = sizes.split_whitespace().collect();
        for n in nums.iter() {
            match usize::from_str(n) {
                Ok(v) => {
                    ret.push(v);
                    sz *= v;
                }
                Err(_) => {
                    return Err(Xcsp3Error::get_variable_size_invalid_error(
                        "parse the size of variable error",
                    ));
                }
            };
        }

        Ok((ret, sz))
    }
}

// #[allow(dead_code)]
// pub mod xcsp3_core {
//
//     pub fn split_with_elems(s: &str, delim: &char, elems: &mut Vec<String>) {
//         let de = format!("{}", delim);
//         let result: Vec<&str> = s.split(de.as_str()).collect();
//         for elem in result {
//             elems.push(String::from(elem));
//         }
//     }
//
//     // pub fn parser_domain(domain: &String) -> XDomainInteger
//     // {
//     //     let mut ret:XDomainInteger = XDomainInteger::new();
//     //     let domains :Vec<&str>= domain.split_whitespace().collect();
//     //
//     //     for d in domains.iter()
//     //     {
//     //         if d.find("..").is_some()
//     //         {
//     //             let interval: Vec<&str> = d.split("..").collect();
//     //             if interval.len() == 2 {
//     //                 let left = i32::from_str(interval[0]);
//     //                 let right = i32::from_str(interval[1]);
//     //                 match left {
//     //                     Ok(l) => {
//     //                         match right {
//     //                             Ok(r) => {
//     //                                 ret.add_interval(l,r);
//     //                             }
//     //                             Err(_) => {}
//     //                         }
//     //                     }
//     //                     Err(_) => {}
//     //                 }
//     //             }
//     //
//     //
//     //         }else {
//     //              match  i32::from_str(d){
//     //                 Ok(v) => {
//     //                     ret.add_value(v)
//     //                 }
//     //                 Err(_) => { }
//     //             } ;
//     //         }
//     //     }
//     //
//     //     ret
//     // }
//
//     pub fn split(s: &str, delim: &char) -> Vec<String> {
//         let mut elems: Vec<String> = vec![];
//         split_with_elems(s, delim, &mut elems);
//         elems
//     }
//
//     pub fn replace_string_in_place(subject: &mut str, search: &str, replace: &str) {
//         let _ = subject.replace(search, replace);
//     }
//
//     pub fn remove_char(s: &mut str, c: char) {
//         let _ = s.replace(c, "");
//     }
// }
