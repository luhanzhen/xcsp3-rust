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

pub mod xcsp3_utils {
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    // use std::str::FromStr;

    pub fn list_to_vec_var_val(list: &str) -> Result<Vec<XVarVal>, Xcsp3Error> {
        let mut ret: Vec<XVarVal> = vec![];
        let lists: Vec<&str> = list.split_whitespace().collect();
        for e in lists.iter() {
            match XVarVal::from_string(e) {
                None => {
                    return Err(Xcsp3Error::get_constraint_list_of_values_error(
                        "parse the list of values error. ",
                    ))
                }
                Some(vv) => {
                    ret.push(vv);
                }
            }
        }
        Ok(ret)
    }

    // pub fn list_to_vec_var(list: &str) -> Vec<XVarVal> {
    //     let mut ret: Vec<XVarVal> = vec![];
    //     let lists: Vec<&str> = list.split_whitespace().collect();
    //     for e in lists.into_iter() {
    //         ret.push(XVarVal::IntVar(e.to_owned()));
    //
    //     }
    //     ret
    // }

    pub fn size_to_string(id: &str, size: &[usize]) -> String {
        let mut ret = id.to_string();

        for e in size.iter() {
            ret.push_str(&format!("[{}]", e))
        }
        ret
    }

    ///([2,3,4],[2,4,8]) -> [[2,3,4],[2,3,5],[2,3,6],[2,3,7],[2,3,8],[2,4,4],[2,4,5],[2,4,6],[2,4,7],[2,4,8]]
    pub fn get_all_variables_between_lower_and_upper(
        lower: Vec<usize>,
        upper: Vec<usize>,
    ) -> Vec<Vec<usize>> {
        let mut ret: Vec<Vec<usize>> = vec![];
        let mut tmp: Vec<Vec<usize>> = vec![];
        // println!("{:?},{:?}", lower, upper);

        // recursion_for_get_all_variables(0,&lower,&upper,&ret);
        for i in lower[0]..upper[0] + 1 {
            tmp.push(vec![i]);
        }
        for deep in 1..lower.len() {
            for i in lower[deep]..upper[deep] + 1 {
                for e in tmp.iter() {
                    let mut ee = e.clone();
                    ee.push(i);
                    ret.push(ee)
                }
            }
            tmp = ret.clone();
            ret.clear();
        }
        // println!("ret = {:?}\n", ret);
        // println!("tmp = {:?}\n", tmp);
        tmp
    }

    /// return the nth of the [] in id(str)
    /// eg x[2][5][] -> 2,  y[] -> 0, z[3][] ->1, zzz[4][][4] ->1
    pub fn get_nth_square_of_name(id: &str) -> usize {
        match id.find("[]") {
            None => 0,
            Some(v) => {
                let mut cnt: usize = 0;
                for (_, c) in id[..v].chars().enumerate() {
                    if c == '[' {
                        cnt += 1
                    }
                }
                cnt
            }
        }
    }

