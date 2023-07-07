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
 * </p>
 * <p>@author: luhanzhen
 * </p>
 * <p>@date: 2023/7/7
 * </p>
 * <p>@time: 9:57
 * </p>
 * <p>@this_file_name:xcsp3skeleton
 * </p>
 **/
#[allow(dead_code)]
pub mod xcsp3_core {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct VariableDomain {
        #[serde(rename = "@for")]
        pub r#for: String,
        #[serde(rename = "$value")]
        pub value: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct VariableArray {
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@type", default)]
        pub type_: Option<String>,
        #[serde(rename = "@startIndex", default)]
        pub start_index: Option<String>,
        #[serde(rename = "@note", default)]
        pub note: Option<String>,
        #[serde(rename = "@size", default)]
        pub size: String,
        #[serde(rename = "$value", default)]
        pub value: Option<String>,
        #[serde(rename = "domain", default)]
        pub domains: Vec<VariableDomain>,
    }

    #[derive(Deserialize, Debug)]
    pub struct VariableVar {
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@type", default)]
        pub type_: String,
        #[serde(rename = "$value")]
        pub value: String,
    }

    #[derive(Deserialize, Debug)]
    pub enum VariableType {
        #[serde(rename = "var")]
        Var(VariableVar),
        #[serde(rename = "array")]
        Array(VariableArray),
    }

    #[derive(Deserialize, Debug)]
    pub struct Variable {
        #[serde(rename = "$value", default)]
        pub variables: Vec<VariableType>,
    }

    #[derive(Deserialize, Debug)]
    pub struct ConstraintGroup {
        #[serde(rename = "@class")]
        pub class: Option<String>,
        #[serde(rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "$value", default)]
        pub constraints: Vec<ConstraintType>,
        #[serde(rename = "args", default)]
        pub args: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct ConstraintBlock {
        #[serde(rename = "@id", default)]
        pub id: String,
        #[serde(rename = "@note", default)]
        pub note: String,
        #[serde(rename = "@class", default)]
        pub r#type: Option<String>,
        #[serde(rename = "$value", default)]
        pub constraints: Vec<ConstraintType>,
    }

    #[derive(Deserialize, Debug)]
    pub enum ConstraintType {
        // #[serde(rename = "metaConstraint")]
        // eta_constraint(ConstraintMetaConstraint),
        #[serde(rename = "group")]
        Group(ConstraintGroup),
        #[serde(rename = "block")]
        Block(ConstraintBlock),
        #[serde(rename = "allDifferent")]
        AllDifferent {
            #[serde(rename = "$value")]
            vars: String,
        },
        #[serde(rename = "allEqual")]
        AllEqual {
            #[serde(rename = "$value")]
            vars: String,
        },
        #[serde(rename = "circuit")]
        Circuit {
            #[serde(rename = "$value")]
            vars: String,
        },
        #[serde(rename = "ordered")]
        Ordered {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "operator", default)]
            operator: String,
            #[serde(rename = "lengths", default)]
            lengths: String,
        },

        #[serde(rename = "intension")]
        Intension {
            #[serde(rename = "$value")]
            value: String,
        },
        #[serde(rename = "extension")]
        Extension {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "supports", default)]
            supports: String,
        },
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
        #[serde(rename = "mdd")]
        Mdd {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "transitions", default)]
            transitions: String,
        },
        #[serde(rename = "sum")]
        Sum {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "coeffs", default)]
            coeffs: String,
        },
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
        #[serde(rename = "nValues")]
        NValues {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "except", default)]
            except: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "cardinality")]
        Cardinality {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
            #[serde(rename = "occurs", default)]
            occurs: String,
        },
        #[serde(rename = "minimum")]
        Minimum {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "maximum")]
        Maximum {
            #[serde(rename = "list", default)]
            list: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "element")]
        Element {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            value: String,
            #[serde(rename = "index", default)]
            index: String,
        },
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
        #[serde(rename = "noOverlap")]
        NoOverlap {
            #[serde(rename = "origins", default)]
            origins: String,
            #[serde(rename = "lengths", default)]
            lengths: String,
        },
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
        #[serde(rename = "instantiation")]
        Instantiation {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
        },
        #[serde(rename = "slide")]
        Slide {
            #[serde(rename = "@id", default)]
            id: String,
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "$value", default)]
            constraints: Vec<ConstraintType>,
        },
        #[serde(rename = "channel")]
        Channel {
            #[serde(rename = "list", default)]
            vars: Vec<String>,
            #[serde(rename = "value", default)]
            value: String,
        },
        #[serde(rename = "allDistant")]
        AllDistant {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "precedence")]
        Precedence {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
        },
        #[serde(rename = "balance")]
        Balance {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "values", default)]
            values: String,
        },
        #[serde(rename = "spread")]
        Spread {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "total", default)]
            total: String,
        },
        #[serde(rename = "deviation")]
        Deviation {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "total", default)]
            total: String,
        },

        #[serde(rename = "binPacking")]
        BinPacking {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "sizes", default)]
            sizes: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
    }

    #[derive(Deserialize, Debug)]
    pub struct Constraint {
        // #[serde(rename = "constraint", default)]
        #[serde(rename = "$value", default)]
        pub constraints: Vec<ConstraintType>,
    }

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
    }

    #[derive(Deserialize, Debug)]
    pub struct MaximizeMinimize {
        #[serde(rename = "id", default)]
        pub id: String,
        #[serde(rename = "type")]
        pub r#type: Option<ObjectiveTarget>,
        #[serde(rename = "list", default)]
        pub list: String,
        #[serde(rename = "coeffs", default)]
        pub coeffs: String,
        #[serde(rename = "$value", default)]
        pub value: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Objective {
        #[serde(rename = "minimize", default)]
        pub minimize: Vec<MaximizeMinimize>,
        #[serde(rename = "maximize", default)]
        pub maximize: Vec<MaximizeMinimize>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Instance {
        #[serde(rename = "@format")]
        pub format: String,

        #[serde(rename = "@type")]
        pub r#type: String,

        pub variables: Option<Variable>,

        pub constraints: Constraint,

        pub objectives: Option<Objective>,
    }
}
