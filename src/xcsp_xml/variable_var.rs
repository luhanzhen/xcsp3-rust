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
    use serde::Deserialize;

    /**
    syntax.
    ```xml
     <var id="identifier"  [ type="integer" ]>
      ((intVal | intIntvl) wspace)*
    </var>
    ```

    eg.
    ```xml
    <var id="foo"> 0 1 2 3 4 5 6 </var>
    <var id="bar"> 0..6 </var>
    <var id="qux"> -6..-2 0 1..3 4 7 8..11 </var>
    ```
    */
    #[derive(Deserialize, Debug)]
    pub struct VariableVar {
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@type", default)]
        pub r#type: String,
        #[serde(rename = "$value", default)]
        pub value: String,
        #[serde(rename = "@as", default)]
        pub r#as: String,
    }
}
