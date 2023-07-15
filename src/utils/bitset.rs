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
 * <p>@date: 2023/7/15
 * </p>
 * <p>@time: 21:10
 * </p>
 * <p>@this_file_name: bitset.rs
 * </p>
 */

pub mod xcsp3_core {

    #[derive(Clone)]
    pub struct Bitset {
        values: Vec<u128>,
    }

    impl From<Vec<u128>> for Bitset {
        fn from(value: Vec<u128>) -> Self {
            Bitset::new(value)
        }
    }

    impl Bitset {
        pub fn remove(&self, index: usize) -> usize {
            let mut cnt: usize = 0;
            for e in self.values.iter() {
                cnt += e.count_zeros() as usize
            }
            cnt
        }

        pub fn add(index: usize) {}

        pub fn to_string(&self) -> String {
            format!("{:?}", self.values)
        }

        pub fn from_width(max: usize) -> Self {
            let mut values: Vec<u128> = vec![];
            let mut width = max / 128;
            if max % 128 != 0 {
                width += 1
            }
            values.resize(width, 0);
            Bitset::new(values)
        }

        pub fn new(values: Vec<u128>) -> Self {
            Bitset { values }
        }
    }
}
