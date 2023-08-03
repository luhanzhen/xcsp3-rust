// /*=============================================================================
// * parser for CSP instances represented in XCSP3 Format
// *
// * Copyright (c) 2023 xcsp.org (contact @ xcsp.org)
// *
// * Permission is hereby granted, free of charge, to any person obtaining a copy
// * of this software and associated documentation files (the "Software"), to deal
// * in the Software without restriction, including without limitation the rights
// * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// * copies of the Software, and to permit persons to whom the Software is
// * furnished to do so, subject to the following conditions:
// *
// * The above copyright notice and this permission notice shall be included in
// * all copies or substantial portions of the Software.
// *
// * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// * THE SOFTWARE.
// *=============================================================================
// */
//
// /*
//  * <p>@project_name: XCSP3-Rust
//  * </p>
//  * <p>@author: luhanzhen
//  * </p>
//  * <p>@date: 2023/7/15
//  * </p>
//  * <p>@time: 21:10
//  * </p>
//  * <p>@this_file_name: bitset.rs
//  * </p>
//  */
//
// #[allow(dead_code)]
// pub mod xcsp3_utils {
//
//     #[derive(Clone)]
//     pub struct Bitset {
//         words: Vec<u128>,
//         size: usize,
//     }
//
//     impl From<Vec<u128>> for Bitset {
//         fn from(value: Vec<u128>) -> Self {
//             let size = value.len() * 128usize;
//             Bitset::new(value, size)
//         }
//     }
//
//     pub struct IterBitSet<'a> {
//         current: usize,
//         bitset: &'a Bitset,
//     }
//
//     impl Iterator for IterBitSet<'_> {
//         type Item = usize;
//
//         fn next(&mut self) -> Option<Self::Item> {
//             match self.bitset.next_set_bit(self.current) {
//                 None => None,
//                 Some(c) => {
//                     self.current = c + 1;
//                     Some(c)
//                 }
//             }
//         }
//     }
//
//     impl Bitset {
//         pub fn iter(&self) -> IterBitSet {
//             IterBitSet {
//                 current: 0usize,
//                 bitset: self,
//             }
//         }
//         pub fn next_set_bit(&self, from: usize) -> Option<usize> {
//             if from >= self.size {
//                 return None;
//             }
//             let mut u: usize = from / 128usize;
//             let mut word: u128 = self.words[u] & (u128::MAX << (from % 128usize));
//             loop {
//                 if word != 0u128 {
//                     let t: usize = (u * 128usize) + word.trailing_zeros() as usize;
//                     return Some(t);
//                 }
//                 u += 1;
//                 if u == self.words.len() {
//                     return None;
//                 }
//                 word = self.words[u];
//             }
//         }
//
//         pub fn remove(&mut self, index: usize) {
//             let tmp: u128 = 1u128 << (index % 128usize);
//             self.words[index / 128usize] &= !tmp
//         }
//         pub fn flip(&mut self) {
//             for e in self.words.iter_mut() {
//                 let t = !(*e);
//                 *e = t;
//             }
//         }
//
//         pub fn clear(&mut self) {
//             for e in self.words.iter_mut() {
//                 *e = 0u128;
//             }
//         }
//
//         pub fn count(&mut self) -> usize {
//             let mut cnt: usize = 0;
//             for e in self.words.iter() {
//                 cnt += e.count_ones() as usize;
//             }
//             cnt
//         }
//
//         pub fn add(&mut self, index: usize) {
//             self.words[index / 128usize] |= 1u128 << (index % 128usize)
//         }
//
//         // pub fn to_string(&self) -> String {
//         //     let mut ret = String::default();
//         //     for e in self.words.iter()
//         //     {
//         //         ret.push_str(&format!("{:b}  ", e));
//         //     }
//         //     ret
//         // }
//
//         pub fn to_string(&self) -> String {
//             let mut ret = String::default();
//             for e in self.iter() {
//                 ret.push_str(&format!("{} ", e));
//             }
//             ret
//         }
//
//         pub fn from_width(max: usize) -> Self {
//             let mut values: Vec<u128> = vec![];
//             let mut width = max / 128;
//             if max % 128 != 0 {
//                 width += 1
//             }
//             values.resize(width, 0);
//             Bitset::new(values, max)
//         }
//
//         pub fn new(values: Vec<u128>, size: usize) -> Self {
//             Bitset {
//                 words: values,
//                 size,
//             }
//         }
//     }
// }
