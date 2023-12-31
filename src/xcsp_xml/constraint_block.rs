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
* <p>@project_name: xcsp3-rust
* </p>
* <p>@author: luhan zhen
* </p>
* <p>@date:  2023/7/14 20:45
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 */

pub mod xcsp3_xml {
    use crate::xcsp_xml::constraint_type::xcsp3_xml::ConstraintType;
    use serde::Deserialize;
    /**
    syntax.
    ```xml
    <block  [ id="identifier" ]  [ class="(identifier wspace)+" ]>
      (<constraint.../> | <metaConstraint.../> | <group.../> | <block... />)+
    </block>
    ```

    eg.
    ```xml
    <block class="clues">
        <intension> ... </intension>
        <intension> ... </intension>
        ...
      </block>
      <block class="symmetryBreaking">
        <lex> ... </lex>
        <lex> ... </lex>
        ...
      </block>
      <block class="redundantConstraints"> ... </block>
      <block note="Management of first week"> ... </block>
      <block note="Management of second week"> ... </block>
    ```
     */
    #[derive(Deserialize, Debug)]
    pub struct ConstraintBlock {
        #[serde(rename = "@id", default)]
        pub id: String,
        #[serde(rename = "@note", default)]
        pub note: String,
        #[serde(rename = "@class", default)]
        pub r#type: String,
        #[serde(rename = "$value", default)]
        pub constraints: Vec<ConstraintType>,
    }
}
