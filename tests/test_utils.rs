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
 * <p>@time: 21:14
 * </p>
 * <p>@this_file_name: test_utils
 * </p>
 */

#[cfg(test)]
mod test_xcsp3domain {
    // use bitmaps::Bitmap;
    use xcsp3_rust::utils::bitset::xcsp3_core::Bitset;

    #[test]
    fn test_bitset() {
        let mut set = Bitset::from_width(1000);
        set.remove(9);
        println!("set:  {}", set.to_string());

        assert_eq!(set.count(), 0);
        set.add(300);
        set.add(400);
        println!("set:  {}", set.to_string());
        assert_eq!(set.count(), 2);
        // set.flip();
        print!("set: ");
        for e in set.iter() {
            print!(" {}  ", e);
        }

        // let mut bit: Bitmap<1024> = Bitmap::new();
    }
}
