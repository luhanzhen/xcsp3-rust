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
 * <p/>
 * <p>@author: luhanzhen
 * <p/>
 * <p>@date: 2023/7/2
 * <p/>
 * <p>@time: 14:45
 * <p/>
 * <p>@this_file_name:xcsp3treenode
 * <p/>
 */
#[allow(dead_code)]
pub mod xcsp3_core {
    use std::collections::{HashMap, HashSet};
    use std::ops::Deref;

    #[derive(Copy, Clone)]
    pub enum ExpressionType {
        OUNDEF,
        ONEG,
        OABS,
        OSQR,
        OADD,
        OSUB,
        OMUL,
        ODIV,
        OMOD,
        OPOW,
        ODIST,
        OMIN,
        OMAX,
        OLT,
        OLE,
        OGE,
        OGT,
        ONE,
        OEQ,
        OSET,
        OIN,
        ONOTIN,
        ONOT,
        OAND,
        OOR,
        OXOR,
        OIFF,
        OIMP,
        OIF,
        OCARD,
        OUNION,
        OINTER,
        ODIFF,
        OSDIFF,
        OHULL,
        ODJOINT,
        OSUBSET,
        OSUBSEQ,
        OSUPSEQ,
        OSUPSET,
        OCONVEX,
        OFDIV,
        OFMOD,
        OSQRT,
        ONROOT,
        OEXP,
        OLN,
        OLOG,
        OSIN,
        OCOS,
        OTAN,
        OASIN,
        OACOS,
        OATAN,
        OSINH,
        OCOSH,
        OTANH,
        OVAR,
        OPAR,
        OLONG,
        ORATIONAL,
        ODECIMAL,
        OSYMBOL,
        OFAKEOP, // Used only to match primitives
    }

    pub trait NodeTraits {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32;
        fn to_string(&self) -> String;

    }

