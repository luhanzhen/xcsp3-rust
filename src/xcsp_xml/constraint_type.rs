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
        <allEqual> (intVar wspace)2+ </allEqual>
        ```
        or
        ```xml
            <allEqual>
          <list> (intVar wspace)2+ </list>
        </allEqual>
        ```
        eg.
        ```xml
        <allEqual>
          x1 x2 x3 x4 x5
        </allEqual>
        ```
        or
        ```xml
        <allEqual>
            <list>  x1 x2 x3 </list>
            <list>  y1 y2 y3 </list>
        </allEqual>
        ```
         */
        #[serde(rename = "allEqual")]
        AllEqual {
            #[serde(rename = "$value", default)]
            vars: String,
            #[serde(rename = "list", default)]
            list: Vec<String>,
        },

        /**

        syntax
           ```xm

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
        <ordered>
          <list> (intVar wspace)2+ </list>
          [<lengths> (intVal wspace)2+ | (intVar wspace)2+ </lengths>]
          <operator> "lt" | "le" | "ge" | "gt" </operator>
        </ordered>
        ```

        eg.
        ```xml
        <ordered>
          <list> x1 x2 x3 x4 </list>
          <operator> lt </operator>
        </ordered>
        <ordered>
          <list> y1 y2 y3  </list>
          <lengths> 5 3 </lengths>
          <operator> ge </operator>
        </ordered>
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
        <regular>
          <list> (intVar wspace)+ </list>
          <transitions> ("(" state "," intVal "," state ")")+ </transitions>
          <start> state </start>
          <final> (state wspace)+ </final>
        </regular>
        ```

        eg.
        ```xml
        <regular>
          <list> x1 x2 x3 x4 x5 x6 x7 </list>
          <transitions>
            (a,0,a)(a,1,b)(b,1,c)(c,0,d)(d,0,d)(d,1,e)(e,0,e)
          </transitions>
          <start> a </start>
          <final> e </final>
        </regular>
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
        <mdd>
          <list> (intVar wspace)+ </list>
          <transitions> ("(" state "," intVal "," state ")")+ </transitions>
        </mdd>
        ```

        eg.
        ```xml
        <mdd>
          <list> x1 x2 x3 </list>
          <transitions>
            (r,0,n1)(r,1,n2)(r,2,n3)
            (n1,2,n4)(n2,2,n4)(n3,0,n5)
            (n4,0,t)(n5,0,t)
          </transitions>
        </mdd>
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
        <instantiation>
            <list> (intVar wspace)+ </list>
            <values> (intVal wspace)+ </values>
        </instantiation>
        ```

        eg.
        ```xml
        <instantiation>
            <list> x y z </list>
            <values> 12 4 30 </values>
        </instantiation>
        <instantiation>
        <list> w[] </list>
        <values> 1 0 2 1 3 1 </values>
        </instantiation>
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