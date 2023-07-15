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
* <p>@date:  2023/7/14 20:47
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 **/

pub mod xcsp3_xml {
    use crate::xcsp_xml::variable_domain::xcsp3_xml::VariableDomain;
    use serde::Deserialize;

    /**
        syntax.
        ```xml
         <array id="identifier"  [ type="varType" ]  size="dimensions"  [ startIndex="integer" ]>
          ...
        </array>
        ```

        eg.
        ```xml
        <array id="x" size="[10]"> 1..100 </array>
        <array id="y" size="[5][8]"> 0 1 </array>
        ```

        or
        syntax.
        ```xml
          <array id="identifier"  [ type="varType" ]  size="dimensions"  [ startIndex="integer" ]>
            (<domain for="(intVar wspace)+"> ... </domain>)+
            [<domain for="others"> ... </domain>]
         </array>
        ```

        eg.
        ```xml
       <array id="x" size="[3][5]">
      <domain for="x[0][]"> 1..10 </domain>
      <domain for="x[1][]"> 1..20 </domain>
      <domain for="x[2][]"> 1..15 </domain>
    </array>
    <array id="y" size="[10]">
      <domain for="y[4]"> 0 1 </domain>
      <domain for="others"> 2 4 6 </domain>
    </array>
    <array id="z" size="[5][5][5]">
      <domain for="z[][0..1][] z[][2][2..4]"> 0..10 </domain>
      <domain for="others"> 0 1 </domain>
    </array>
        ```
     */
    #[derive(Deserialize, Debug)]
    pub struct VariableArray {
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@type", default)]
        pub type_: String,
        #[serde(rename = "@startIndex", default)]
        pub start_index: String,
        #[serde(rename = "@note", default)]
        pub note: String,
        #[serde(rename = "@size", default)]
        pub size: String,
        #[serde(rename = "$value", default)]
        pub value: String,
        #[serde(rename = "domain", default)]
        pub domains: Vec<VariableDomain>,
    }
}
