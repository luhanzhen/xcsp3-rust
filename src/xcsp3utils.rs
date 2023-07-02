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
 * <p/>
 * <p>@author: luhanzhen
 * <p/>
 * <p>@date: 2023/6/30
 * <p/>
 * <p>@time: 13:55
 * <p/>
 * <p>@this_file_name:xcsp3domain
 * <p/>
 */
#[allow(dead_code)]
pub mod xcsp3_core {

    pub fn split_with_elems(s: &String, delim: &char, elems: &mut Vec<String>) {
        let de = format!("{}", delim);
        let result: Vec<&str> = s.split(de.as_str()).collect();
        for elem in result {
            elems.push(String::from(elem));
        }
    }

    pub fn split(s: &String, delim: &char) -> Vec<String> {
        let mut elems: Vec<String> = vec![];
        split_with_elems(s, delim, &mut elems);
        elems
    }

    pub fn replace_string_in_place(subject: &mut String, search: &String, replace: &String) {
        let _ = subject.replace(search, replace);
    }

    pub fn remove_char(s: &mut String, c: char) {
        let _ = s.replace(c, "");
    }
}