    pub struct Node<'a> {
        node_type: ExpressionType,
        parameters: Vec<&'a dyn NodeTraits>,
    }

    impl<'a> Node<'a> {
        pub fn new(node_type: ExpressionType) -> Node<'a> {
            Node {
                node_type,
                parameters: vec![],
            }
        }
    }

    pub struct NodeConstant<'a> {
        node: Node<'a>,
        val: i32,
    }

    impl<'a> NodeConstant<'a> {
        pub fn new(v: i32) -> NodeConstant<'a> {
            NodeConstant {
                val: v,
                node: Node::new(ExpressionType::ODECIMAL),
            }
        }
        pub fn canonize(&self) -> &Self {
            self
        }
    }

    impl<'a> NodeTraits for NodeConstant<'a> {
        fn evaluate(&self, _: &HashMap<&String, i32>) -> i32 {
            self.val
        }

        fn to_string(&self) -> String {
            format!("{}", self.val)
        }
    }

    pub struct NodeVariable<'a> {
        node: Node<'a>,
        var: String,
    }

    impl<'a> NodeVariable<'a> {
        pub fn new(v: String) -> NodeVariable<'a> {
            NodeVariable {
                var: v,
                node: Node::new(ExpressionType::OVAR),
            }
        }
        pub fn canonize(&self) -> &Self {
            self
        }
    }

    impl<'a> NodeTraits for NodeVariable<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            tuple[&self.var]
        }

        fn to_string(&self) -> String {
            let mut str = format!("{}(", self.var);
            for e in self.node.parameters.iter() {
                str = format!("{},{}", str, e.to_string());
            }
            str = format!("{})", str);
            str
        }
    }

    pub struct NodeOperator<'a> {
        node: Node<'a>,
        op: String,
    }

    impl<'a> NodeOperator<'a> {
        pub fn new(v: String, _operator: ExpressionType) -> NodeOperator<'a> {
            NodeOperator {
                op: v,
                node: Node::new(_operator),
            }
        }

        pub fn add_parameter(&mut self, p: &'a dyn NodeTraits) {
            self.node.parameters.push(p);
        }
        pub fn add_parameters(&mut self, p: &mut Vec<Box<&'a dyn NodeTraits>>) {
            for e in p.iter() {
                let ee = *e.deref();
                self.node.parameters.push(ee);
            }
        }
    }

    impl<'a> NodeTraits for NodeOperator<'a> {
        fn evaluate(&self, _: &HashMap<&String, i32>) -> i32 {
            0
        }

        fn to_string(&self) -> String {
            self.op.to_string()
        }
    }

    // pub struct NodeUnary<'a>
    // {
    //     ope: NodeOperator<'a>
    // }
    //
    // pub struct NodeBinary<'a>
    // {
    //     ope: NodeOperator<'a>
    // }
    //
    // pub struct NodeNAry<'a>
    // {
    //     ope: NodeOperator<'a>
    // }

    pub struct NodeNeg<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeNeg<'a> {
        pub fn new() -> NodeNeg<'a> {
            NodeNeg {
                ope: NodeOperator::new("neg".to_string(), ExpressionType::ONEG),
            }
        }
    }

    impl<'a> NodeTraits for NodeNeg<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            -self.ope.node.parameters[0].evaluate(tuple)
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeAbs<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeAbs<'a> {
        pub fn new() -> NodeAbs<'a> {
            NodeAbs {
                ope: NodeOperator::new("abs".to_string(), ExpressionType::OABS),
            }
        }
    }

    impl<'a> NodeTraits for NodeAbs<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v = self.ope.node.parameters[0].evaluate(tuple);
            if v > 0 {
                v
            } else {
                -v
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeSquare<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeSquare<'a> {
        pub fn new() -> NodeSquare<'a> {
            NodeSquare {
                ope: NodeOperator::new("sqr".to_string(), ExpressionType::OSQR),
            }
        }
    }

    impl<'a> NodeTraits for NodeSquare<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v = self.ope.node.parameters[0].evaluate(tuple);
            v * v
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeNot<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeNot<'a> {
        pub fn new() -> NodeNot<'a> {
            NodeNot {
                ope: NodeOperator::new("not".to_string(), ExpressionType::ONOT),
            }
        }
    }

    impl<'a> NodeTraits for NodeNot<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v = self.ope.node.parameters[0].evaluate(tuple);
            if v > 0 {
                1
            } else {
                0
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeSub<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeSub<'a> {
        pub fn new() -> NodeSub<'a> {
            NodeSub {
                ope: NodeOperator::new("sub".to_string(), ExpressionType::OSUB),
            }
        }
    }

    impl<'a> NodeTraits for NodeSub<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v1 = self.ope.node.parameters[0].evaluate(tuple);
            let v2 = self.ope.node.parameters[1].evaluate(tuple);
            v1 - v2
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeDiv<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeDiv<'a> {
        pub fn new() -> NodeDiv<'a> {
            NodeDiv {
                ope: NodeOperator::new("div".to_string(), ExpressionType::ODIV),
            }
        }
    }

    impl<'a> NodeTraits for NodeDiv<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v1 = self.ope.node.parameters[0].evaluate(tuple);
            let v2 = self.ope.node.parameters[1].evaluate(tuple);
            v1 / v2
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeMod<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeMod<'a> {
        pub fn new() -> NodeMod<'a> {
            NodeMod {
                ope: NodeOperator::new("mod".to_string(), ExpressionType::OMOD),
            }
        }
    }

    impl<'a> NodeTraits for NodeMod<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v1 = self.ope.node.parameters[0].evaluate(tuple);
            let v2 = self.ope.node.parameters[1].evaluate(tuple);
            v1 % v2
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodePow<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodePow<'a> {
        pub fn new() -> NodePow<'a> {
            NodePow {
                ope: NodeOperator::new("pow".to_string(), ExpressionType::OPOW),
            }
        }
    }

    impl<'a> NodeTraits for NodePow<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v1 = self.ope.node.parameters[0].evaluate(tuple);
            let v2 = self.ope.node.parameters[1].evaluate(tuple);
            v1.pow(v2 as u32)
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeDist<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeDist<'a> {
        pub fn new() -> NodeDist<'a> {
            NodeDist {
                ope: NodeOperator::new("dist".to_string(), ExpressionType::ODIST),
            }
        }
    }

    impl<'a> NodeTraits for NodeDist<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v1 = self.ope.node.parameters[0].evaluate(tuple);
            let v2 = self.ope.node.parameters[1].evaluate(tuple);
            let v = v1 - v2;
            if v > 0 {
                v
            } else {
                -v
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeLE<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeLE<'a> {
        pub fn new() -> NodeLE<'a> {
            NodeLE {
                ope: NodeOperator::new("le".to_string(), ExpressionType::OLE),
            }
        }
    }

    impl<'a> NodeTraits for NodeLE<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v1 = self.ope.node.parameters[0].evaluate(tuple);
            let v2 = self.ope.node.parameters[1].evaluate(tuple);

            if v1 <= v2 {
                1
            } else {
                0
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeLT<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeLT<'a> {
        pub fn new() -> NodeLT<'a> {
            NodeLT {
                ope: NodeOperator::new("lt".to_string(), ExpressionType::OLT),
            }
        }
    }

    impl<'a> NodeTraits for NodeLT<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v1 = self.ope.node.parameters[0].evaluate(tuple);
            let v2 = self.ope.node.parameters[1].evaluate(tuple);

            if v1 < v2 {
                1
            } else {
                0
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeGE<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeGE<'a> {
        pub fn new() -> NodeGE<'a> {
            NodeGE {
                ope: NodeOperator::new("ge".to_string(), ExpressionType::OGE),
            }
        }
    }

    impl<'a> NodeTraits for NodeGE<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v1 = self.ope.node.parameters[0].evaluate(tuple);
            let v2 = self.ope.node.parameters[1].evaluate(tuple);

            if v1 >= v2 {
                1
            } else {
                0
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeGT<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeGT<'a> {
        pub fn new() -> NodeGT<'a> {
            NodeGT {
                ope: NodeOperator::new("gt".to_string(), ExpressionType::OGT),
            }
        }
    }

    impl<'a> NodeTraits for NodeGT<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v1 = self.ope.node.parameters[0].evaluate(tuple);
            let v2 = self.ope.node.parameters[1].evaluate(tuple);

            if v1 > v2 {
                1
            } else {
                0
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeNE<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeNE<'a> {
        pub fn new() -> NodeNE<'a> {
            NodeNE {
                ope: NodeOperator::new("ne".to_string(), ExpressionType::ONE),
            }
        }
    }

    impl<'a> NodeTraits for NodeNE<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v1 = self.ope.node.parameters[0].evaluate(tuple);
            let v2 = self.ope.node.parameters[1].evaluate(tuple);

            if v1 != v2 {
                1
            } else {
                0
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeImp<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeImp<'a> {
        pub fn new() -> NodeImp<'a> {
            NodeImp {
                ope: NodeOperator::new("imp".to_string(), ExpressionType::OIMP),
            }
        }
    }

    impl<'a> NodeTraits for NodeImp<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v1 = self.ope.node.parameters[0].evaluate(tuple);
            let v2 = self.ope.node.parameters[1].evaluate(tuple);

            if v1 == 0 || v2 != 0 {
                1
            } else {
                0
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeAdd<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeAdd<'a> {
        pub fn new() -> NodeAdd<'a> {
            NodeAdd {
                ope: NodeOperator::new("add".to_string(), ExpressionType::OADD),
            }
        }
    }

    impl<'a> NodeTraits for NodeAdd<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let mut nb = 0;
            for e in self.ope.node.parameters.iter() {
                nb += e.evaluate(tuple);
            }
            nb
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeMult<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeMult<'a> {
        pub fn new() -> NodeMult<'a> {
            NodeMult {
                ope: NodeOperator::new("mul".to_string(), ExpressionType::OMUL),
            }
        }
    }

    impl<'a> NodeTraits for NodeMult<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let mut nb = 1;
            for e in self.ope.node.parameters.iter() {
                nb *= e.evaluate(tuple);
            }
            nb
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeMin<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeMin<'a> {
        pub fn new() -> NodeMin<'a> {
            NodeMin {
                ope: NodeOperator::new("min".to_string(), ExpressionType::OMIN),
            }
        }
    }

    impl<'a> NodeTraits for NodeMin<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let mut nb = i32::MAX;
            for e in self.ope.node.parameters.iter() {
                let n = e.evaluate(tuple);
                if nb > n {
                    nb = n
                }
            }
            nb
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeMax<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeMax<'a> {
        pub fn new() -> NodeMax<'a> {
            NodeMax {
                ope: NodeOperator::new("max".to_string(), ExpressionType::OMAX),
            }
        }
    }

    impl<'a> NodeTraits for NodeMax<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let mut nb = i32::MIN;
            for e in self.ope.node.parameters.iter() {
                let n = e.evaluate(tuple);
                if nb < n {
                    nb = n
                }
            }
            nb
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeEQ<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeEQ<'a> {
        pub fn new() -> NodeEQ<'a> {
            NodeEQ {
                ope: NodeOperator::new("eq".to_string(), ExpressionType::OEQ),
            }
        }
    }

    impl<'a> NodeTraits for NodeEQ<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let nb = self.ope.node.parameters[0].evaluate(tuple);
            for e in self.ope.node.parameters.iter() {
                let n = e.evaluate(tuple);
                if nb != n {
                    return 0;
                }
            }
            1
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeAnd<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeAnd<'a> {
        pub fn new() -> NodeAnd<'a> {
            NodeAnd {
                ope: NodeOperator::new("and".to_string(), ExpressionType::OAND),
            }
        }
    }

    impl<'a> NodeTraits for NodeAnd<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            for e in self.ope.node.parameters.iter() {
                let n = e.evaluate(tuple);
                if n == 0 {
                    return 0;
                }
            }
            1
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeOr<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeOr<'a> {
        pub fn new() -> NodeOr<'a> {
            NodeOr {
                ope: NodeOperator::new("or".to_string(), ExpressionType::OOR),
            }
        }
    }

    impl<'a> NodeTraits for NodeOr<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            for e in self.ope.node.parameters.iter() {
                let n = e.evaluate(tuple);
                if n == 1 {
                    return 1;
                }
            }
            0
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeXor<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeXor<'a> {
        pub fn new() -> NodeXor<'a> {
            NodeXor {
                ope: NodeOperator::new("xor".to_string(), ExpressionType::OXOR),
            }
        }
    }

    impl<'a> NodeTraits for NodeXor<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let mut nb = 0;
            for e in self.ope.node.parameters.iter() {
                nb += e.evaluate(tuple);
            }
            if nb % 2 == 1 {
                1
            } else {
                0
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeIf<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeIf<'a> {
        pub fn new() -> NodeIf<'a> {
            NodeIf {
                ope: NodeOperator::new("if".to_string(), ExpressionType::OIF),
            }
        }
    }

    impl<'a> NodeTraits for NodeIf<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v0 = self.ope.node.parameters[0].evaluate(tuple);
            if v0 != 0 {
                self.ope.node.parameters[1].evaluate(tuple)
            } else {
                self.ope.node.parameters[2].evaluate(tuple)
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeIff<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeIff<'a> {
        pub fn new() -> NodeIff<'a> {
            NodeIff {
                ope: NodeOperator::new("iff".to_string(), ExpressionType::OIFF),
            }
        }
    }

    impl<'a> NodeTraits for NodeIff<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v0 = self.ope.node.parameters[0].evaluate(tuple);
            let v1 = self.ope.node.parameters[1].evaluate(tuple);
            if v0 != 0 {
                if v1 != 0 {
                    1
                } else {
                    0
                }
            } else {
                if v1 == 0 {
                    1
                } else {
                    0
                }
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeSet<'a> {
        ope: NodeOperator<'a>,
    }

    impl<'a> NodeSet<'a> {
        pub fn new() -> NodeSet<'a> {
            NodeSet {
                ope: NodeOperator::new("set".to_string(), ExpressionType::OSET),
            }
        }
    }

    impl<'a> NodeTraits for NodeSet<'a> {
        fn evaluate(&self, _: &HashMap<&String, i32>) -> i32 {
            panic!("can't evaluate set")
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeIn<'a> {
        ope: NodeOperator<'a>,
        set: HashSet<i32>,
    }

    impl<'a> NodeIn<'a> {
        pub fn new() -> NodeIn<'a> {
            NodeIn {
                ope: NodeOperator::new("in".to_string(), ExpressionType::OIN),
                set: HashSet::new(),
            }
        }
    }

    impl<'a> NodeTraits for NodeIn<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v0 = self.ope.node.parameters[0].evaluate(tuple);
            if self.set.contains(&v0) {
                1
            } else {
                0
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub struct NodeNotIn<'a> {
        ope: NodeOperator<'a>,
        set: HashSet<i32>,
    }

    impl<'a> NodeNotIn<'a> {
        pub fn new() -> NodeNotIn<'a> {
            NodeNotIn {
                ope: NodeOperator::new("notin".to_string(), ExpressionType::ONOTIN),
                set: HashSet::new(),
            }
        }
    }

    impl<'a> NodeTraits for NodeNotIn<'a> {
        fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
            let v0 = self.ope.node.parameters[0].evaluate(tuple);
            if !self.set.contains(&v0) {
                1
            } else {
                0
            }
        }

        fn to_string(&self) -> String {
            self.ope.to_string()
        }
    }

    pub fn is_symmetric_operator(exp_type: &ExpressionType) -> bool {
        match exp_type {
            ExpressionType::OADD => true,
            ExpressionType::OMUL => true,
            ExpressionType::OMIN => true,
            ExpressionType::OMAX => true,
            ExpressionType::ODIST => true,
            ExpressionType::ONE => true,
            ExpressionType::OEQ => true,
            ExpressionType::OSET => true,
            ExpressionType::OAND => true,
            ExpressionType::OOR => true,
            ExpressionType::OXOR => true,
            ExpressionType::OIFF => true,
            ExpressionType::OUNION => true,
            ExpressionType::OINTER => true,
            ExpressionType::ODJOINT => true,
            _ => false,
        }
    }

    pub fn is_non_symmetric_relational_operator(exp_type: &ExpressionType) -> bool {
        match exp_type {
            ExpressionType::OLT => true,
            ExpressionType::OLE => true,
            ExpressionType::OGE => true,
            ExpressionType::OGT => true,
            _ => false,
        }
    }

    pub fn is_predicate_operator(exp_type: &ExpressionType) -> bool {
        if is_symmetric_operator(exp_type) {
            true
        } else {
            match exp_type {
                ExpressionType::ONOT => true,
                ExpressionType::OIMP => true,
                ExpressionType::OAND => true,
                ExpressionType::OOR => true,
                ExpressionType::OXOR => true,
                ExpressionType::OIFF => true,
                ExpressionType::OIN => true,
                ExpressionType::ONOTIN => true,
                _ => false,
            }
        }
    }

    pub fn is_relational_operator(exp_type: &ExpressionType) -> bool {
        if is_non_symmetric_relational_operator(exp_type) {
            true
        } else {
            match exp_type {
                ExpressionType::ONE => true,
                ExpressionType::OEQ => true,
                _ => false,
            }
        }
    }

    pub fn arithmetic_inversion(exp_type: &ExpressionType) -> ExpressionType {
        match exp_type {
            ExpressionType::OLT => ExpressionType::OGT,
            _ => match exp_type {
                ExpressionType::OLE => ExpressionType::OGE,
                _ => match exp_type {
                    ExpressionType::OGE => ExpressionType::OLE,
                    _ => match exp_type {
                        ExpressionType::OGT => ExpressionType::OLT,
                        _ => *exp_type,
                    },
                },
            },
        }
    }

    pub fn operator_to_string(exp_type: &ExpressionType) -> String {
        match exp_type {
            ExpressionType::ONEG => "neg".to_string(),
            ExpressionType::OABS => "abs".to_string(),

            ExpressionType::OADD => "add".to_string(),
            ExpressionType::OMUL => "mul".to_string(),
            ExpressionType::OSUB => "sub".to_string(),
            ExpressionType::ODIV => "div".to_string(),
            ExpressionType::OMOD => "mod".to_string(),

            ExpressionType::OSQR => "sqr".to_string(),
            ExpressionType::OPOW => "pow".to_string(),

            ExpressionType::OMIN => "min".to_string(),
            ExpressionType::OMAX => "max".to_string(),

            ExpressionType::ODIST => "dist".to_string(),

            ExpressionType::ONE => "ne".to_string(),
            ExpressionType::OEQ => "eq".to_string(),

            ExpressionType::OLE => "le".to_string(),
            ExpressionType::OLT => "lt".to_string(),
            ExpressionType::OGE => "ge".to_string(),
            ExpressionType::OGT => "gt".to_string(),

            ExpressionType::ONOT => "not".to_string(),
            ExpressionType::OAND => "and".to_string(),
            ExpressionType::OOR => "or".to_string(),
            ExpressionType::OXOR => "xor".to_string(),
            ExpressionType::OIMP => "imp".to_string(),
            ExpressionType::OIFF => "iff".to_string(),
            ExpressionType::OIF => "if".to_string(),

            ExpressionType::OIN => "in".to_string(),
            ExpressionType::ONOTIN => "notin".to_string(),
            ExpressionType::OSET => "set".to_string(),
            _ => "undefined".to_string(),
        }
    }

    pub fn create_node_operator(op: &String) -> Option<Box<dyn NodeTraits>>
    {
        match op.as_str() {
            "neg" => Some(Box::new(NodeNeg::new())),

            _ => None,
        }
    }

}
