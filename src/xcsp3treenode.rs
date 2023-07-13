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
 * <p>@date: 2023/7/2
 * </p>
 * <p>@time: 14:45
 * </p>
 * <p>@this_file_name:xcsp3treenode
 * </p>
 */

// #[allow(dead_code)]
// pub mod xcsp3_core {
//     use std::cmp::Ordering;
//     use std::collections::{HashMap, HashSet};
//     use std::ops::Deref;
//
//     #[derive(Copy, Clone, PartialEq)]
//     pub enum ExpressionType {
//         OUNDEF,
//         ONEG,
//         OABS,
//         OSQR,
//         OADD,
//         OSUB,
//         OMUL,
//         ODIV,
//         OMOD,
//         OPOW,
//         ODIST,
//         OMIN,
//         OMAX,
//         OLT,
//         OLE,
//         OGE,
//         OGT,
//         ONE,
//         OEQ,
//         OSET,
//         OIN,
//         ONOTIN,
//         ONOT,
//         OAND,
//         OOR,
//         OXOR,
//         OIFF,
//         OIMP,
//         OIF,
//         OCARD,
//         OUNION,
//         OINTER,
//         ODIFF,
//         OSDIFF,
//         OHULL,
//         ODJOINT,
//         OSUBSET,
//         OSUBSEQ,
//         OSUPSEQ,
//         OSUPSET,
//         OCONVEX,
//         OFDIV,
//         OFMOD,
//         OSQRT,
//         ONROOT,
//         OEXP,
//         OLN,
//         OLOG,
//         OSIN,
//         OCOS,
//         OTAN,
//         OASIN,
//         OACOS,
//         OATAN,
//         OSINH,
//         OCOSH,
//         OTANH,
//         OVAR,
//         OPAR,
//         OLONG,
//         ORATIONAL,
//         ODECIMAL,
//         OSYMBOL,
//         OFAKEOP, // Used only to match primitives
//     }
//
//     pub fn get_order(enum_type: &ExpressionType) -> i32 {
//         match enum_type {
//             ExpressionType::OUNDEF => 1,
//             ExpressionType::ONEG => 2,
//             ExpressionType::OABS => 3,
//             ExpressionType::OSQR => 4,
//             ExpressionType::OADD => 5,
//             ExpressionType::OSUB => 6,
//             ExpressionType::OMUL => 7,
//             ExpressionType::ODIV => 8,
//             ExpressionType::OMOD => 9,
//             ExpressionType::OPOW => 10,
//             ExpressionType::ODIST => 11,
//             ExpressionType::OMIN => 12,
//             ExpressionType::OMAX => 13,
//             ExpressionType::OLT => 14,
//             ExpressionType::OLE => 15,
//             ExpressionType::OGE => 16,
//             ExpressionType::OGT => 17,
//             ExpressionType::ONE => 18,
//             ExpressionType::OEQ => 19,
//             ExpressionType::OSET => 20,
//             ExpressionType::OIN => 21,
//             ExpressionType::ONOTIN => 22,
//             ExpressionType::ONOT => 23,
//             ExpressionType::OAND => 24,
//             ExpressionType::OOR => 25,
//             ExpressionType::OXOR => 26,
//             ExpressionType::OIFF => 27,
//             ExpressionType::OIMP => 28,
//             ExpressionType::OIF => 29,
//             ExpressionType::OCARD => 30,
//             ExpressionType::OUNION => 31,
//             ExpressionType::OINTER => 32,
//             ExpressionType::ODIFF => 33,
//             ExpressionType::OSDIFF => 34,
//             ExpressionType::OHULL => 35,
//             ExpressionType::ODJOINT => 36,
//             ExpressionType::OSUBSET => 37,
//             ExpressionType::OSUBSEQ => 38,
//             ExpressionType::OSUPSEQ => 39,
//             ExpressionType::OSUPSET => 40,
//             ExpressionType::OCONVEX => 41,
//             ExpressionType::OFDIV => 42,
//             ExpressionType::OFMOD => 43,
//             ExpressionType::OSQRT => 44,
//             ExpressionType::ONROOT => 45,
//             ExpressionType::OEXP => 46,
//             ExpressionType::OLN => 47,
//             ExpressionType::OLOG => 48,
//             ExpressionType::OSIN => 49,
//             ExpressionType::OCOS => 50,
//             ExpressionType::OTAN => 51,
//             ExpressionType::OASIN => 52,
//             ExpressionType::OACOS => 53,
//             ExpressionType::OATAN => 54,
//             ExpressionType::OSINH => 55,
//             ExpressionType::OCOSH => 56,
//             ExpressionType::OTANH => 57,
//             ExpressionType::OVAR => 58,
//             ExpressionType::OPAR => 59,
//             ExpressionType::OLONG => 60,
//             ExpressionType::ORATIONAL => 61,
//             ExpressionType::ODECIMAL => 62,
//             ExpressionType::OSYMBOL => 63,
//             ExpressionType::OFAKEOP => 64,
//         }
//     }
//
//     // impl Sub for ExpressionType {
//     //     type Output = i32;
//     //     fn sub(self, rhs: ExpressionType) -> Self::Output {
//     //         get_order(&self) - get_order(&rhs)
//     //     }
//     // }
//
//     pub enum NodeType {
//         Constant,
//         Variable,
//         Operator,
//     }
//
//     pub trait NodeTraits {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32;
//         fn to_string(&self) -> String;
//         // fn get_node(&self) -> &Node;
//         fn get_cons_or_var_or_ope(&self) -> NodeType {
//             NodeType::Operator
//         }
//         fn get_value(&self) -> i32 {
//             -1
//         }
//         fn get_variable(&self) -> String {
//             String::default()
//         }
//         fn get_parameters(&self) -> &Vec<&dyn NodeTraits>;
//
//         fn get_express_type(&self) -> &ExpressionType;
//     }
//
//     pub struct NodeConstant<'a> {
//         parameters: Vec<&'a dyn NodeTraits>,
//         express_type: ExpressionType,
//         val: i32,
//     }
//
//     impl<'a> NodeConstant<'a> {
//         pub fn new(v: i32) -> NodeConstant<'a> {
//             NodeConstant {
//                 val: v,
//                 parameters: vec![],
//                 express_type: ExpressionType::ODECIMAL,
//             }
//         }
//         pub fn canonize(&self) -> &Self {
//             self
//         }
//     }
//
//     impl<'a> NodeTraits for NodeConstant<'a> {
//         fn evaluate(&self, _: &HashMap<&String, i32>) -> i32 {
//             self.val
//         }
//         fn to_string(&self) -> String {
//             format!("{}", self.val)
//         }
//
//         fn get_cons_or_var_or_ope(&self) -> NodeType {
//             NodeType::Constant
//         }
//
//         fn get_value(&self) -> i32 {
//             self.val
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.parameters
//         }
//
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.express_type
//         }
//     }
//
//     pub struct NodeVariable<'a> {
//         parameters: Vec<&'a dyn NodeTraits>,
//         express_type: ExpressionType,
//         var: String,
//     }
//
//     impl<'a> NodeVariable<'a> {
//         pub fn new(v: String) -> NodeVariable<'a> {
//             NodeVariable {
//                 var: v,
//                 parameters: vec![],
//                 express_type: ExpressionType::OVAR,
//             }
//         }
//         pub fn canonize(&self) -> &Self {
//             self
//         }
//     }
//
//     impl<'a> NodeTraits for NodeVariable<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             tuple[&self.var]
//         }
//         fn to_string(&self) -> String {
//             let mut str = format!("{}(", self.var);
//             for e in self.parameters.iter() {
//                 str = format!("{},{}", str, e.to_string());
//             }
//             str = format!("{})", str);
//             str
//         }
//
//         fn get_cons_or_var_or_ope(&self) -> NodeType {
//             NodeType::Variable
//         }
//         fn get_variable(&self) -> String {
//             self.var.clone()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.express_type
//         }
//     }
//
//     pub struct NodeOperator<'a> {
//         parameters: Vec<&'a dyn NodeTraits>,
//         express_type: ExpressionType,
//         op: String,
//     }
//
//     impl<'a> NodeOperator<'a> {
//         pub fn new(v: String, _operator: ExpressionType) -> NodeOperator<'a> {
//             NodeOperator {
//                 op: v,
//                 express_type: _operator,
//                 parameters: vec![],
//             }
//         }
//
//         pub fn add_parameter(&mut self, p: &'a dyn NodeTraits) {
//             self.parameters.push(p);
//         }
//
//         pub fn add_parameters(&mut self, p: &mut [Box<&'a dyn NodeTraits>]) {
//             for e in p.iter() {
//                 let ee = *e.deref();
//                 self.parameters.push(ee);
//             }
//         }
//         pub fn to_string(&self) -> String {
//             self.op.to_string()
//         }
//     }
//
//     pub struct NodeNeg<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeNeg<'a> {
//         pub fn new() -> NodeNeg<'a> {
//             NodeNeg {
//                 ope: NodeOperator::new("neg".to_string(), ExpressionType::ONEG),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeNeg<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             -self.ope.parameters[0].evaluate(tuple)
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeAbs<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeAbs<'a> {
//         pub fn new() -> NodeAbs<'a> {
//             NodeAbs {
//                 ope: NodeOperator::new("abs".to_string(), ExpressionType::OABS),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeAbs<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v = self.ope.parameters[0].evaluate(tuple);
//             if v > 0 {
//                 v
//             } else {
//                 -v
//             }
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeSquare<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeSquare<'a> {
//         pub fn new() -> NodeSquare<'a> {
//             NodeSquare {
//                 ope: NodeOperator::new("sqr".to_string(), ExpressionType::OSQR),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeSquare<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v = self.ope.parameters[0].evaluate(tuple);
//             v * v
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeNot<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeNot<'a> {
//         pub fn new() -> NodeNot<'a> {
//             NodeNot {
//                 ope: NodeOperator::new("not".to_string(), ExpressionType::ONOT),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeNot<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v = self.ope.parameters[0].evaluate(tuple);
//             if v > 0 {
//                 1
//             } else {
//                 0
//             }
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeSub<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeSub<'a> {
//         pub fn new() -> NodeSub<'a> {
//             NodeSub {
//                 ope: NodeOperator::new("sub".to_string(), ExpressionType::OSUB),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeSub<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v1 = self.ope.parameters[0].evaluate(tuple);
//             let v2 = self.ope.parameters[1].evaluate(tuple);
//             v1 - v2
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeDiv<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeDiv<'a> {
//         pub fn new() -> NodeDiv<'a> {
//             NodeDiv {
//                 ope: NodeOperator::new("div".to_string(), ExpressionType::ODIV),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeDiv<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v1 = self.ope.parameters[0].evaluate(tuple);
//             let v2 = self.ope.parameters[1].evaluate(tuple);
//             v1 / v2
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeMod<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeMod<'a> {
//         pub fn new() -> NodeMod<'a> {
//             NodeMod {
//                 ope: NodeOperator::new("mod".to_string(), ExpressionType::OMOD),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeMod<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v1 = self.ope.parameters[0].evaluate(tuple);
//             let v2 = self.ope.parameters[1].evaluate(tuple);
//             v1 % v2
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodePow<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodePow<'a> {
//         pub fn new() -> NodePow<'a> {
//             NodePow {
//                 ope: NodeOperator::new("pow".to_string(), ExpressionType::OPOW),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodePow<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v1 = self.ope.parameters[0].evaluate(tuple);
//             let v2 = self.ope.parameters[1].evaluate(tuple);
//             v1.pow(v2 as u32)
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeDist<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeDist<'a> {
//         pub fn new() -> NodeDist<'a> {
//             NodeDist {
//                 ope: NodeOperator::new("dist".to_string(), ExpressionType::ODIST),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeDist<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v1 = self.ope.parameters[0].evaluate(tuple);
//             let v2 = self.ope.parameters[1].evaluate(tuple);
//             let v = v1 - v2;
//             if v > 0 {
//                 v
//             } else {
//                 -v
//             }
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeLE<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeLE<'a> {
//         pub fn new() -> NodeLE<'a> {
//             NodeLE {
//                 ope: NodeOperator::new("le".to_string(), ExpressionType::OLE),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeLE<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             if self.ope.parameters[0].evaluate(tuple) <= self.ope.parameters[1].evaluate(tuple) {
//                 1
//             } else {
//                 0
//             }
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeLT<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeLT<'a> {
//         pub fn new() -> NodeLT<'a> {
//             NodeLT {
//                 ope: NodeOperator::new("lt".to_string(), ExpressionType::OLT),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeLT<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v1 = self.ope.parameters[0].evaluate(tuple);
//             let v2 = self.ope.parameters[1].evaluate(tuple);
//
//             if v1 < v2 {
//                 1
//             } else {
//                 0
//             }
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeGE<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeGE<'a> {
//         pub fn new() -> NodeGE<'a> {
//             NodeGE {
//                 ope: NodeOperator::new("ge".to_string(), ExpressionType::OGE),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeGE<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v1 = self.ope.parameters[0].evaluate(tuple);
//             let v2 = self.ope.parameters[1].evaluate(tuple);
//
//             if v1 >= v2 {
//                 1
//             } else {
//                 0
//             }
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeGT<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeGT<'a> {
//         pub fn new() -> NodeGT<'a> {
//             NodeGT {
//                 ope: NodeOperator::new("gt".to_string(), ExpressionType::OGT),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeGT<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v1 = self.ope.parameters[0].evaluate(tuple);
//             let v2 = self.ope.parameters[1].evaluate(tuple);
//
//             if v1 > v2 {
//                 1
//             } else {
//                 0
//             }
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeNE<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeNE<'a> {
//         pub fn new() -> NodeNE<'a> {
//             NodeNE {
//                 ope: NodeOperator::new("ne".to_string(), ExpressionType::ONE),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeNE<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v1 = self.ope.parameters[0].evaluate(tuple);
//             let v2 = self.ope.parameters[1].evaluate(tuple);
//
//             if v1 != v2 {
//                 1
//             } else {
//                 0
//             }
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeImp<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeImp<'a> {
//         pub fn new() -> NodeImp<'a> {
//             NodeImp {
//                 ope: NodeOperator::new("imp".to_string(), ExpressionType::OIMP),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeImp<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v1 = self.ope.parameters[0].evaluate(tuple);
//             let v2 = self.ope.parameters[1].evaluate(tuple);
//
//             if v1 == 0 || v2 != 0 {
//                 1
//             } else {
//                 0
//             }
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeAdd<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeAdd<'a> {
//         pub fn new() -> NodeAdd<'a> {
//             NodeAdd {
//                 ope: NodeOperator::new("add".to_string(), ExpressionType::OADD),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeAdd<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let mut nb = 0;
//             for e in self.ope.parameters.iter() {
//                 nb += e.evaluate(tuple);
//             }
//             nb
//         }
//
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeMult<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeMult<'a> {
//         pub fn new() -> NodeMult<'a> {
//             NodeMult {
//                 ope: NodeOperator::new("mul".to_string(), ExpressionType::OMUL),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeMult<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let mut nb = 1;
//             for e in self.ope.parameters.iter() {
//                 nb *= e.evaluate(tuple);
//             }
//             nb
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeMin<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeMin<'a> {
//         pub fn new() -> NodeMin<'a> {
//             NodeMin {
//                 ope: NodeOperator::new("min".to_string(), ExpressionType::OMIN),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeMin<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let mut nb = self.ope.parameters[0].evaluate(tuple);
//             for e in self.ope.parameters.iter() {
//                 let n = e.evaluate(tuple);
//                 if nb > n {
//                     nb = n
//                 }
//             }
//             nb
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeMax<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeMax<'a> {
//         pub fn new() -> NodeMax<'a> {
//             NodeMax {
//                 ope: NodeOperator::new("max".to_string(), ExpressionType::OMAX),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeMax<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let mut nb = self.ope.parameters[0].evaluate(tuple);
//             for e in self.ope.parameters.iter() {
//                 let n = e.evaluate(tuple);
//                 if nb < n {
//                     nb = n
//                 }
//             }
//             nb
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeEQ<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeEQ<'a> {
//         pub fn new() -> NodeEQ<'a> {
//             NodeEQ {
//                 ope: NodeOperator::new("eq".to_string(), ExpressionType::OEQ),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeEQ<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let nb = self.ope.parameters[0].evaluate(tuple);
//             for e in self.ope.parameters.iter() {
//                 let n = e.evaluate(tuple);
//                 if nb != n {
//                     return 0;
//                 }
//             }
//             1
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeAnd<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeAnd<'a> {
//         pub fn new() -> NodeAnd<'a> {
//             NodeAnd {
//                 ope: NodeOperator::new("and".to_string(), ExpressionType::OAND),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeAnd<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             for e in self.ope.parameters.iter() {
//                 let n = e.evaluate(tuple);
//                 if n == 0 {
//                     return 0;
//                 }
//             }
//             1
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeOr<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeOr<'a> {
//         pub fn new() -> NodeOr<'a> {
//             NodeOr {
//                 ope: NodeOperator::new("or".to_string(), ExpressionType::OOR),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeOr<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             for e in self.ope.parameters.iter() {
//                 let n = e.evaluate(tuple);
//                 if n == 1 {
//                     return 1;
//                 }
//             }
//             0
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeXor<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeXor<'a> {
//         pub fn new() -> NodeXor<'a> {
//             NodeXor {
//                 ope: NodeOperator::new("xor".to_string(), ExpressionType::OXOR),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeXor<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let mut nb = 0;
//             for e in self.ope.parameters.iter() {
//                 nb += e.evaluate(tuple);
//             }
//             if nb % 2 == 1 {
//                 1
//             } else {
//                 0
//             }
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeIf<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeIf<'a> {
//         pub fn new() -> NodeIf<'a> {
//             NodeIf {
//                 ope: NodeOperator::new("if".to_string(), ExpressionType::OIF),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeIf<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v0 = self.ope.parameters[0].evaluate(tuple);
//             if v0 != 0 {
//                 self.ope.parameters[1].evaluate(tuple)
//             } else {
//                 self.ope.parameters[2].evaluate(tuple)
//             }
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeIff<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeIff<'a> {
//         pub fn new() -> NodeIff<'a> {
//             NodeIff {
//                 ope: NodeOperator::new("iff".to_string(), ExpressionType::OIFF),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeIff<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v0 = self.ope.parameters[0].evaluate(tuple);
//             let v1 = self.ope.parameters[1].evaluate(tuple);
//             if v0 != 0 {
//                 if v1 != 0 {
//                     1
//                 } else {
//                     0
//                 }
//             } else if v1 == 0 {
//                 1
//             } else {
//                 0
//             }
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeSet<'a> {
//         pub ope: NodeOperator<'a>,
//     }
//
//     impl<'a> NodeSet<'a> {
//         pub fn new() -> NodeSet<'a> {
//             NodeSet {
//                 ope: NodeOperator::new("set".to_string(), ExpressionType::OSET),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeSet<'a> {
//         fn evaluate(&self, _: &HashMap<&String, i32>) -> i32 {
//             panic!("can't evaluate set")
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeIn<'a> {
//         pub ope: NodeOperator<'a>,
//         set: HashSet<i32>,
//     }
//
//     impl<'a> NodeIn<'a> {
//         pub fn new() -> NodeIn<'a> {
//             NodeIn {
//                 ope: NodeOperator::new("in".to_string(), ExpressionType::OIN),
//                 set: HashSet::new(),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeIn<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v0 = self.ope.parameters[0].evaluate(tuple);
//             if self.set.contains(&v0) {
//                 1
//             } else {
//                 0
//             }
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub struct NodeNotIn<'a> {
//         pub ope: NodeOperator<'a>,
//         set: HashSet<i32>,
//     }
//
//     impl<'a> NodeNotIn<'a> {
//         pub fn new() -> NodeNotIn<'a> {
//             NodeNotIn {
//                 ope: NodeOperator::new("notin".to_string(), ExpressionType::ONOTIN),
//                 set: HashSet::new(),
//             }
//         }
//     }
//
//     impl<'a> NodeTraits for NodeNotIn<'a> {
//         fn evaluate(&self, tuple: &HashMap<&String, i32>) -> i32 {
//             let v0 = self.ope.parameters[0].evaluate(tuple);
//             if !self.set.contains(&v0) {
//                 1
//             } else {
//                 0
//             }
//         }
//         fn to_string(&self) -> String {
//             self.ope.to_string()
//         }
//         fn get_parameters(&self) -> &Vec<&'a dyn NodeTraits> {
//             &self.ope.parameters
//         }
//         fn get_express_type(&self) -> &ExpressionType {
//             &self.ope.express_type
//         }
//     }
//
//     pub fn is_symmetric_operator(exp_type: &ExpressionType) -> bool {
//         // match exp_type {
//         //     ExpressionType::OADD => true,
//         //     ExpressionType::OMUL => true,
//         //     ExpressionType::OMIN => true,
//         //     ExpressionType::OMAX => true,
//         //     ExpressionType::ODIST => true,
//         //     ExpressionType::ONE => true,
//         //     ExpressionType::OEQ => true,
//         //     ExpressionType::OSET => true,
//         //     ExpressionType::OAND => true,
//         //     ExpressionType::OOR => true,
//         //     ExpressionType::OXOR => true,
//         //     ExpressionType::OIFF => true,
//         //     ExpressionType::OUNION => true,
//         //     ExpressionType::OINTER => true,
//         //     ExpressionType::ODJOINT => true,
//         //     _ => false,
//         // }
//         matches!(
//             exp_type,
//             ExpressionType::OADD
//                 | ExpressionType::OMUL
//                 | ExpressionType::OMIN
//                 | ExpressionType::OMAX
//                 | ExpressionType::ODIST
//                 | ExpressionType::ONE
//                 | ExpressionType::OEQ
//                 | ExpressionType::OSET
//                 | ExpressionType::OAND
//                 | ExpressionType::OOR
//                 | ExpressionType::OXOR
//                 | ExpressionType::OIFF
//                 | ExpressionType::OUNION
//                 | ExpressionType::OINTER
//                 | ExpressionType::ODJOINT
//         )
//     }
//
//     pub fn is_non_symmetric_relational_operator(exp_type: &ExpressionType) -> bool {
//         // match exp_type {
//         //     ExpressionType::OLT => true,
//         //     ExpressionType::OLE => true,
//         //     ExpressionType::OGE => true,
//         //     ExpressionType::OGT => true,
//         //     _ => false,
//         // }
//         matches!(
//             exp_type,
//             ExpressionType::OLT | ExpressionType::OLE | ExpressionType::OGE | ExpressionType::OGT
//         )
//     }
//
//     pub fn is_predicate_operator(exp_type: &ExpressionType) -> bool {
//         if is_symmetric_operator(exp_type) {
//             true
//         } else {
//             // match exp_type {
//             //     ExpressionType::ONOT => true,
//             //     ExpressionType::OIMP => true,
//             //     ExpressionType::OAND => true,
//             //     ExpressionType::OOR => true,
//             //     ExpressionType::OXOR => true,
//             //     ExpressionType::OIFF => true,
//             //     ExpressionType::OIN => true,
//             //     ExpressionType::ONOTIN => true,
//             //     _ => false,
//             // }
//             matches!(
//                 exp_type,
//                 ExpressionType::ONOT
//                     | ExpressionType::OIMP
//                     | ExpressionType::OAND
//                     | ExpressionType::OOR
//                     | ExpressionType::OXOR
//                     | ExpressionType::OIFF
//                     | ExpressionType::OIN
//                     | ExpressionType::ONOTIN
//             )
//         }
//     }
//
//     pub fn is_relational_operator(exp_type: &ExpressionType) -> bool {
//         if is_non_symmetric_relational_operator(exp_type) {
//             true
//         } else {
//             // match exp_type {
//             //     ExpressionType::ONE => true,
//             //     ExpressionType::OEQ => true,
//             //     _ => false,
//             // }
//             matches!(exp_type, ExpressionType::ONE | ExpressionType::OEQ)
//         }
//     }
//
//     pub fn arithmetic_inversion(exp_type: &ExpressionType) -> ExpressionType {
//         match exp_type {
//             ExpressionType::OLT => ExpressionType::OGT,
//             _ => match exp_type {
//                 ExpressionType::OLE => ExpressionType::OGE,
//                 _ => match exp_type {
//                     ExpressionType::OGE => ExpressionType::OLE,
//                     _ => match exp_type {
//                         ExpressionType::OGT => ExpressionType::OLT,
//                         _ => *exp_type,
//                     },
//                 },
//             },
//         }
//     }
//
//     pub fn logical_inversion(exp_type: &ExpressionType) -> ExpressionType {
//         match exp_type {
//             ExpressionType::OLT => ExpressionType::OGE,
//             _ => match exp_type {
//                 ExpressionType::OLE => ExpressionType::OGT,
//                 _ => match exp_type {
//                     ExpressionType::OGT => ExpressionType::OLE,
//                     _ => match exp_type {
//                         ExpressionType::ONE => ExpressionType::OEQ,
//                         _ => match exp_type {
//                             ExpressionType::OEQ => ExpressionType::ONE,
//                             _ => match exp_type {
//                                 ExpressionType::OIN => ExpressionType::ONOTIN,
//                                 _ => match exp_type {
//                                     ExpressionType::ONOTIN => ExpressionType::OIN,
//                                     _ => match exp_type {
//                                         ExpressionType::OSUBSET => ExpressionType::OSUPSET,
//                                         _ => match exp_type {
//                                             ExpressionType::OSUPSET => ExpressionType::OSUBSET,
//                                             _ => match exp_type {
//                                                 ExpressionType::OSUPSEQ => ExpressionType::OSUBSET,
//                                                 _ => match exp_type {
//                                                     ExpressionType::OSUPSET => {
//                                                         ExpressionType::OSUPSEQ
//                                                     }
//                                                     _ => ExpressionType::OUNDEF,
//                                                 },
//                                             },
//                                         },
//                                     },
//                                 },
//                             },
//                         },
//                     },
//                 },
//             },
//         }
//     }
//
//     pub fn operator_to_string(exp_type: &ExpressionType) -> String {
//         match exp_type {
//             ExpressionType::ONEG => "neg".to_string(),
//             ExpressionType::OABS => "abs".to_string(),
//
//             ExpressionType::OADD => "add".to_string(),
//             ExpressionType::OMUL => "mul".to_string(),
//             ExpressionType::OSUB => "sub".to_string(),
//             ExpressionType::ODIV => "div".to_string(),
//             ExpressionType::OMOD => "mod".to_string(),
//
//             ExpressionType::OSQR => "sqr".to_string(),
//             ExpressionType::OPOW => "pow".to_string(),
//
//             ExpressionType::OMIN => "min".to_string(),
//             ExpressionType::OMAX => "max".to_string(),
//             ExpressionType::ODIST => "dist".to_string(),
//
//             ExpressionType::ONE => "ne".to_string(),
//             ExpressionType::OEQ => "eq".to_string(),
//
//             ExpressionType::OLE => "le".to_string(),
//             ExpressionType::OLT => "lt".to_string(),
//             ExpressionType::OGE => "ge".to_string(),
//             ExpressionType::OGT => "gt".to_string(),
//
//             ExpressionType::ONOT => "not".to_string(),
//             ExpressionType::OAND => "and".to_string(),
//             ExpressionType::OOR => "or".to_string(),
//             ExpressionType::OXOR => "xor".to_string(),
//             ExpressionType::OIMP => "imp".to_string(),
//             ExpressionType::OIFF => "iff".to_string(),
//             ExpressionType::OIF => "if".to_string(),
//
//             ExpressionType::OIN => "in".to_string(),
//             ExpressionType::ONOTIN => "notin".to_string(),
//             ExpressionType::OSET => "set".to_string(),
//             _ => "undefined".to_string(),
//         }
//     }
//
//     pub fn create_node_operator(op: &str) -> Option<Box<dyn NodeTraits>> {
//         match op {
//             "neg" => Some(Box::<NodeNeg<'_>>::default()),
//             "abs" => Some(Box::<NodeAbs<'_>>::default()),
//
//             "sqr" => Some(Box::<NodeSquare<'_>>::default()),
//             "pow" => Some(Box::<NodePow<'_>>::default()),
//
//             "add" => Some(Box::<NodeAdd<'_>>::default()),
//             "mul" => Some(Box::<NodeMult<'_>>::default()),
//             "div" => Some(Box::<NodeDiv<'_>>::default()),
//             "mod" => Some(Box::<NodeMod<'_>>::default()),
//             "sub" => Some(Box::<NodeSub<'_>>::default()),
//
//             "min" => Some(Box::<NodeMin<'_>>::default()),
//             "max" => Some(Box::<NodeMax<'_>>::default()),
//             "dist" => Some(Box::<NodeDist<'_>>::default()),
//
//             "ne" => Some(Box::<NodeNE<'_>>::default()),
//             "eq" => Some(Box::<NodeEQ<'_>>::default()),
//
//             "le" => Some(Box::<NodeLE<'_>>::default()),
//             "lt" => Some(Box::<NodeLT<'_>>::default()),
//             "ge" => Some(Box::<NodeGE<'_>>::default()),
//             "gt" => Some(Box::<NodeGT<'_>>::default()),
//
//             "not" => Some(Box::<NodeNot<'_>>::default()),
//             "and" => Some(Box::<NodeAnd<'_>>::default()),
//             "or" => Some(Box::<NodeOr<'_>>::default()),
//             "xor" => Some(Box::<NodeXor<'_>>::default()),
//             "imp" => Some(Box::<NodeImp<'_>>::default()),
//             "iff" => Some(Box::<NodeIff<'_>>::default()),
//             "if" => Some(Box::<NodeIf<'_>>::default()),
//             "in" => Some(Box::<NodeIn<'_>>::default()),
//             "notin" => Some(Box::<NodeNotIn<'_>>::default()),
//             "set" => Some(Box::<NodeSet<'_>>::default()),
//             _ => None,
//         }
//     }
//
//     impl<'a> Default for NodeDist<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeNotIn<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeSet<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeImp<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeIff<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeIf<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeIn<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeNot<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeAnd<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeOr<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeXor<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeLE<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeLT<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeGE<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeGT<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodePow<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeSquare<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeAbs<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeNeg<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeAdd<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeMult<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeDiv<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeMod<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeSub<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeMin<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeMax<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeNE<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     impl<'a> Default for NodeEQ<'a> {
//         fn default() -> Self {
//             Self::new()
//         }
//     }
//
//     pub fn equal_nodes(a: &dyn NodeTraits, b: &dyn NodeTraits) -> i32 {
//         return match a.get_cons_or_var_or_ope() {
//             NodeType::Constant => match b.get_cons_or_var_or_ope() {
//                 NodeType::Constant => a.get_value() - b.get_value(),
//                 _ => get_order(a.get_express_type()) - get_order(b.get_express_type()),
//             },
//             NodeType::Variable => match a.get_cons_or_var_or_ope() {
//                 NodeType::Variable => match a.get_variable().cmp(&b.get_variable()) {
//                     Ordering::Less => -1,
//                     Ordering::Equal => 0,
//                     Ordering::Greater => 1,
//                 },
//                 _ => get_order(a.get_express_type()) - get_order(b.get_express_type()),
//             },
//             NodeType::Operator => match a.get_cons_or_var_or_ope() {
//                 NodeType::Operator => {
//                     let ap = a.get_parameters();
//                     let bp = b.get_parameters();
//                     if ap.len() < bp.len() {
//                         -1
//                     } else if ap.len() < bp.len() {
//                         1
//                     } else {
//                         for (i, _) in ap.iter().enumerate() {
//                             let cmp = equal_nodes(ap[i], bp[i]);
//                             if cmp != 0 {
//                                 return cmp;
//                             }
//                         }
//                         0
//                     }
//                 }
//                 _ => get_order(a.get_express_type()) - get_order(b.get_express_type()),
//             },
//         };
//     }
//     pub fn compare_nodes(a: &dyn NodeTraits, b: &dyn NodeTraits) -> bool {
//         equal_nodes(a, b) < 0
//     }
// }
