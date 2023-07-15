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
* <p>@date:  2023/7/14 19:09
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 **/

pub mod xcsp3_xml {
    use crate::xcsp_xml::constraint_block::xcsp3_xml::ConstraintBlock;
    use crate::xcsp_xml::constraint_group::xcsp3_xml::ConstraintGroup;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub enum ConstraintType {
        // #[serde(rename = "metaConstraint")]
        // eta_constraint(ConstraintMetaConstraint),
        #[serde(rename = "group")]
        Group(ConstraintGroup),
        #[serde(rename = "block")]
        Block(ConstraintBlock),

        /**
          syntax.
          ```xml
           <allDifferent>
          <list> (intVar wspace)2+ </list>
          [<except> (intVal wspace)+ </except>]
        </allDifferent>
          ```

          eg.
          ```xml
         <allDifferent>
          x1 x2 x3 x4 x5
        </allDifferent>
        <allDifferent>
          <list> y[] </list>
          <except> 0 </except>
        </allDifferent>
          ```
        or
          ```xml
        <allDifferent>
          <matrix>
            (x1,x2,x3,x4,x5)
            (y1,y2,y3,y4,y5)
            (z1,z2,z3,z4,z5)
          </matrix>
        </allDifferent>
          ```
         */
        #[serde(rename = "allDifferent")]
        AllDifferent {
            #[serde(rename = "$value", default)]
            vars: String,
            #[serde(rename = "list", default)]
            list: Vec<String>,
            #[serde(rename = "except", default)]
            except: String,
            #[serde(rename = "matrix", default)]
            matrix: String,
        },
        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "allEqual")]
        AllEqual {
            #[serde(rename = "$value")]
            vars: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "circuit")]
        Circuit {
            #[serde(rename = "$value")]
            vars: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "ordered")]
        Ordered {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "operator", default)]
            operator: String,
            #[serde(rename = "lengths", default)]
            lengths: String,
        },
        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "intension")]
        Intension {
            #[serde(rename = "$value")]
            value: String,
        },
        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "extension")]
        Extension {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "supports", default)]
            supports: String,
            #[serde(rename = "conflicts", default)]
            conflicts: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "regular")]
        Regular {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "transitions", default)]
            transitions: String,
            #[serde(rename = "start", default)]
            start: String,
            #[serde(rename = "final", default)]
            r#final: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "mdd")]
        Mdd {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "transitions", default)]
            transitions: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "sum")]
        Sum {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "coeffs", default)]
            coeffs: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "count")]
        Count {
            #[serde(rename = "@id", default)]
            id: String,
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "nValues")]
        NValues {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "except", default)]
            except: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "cardinality")]
        Cardinality {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
            #[serde(rename = "occurs", default)]
            occurs: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "minimum")]
        Minimum {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "maximum")]
        Maximum {
            #[serde(rename = "list", default)]
            list: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "element")]
        Element {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            value: String,
            #[serde(rename = "index", default)]
            index: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "stretch")]
        Stretch {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
            #[serde(rename = "widths", default)]
            widths: String,
            #[serde(rename = "patterns", default)]
            patterns: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "noOverlap")]
        NoOverlap {
            #[serde(rename = "origins", default)]
            origins: String,
            #[serde(rename = "lengths", default)]
            lengths: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "cumulative")]
        Cumulative {
            #[serde(rename = "origins", default)]
            origins: String,
            #[serde(rename = "lengths", default)]
            lengths: String,
            #[serde(rename = "heights", default)]
            heights: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "ends", default)]
            ends: String,
            #[serde(rename = "machines", default)]
            machines: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "instantiation")]
        Instantiation {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "slide")]
        Slide {
            #[serde(rename = "@id", default)]
            id: String,
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "$value", default)]
            constraints: Vec<ConstraintType>,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "channel")]
        Channel {
            #[serde(rename = "list", default)]
            vars: Vec<String>,
            #[serde(rename = "value", default)]
            value: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "allDistant")]
        AllDistant {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "precedence")]
        Precedence {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "balance")]
        Balance {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "values", default)]
            values: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "spread")]
        Spread {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "total", default)]
            total: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "deviation")]
        Deviation {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "total", default)]
            total: String,
        },
        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "binPacking")]
        BinPacking {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "sizes", default)]
            sizes: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "lex")]
        Lex {
            #[serde(rename = "matrix", default)]
            matrix: String,
            #[serde(rename = "operator", default)]
            operator: String,
        },

        /**
        syntax.
        ```xml

        ```

        eg.
        ```xml

        ```
         */
        #[serde(rename = "clause")]
        Clause {
            #[serde(rename = "$value", default)]
            vars: String,
            #[serde(rename = "list", default)]
            list: Vec<String>,
        },
    }
}
