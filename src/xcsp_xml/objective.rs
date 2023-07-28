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
* <p>@date:  2023/7/14 19:13
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 */

pub mod xcsp3_xml {
    use std::fmt::{Display, Formatter};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub enum ObjectiveTarget {
        #[serde(rename = "sum")]
        Sum,
        #[serde(rename = "product")]
        Product,
        #[serde(rename = "minimum")]
        Minimum,
        #[serde(rename = "maximum")]
        Maximum,
        #[serde(rename = "nvalues")]
        Nvalues,
        #[serde(rename = "lex")]
        Lex,
        None,
    }
    impl  Display for ObjectiveTarget
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
             write!(f,"{:?}", &self)
        }
    }
    impl Default for ObjectiveTarget {
        fn default() -> Self {
             Self::None
        }
    }
    /**
    ```xml
    <minimize or maximize [ id="identifier" ]  [ type="expression" ]>
      intExpr
    </minimize or maximize>
    ````
     */
    #[derive(Deserialize, Debug)]
    pub struct MaximizeMinimize {
        #[serde(rename = "id", default)]
        pub id: String,
        #[serde(rename = "type", default)]
        pub r#type: ObjectiveTarget,
        #[serde(rename = "list", default)]
        pub list: String,
        #[serde(rename = "coeffs", default)]
        pub coeffs: String,
        #[serde(rename = "$value", default)]
        pub expression: String,
    }

    /**
    ```xml
    <objectives  [ combination="combinationType" ]>
      (<minimize.../> | <maximize.../>)+
    </objectives>
    ````
    or
      ```xml
    <minimize  [ id="identifier" ]  [ type="expression" ]>
      intExpr
    </minimize>
     ````
    */
    #[derive(Deserialize, Debug)]
    pub struct Objective {
        #[serde(rename = "minimize", default)]
        pub minimize: Vec<MaximizeMinimize>,
        #[serde(rename = "maximize", default)]
        pub maximize: Vec<MaximizeMinimize>,
    }
}