    /// return the list of scopes,
    /// eg str"x[1] x[3] x[5]" - > vec[x[1], x[3], x[5]]
    pub fn list_to_scope_ids(list: &str) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        let lists: Vec<&str> = list.split_whitespace().collect();
        for e in lists.iter() {
            ret.push(e.to_string());
        }
        ret
    }

    /// return the transitions,
    /// eg  "(a,0,a)(a,1,b)(b,1,c)(c,0,d)(d,0,d)(d,1,e)(e,0,e)" -> vec[ (a,0,a),(a,1,b),(b,1,c),(c,0,d),(d,0,d),(d,1,e),(e,0,e)]
    pub fn list_to_transitions(list: &str) -> Result<Vec<(String, i32, String)>, Xcsp3Error> {
        let mut ret: Vec<(String, i32, String)> = Vec::new();
        let chars = list.chars();
        if let Some(left) = list.find('(') {
            if let Some(right) = list.find(')') {
                ret.reserve(list.len() / (right - left));
                // n = (right - left - 1) / 2;
            }
        }
        let mut last = usize::MAX;
        let mut last_comma1 = usize::MAX;
        let mut last_comma2 = usize::MAX;
        for (i, x) in chars.enumerate() {
            if x == '(' {
                last = i;
            } else if x == ')' {
                // println!("{}",&tuple_str[last+1..i]);
                match list[last_comma1 + 1..last_comma2].parse::<i32>() {
                    Ok(num) => ret.push((
                        list[last + 1..last_comma1].to_string(),
                        num,
                        list[last_comma2 + 1..i].to_string(),
                    )),
                    Err(_) => {
                        return Err(Xcsp3Error::get_constraint_regular_transitions_error(
                            "parse the transitions error, ",
                        ));
                    }
                }
                last_comma1 = usize::MAX;
            } else if x == ',' {
                if last_comma1 == usize::MAX {
                    last_comma1 = i;
                } else {
                    last_comma2 = i;
                }
            }
        }
        Ok(ret)
    }

    /// return the matrix,
    /// eg str"(x1,x2,x3,x4,x5)(y1,y2,y3,y4,y5)(z1,z2,z3,z4,z5)" - > vec[[x1,x2,x3,x4,x5][y1,y2,y3,y4,y5][z1,z2,z3,z4,z5]]
    pub fn list_to_matrix_ids(list: &str) -> Vec<Vec<String>> {
        let mut ret: Vec<Vec<String>> = Vec::new();
        let list = list
            .to_string()
            .replace(')', "@")
            .replace('\n', "")
            .replace('(', " ");
        let lists: Vec<&str> = list.split('@').collect();
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
        for l in lists.iter() {
            match l.parse::<i32>() {
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
    pub fn list_with_bracket_comma_to_values(list: &str) -> Result<Vec<XVarVal>, Xcsp3Error> {
        let mut ret: Vec<XVarVal> = Vec::new();
        let list = list.to_string().replace(['(', ')', ','], " ");
        let lists: Vec<&str> = list.split_whitespace().collect();
        for e in lists.iter() {
            match e.parse::<i32>() {
                Ok(n) => ret.push(XVarVal::IntArgument(n)),
                Err(_) => {
                    return Err(Xcsp3Error::get_constraint_list_of_values_error(
                        "parse the list of values error. ",
                    ));
                }
            }
        }
        Ok(ret)
    }

    // fn string_to_i32(s: &str) -> Option<i32> {
    //     let char = s.chars().rev();
    //     let mut n: i32 = 0;
    //     for (i, c) in char.enumerate() {
    //         if !c.is_numeric() {
    //             return None;
    //         } else {
    //             n += 10i32.pow((i as i32).try_into().unwrap()) * c as i32
    //         }
    //     }
    //     Some(n)
    // }

    pub fn tuple_to_vector1(tuple_str: &str, is_unary: bool) -> Result<Vec<Vec<i32>>, Xcsp3Error> {
        // let ti = TimeInterval::new();
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let err = Xcsp3Error::get_constraint_extension_error("parse the tuple of extension error");
        if is_unary {
            let tuples: Vec<&str> = tuple_str.split_whitespace().collect();
            for tuple in tuples.iter() {
                if tuple.contains("..") {
                    let interval: Vec<&str> = tuple.split("..").collect();
                    if interval.len() == 2 {
                        let left = interval[0].parse::<i32>();
                        let right = interval[1].parse::<i32>();
                        match left {
                            Ok(l) => match right {
                                Ok(r) => {
                                    if l <= r {
                                        for i in l..r + 1 {
                                            ret.push(vec![i])
                                        }
                                    } else {
                                        return Err(err);
                                    }
                                }
                                Err(_) => {
                                    return Err(err);
                                }
                            },
                            Err(_) => {
                                return Err(err);
                            }
                        }
                    }
                } else {
                    match tuple.parse::<i32>() {
                        Ok(v) => ret.push(vec![v]),
                        Err(_) => {
                            return Err(err);
                        }
                    }
                }
            }
        } else {
            // let chars = tuple_str.chars();

            // if let Some(left) = tuple_str.find('(') {
            //     if let Some(right) = tuple_str.find(')') {
            //
            //     }
            // }
            let mut rightbracket = 0usize;
            let mut comma = 0usize;
            for x in tuple_str.chars() {
                if x == ')' {
                    rightbracket += 1
                } else if x == ',' {
                    comma += 1
                }
            }
            let mut tt: Vec<i32> = vec![];
            tt.resize((comma / rightbracket) + 1, i32::MAX);
            ret.resize(rightbracket, tt);
            let mut rows = 0usize;
            let mut cols = 0usize;

            let mut last = 0usize;
            for (i, x) in tuple_str.chars().enumerate() {
                if x == '(' {
                    // tt.clear();
                    // tt.reserve(n);

                    last = i;
                } else if x == ')' {
                    // println!("{}",&tuple_str[last+1..i]);
                    match tuple_str[last + 1..i].parse::<i32>() {
                        Ok(num) => {
                            let tmp = &mut ret[rows][cols];
                            *tmp = num;
                        }
                        Err(_) => {
                            if &tuple_str[last + 1..i] != "*" {
                                return Err(err);
                            }
                        }
                    }
                    cols = 0;
                    // ret.push(tt.clone())
                    rows += 1;
                } else if x == ',' {
                    // println!("{}",&tuple_str[last+1..i]);
                    match tuple_str[last + 1..i].parse::<i32>() {
                        Ok(num) => {
                            let tmp = &mut ret[rows][cols];
                            *tmp = num;
                        }
                        Err(_) => {
                            if &tuple_str[last + 1..i] != "*" {
                                return Err(err);
                            }
                        }
                    }
                    cols += 1;
                    last = i;
                }
            }
        }
        // println!("parse Extension {:?}",ti.get());
        Ok(ret)
    }
    ///return the tuples by given string,
    /// eg (0,0,1)(0,1,0)(1,0,0)(1,1,1) -> [[0,0,1],[0,1,0],[1,0,0],[1,1,1]]
    pub fn tuple_to_vector(tuple_str: &str, is_unary: bool) -> Result<Vec<Vec<i32>>, Xcsp3Error> {
        // let ti = TimeInterval::new();
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let err = Xcsp3Error::get_constraint_extension_error("parse the tuple of extension error");
        if is_unary {
            let tuples: Vec<&str> = tuple_str.split_whitespace().collect();
            for tuple in tuples.iter() {
                if tuple.contains("..") {
                    let interval: Vec<&str> = tuple.split("..").collect();
                    if interval.len() == 2 {
                        let left = interval[0].parse::<i32>();
                        let right = interval[1].parse::<i32>();
                        match left {
                            Ok(l) => match right {
                                Ok(r) => {
                                    if l <= r {
                                        for i in l..r + 1 {
                                            ret.push(vec![i])
                                        }
                                    } else {
                                        return Err(err);
                                    }
                                }
                                Err(_) => {
                                    return Err(err);
                                }
                            },
                            Err(_) => {
                                return Err(err);
                            }
                        }
                    }
                } else {
                    match tuple.parse::<i32>() {
                        Ok(v) => ret.push(vec![v]),
                        Err(_) => {
                            return Err(err);
                        }
                    }
                }
            }
        } else {
            let chars = tuple_str.chars();
            let mut last = 0;
            let mut tt: Vec<i32> = vec![];
            let mut n: usize = 0;
            if let Some(left) = tuple_str.find('(') {
                if let Some(right) = tuple_str.find(')') {
                    ret.reserve(tuple_str.len() / (right - left));
                    n = (right - left - 1) / 2;
                }
            }

            for (i, x) in chars.enumerate() {
                if x == '(' {
                    tt.clear();
                    tt.reserve(n);
                    last = i;
                } else if x == ')' {
                    // println!("{}",&tuple_str[last+1..i]);
                    match tuple_str[last + 1..i].parse::<i32>() {
                        Ok(num) => {
                            tt.push(num);
                        }
                        Err(_) => {
                            if &tuple_str[last + 1..i] == "*" {
                                tt.push(i32::MAX);
                            } else {
                                return Err(err);
                            }
                        }
                    }
                    ret.push(tt.clone())
                } else if x == ',' {
                    // println!("{}",&tuple_str[last+1..i]);
                    match tuple_str[last + 1..i].parse::<i32>() {
                        Ok(num) => {
                            tt.push(num);
                        }
                        Err(_) => {
                            if &tuple_str[last + 1..i] == "*" {
                                tt.push(i32::MAX);
                            } else {
                                return Err(err);
                            }
                        }
                    }
                    last = i;
                }
            }
            // let chars = tuple_str.chars();
            // let mut last = 0;
            //
            // if let Some(left) = tuple_str.find('(')
            // {
            //     if let Some(right) = tuple_str.find(')')
            //     {
            //         ret.reserve(tuple_str.len()/(right-left));
            //         let mut tt: Vec<i32> = vec![];
            //         tt.resize((right-left)/2,0);
            //         ret.resize(tuple_str.len()/(right-left),tt);
            //     }
            // }
            // let mut tuple_index = 0;
            // let mut tuple_in_index = 0;
            // for (i, x) in chars.enumerate() {
            //     if x == '(' {
            //         // tt.clear();
            //         tuple_in_index = 0;
            //         last = i;
            //     } else if x == ')' {
            //         // println!("{}",&tuple_str[last+1..i]);
            //         match i32::from_str(&tuple_str[last + 1..i]) {
            //             Ok(num) => {
            //                 // tt.push(num);
            //                 ret[tuple_index][tuple_in_index] = num;
            //                 tuple_in_index+=1;
            //             }
            //             Err(_) => {
            //                 if &tuple_str[last + 1..i] == "*" {
            //                     // tt.push(i32::max);
            //                     ret[tuple_index][tuple_in_index] = i32::max;
            //                     tuple_in_index+=1;
            //                 } else {
            //                     return Err(Xcsp3Error::get_constraint_extension_error(
            //                         "parse the tuple of extension error",
            //                     ));
            //                 }
            //             }
            //         }
            //         tuple_index += 1;
            //         // ret.push(tt.clone())
            //     } else if x == ',' {
            //         // println!("{}",&tuple_str[last+1..i]);
            //         match i32::from_str(&tuple_str[last + 1..i]) {
            //             Ok(num) => {
            //                 ret[tuple_index][tuple_in_index] = num;
            //                 tuple_in_index+=1;
            //             }
            //             Err(_) => {
            //                 if &tuple_str[last + 1..i] == "*" {
            //                     ret[tuple_index][tuple_in_index] = i32::max;
            //                     tuple_in_index+=1;
            //                 } else {
            //                     return Err(Xcsp3Error::get_constraint_extension_error(
            //                         "parse the tuple of extension error",
            //                     ));
            //                 }
            //             }
            //         }
            //         last = i;
            //     }
            // }
        }
        // println!("parse Extension {:?}",ti.get());
        Ok(ret)
    }

    /// transform the string size to vector sizes
    /// eg:  [2][3..4][4..8] -> ([2,3,4],[2,4,8])
    pub fn sizes_to_double_vec(sizes: &str) -> Result<(Vec<usize>, Vec<usize>), Xcsp3Error> {
        let mut lower: Vec<usize> = vec![];
        let mut upper: Vec<usize> = vec![];
        let sizes = sizes.replace("[]", "[*]").replace(['[', ']'], " ");
        let nums: Vec<&str> = sizes.split_whitespace().collect();
        for n in nums.iter() {
            if n.find('*').is_some() {
                lower.push(usize::MAX);
                upper.push(usize::MAX);
            } else if n.contains("..") {
                let interval: Vec<&str> = n.split("..").collect();
                if interval.len() == 2 {
                    let low = interval[0].parse::<usize>();
                    let up = interval[1].parse::<usize>();

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
                match n.parse::<usize>() {
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
            match n.parse::<usize>() {
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
