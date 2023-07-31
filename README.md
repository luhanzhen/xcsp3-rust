
<h1 > <div style="text-align: center;"><b>xcsp3-rust </b></div>  </h1>

[//]: # ([![Crate]&#40;https://img.shields.io/crates/v/quick-xml.svg&#41;]&#40;https://crates.io/crates/quick-xml&#41;)

[![xcsp3](https://img.shields.io/badge/xcsp3-red)](http://xcsp.org)
[![xcsp3rust](https://img.shields.io/badge/xcsp3_rust-8A2BE2)](https://github.com/luhanzhen/xcsp3-rust)
[![docs.rs](https://docs.rs/xcsp3-rust/badge.svg)](https://docs.rs/xcsp3-rust)
[![MSRV](https://img.shields.io/badge/rustc-1.70.0-90c541.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)
[![License](https://img.shields.io/badge/License-_MIT-blue)](https://github.com/luhanzhen/xcsp3-rust/LICENSE)


## Description
### This lib is implemented by rust and is licensed under the MIT license. 
### The purpose of this library is to read XCSP files into rust constraint programming solvers.
### You can learn about the semantics of XCSP3 through this site http://xcsp.org/.
### I will keep improving this code to support more constraints and fix possible bugs.
### If you have something to tell me, feel free to contact me.


## Usage

### Just add the following dependency to your project's Cargo.toml file:

```toml
[dependencies]
xcsp3-rust = "0.1.0"
```

### The library is automatically built and statically linked to your binary.

## Example

```rust
    fn main()
    {
        let xml_file = ".//instances//my-example.xml";
        let model = XcspXmlModel::from_path(xml_file).unwrap();
        let variable = model.build_variables();
        
        println!("variables:");
        for v in variable.iter() {
            println!("\t{}", v);
        }
        
        println!("constraints:");
        for c in model.build_constraints(&variable).iter() {
            println!("\t{}", c);
        }
        
        println!("objectives:");
        for o in model.build_objectives(&variable).iter() {
            println!("\t{}", o);
        }
    }
```

## Architecture Graph

### main architecture
```mermaid
graph BT
A["XCSP(xml file)"] --serde--> B(XcspXmlModel)
B --parser--> C([XVariableSet])
B --parser--> D([XConstraintSet])
B --parser--> E([XObjectivesSet])
C --reader--> F[/main.rs/]
D --reader--> F
E --reader--> F

```
### XVariableSet
```mermaid
graph LR
    C([XVariableSet]) -.->XVariableType(XVariableType)
    XVariableType -->  XVariableArray(XVariableArray)
    XVariableType -->  XVariableInt(XVariableInt)
    XVariableType -->  XVariableTree(XVariableTree)
    XVariableTree -.domain.->  XDomainInteger(XDomainInteger)
    XVariableInt -.domain.->  XDomainInteger
    XVariableArray -.domain.->  XDomainInteger
    XDomainInteger -.-> XIntegerType(XIntegerType)
    XIntegerType -->IntegerValue(IntegerValue)
    XIntegerType -->IntegerInterval(IntegerInterval)
    XIntegerType -->XIntegerSymbolic(XIntegerSymbolic)

```
### XConstraintSet
```mermaid
graph LR
    D([XConstraintSet]) -.-> XConstraintType(XConstraintType)
    XConstraintType -->  XExtension(XExtension) -.scope.-> Scope(XVarVal)
    XConstraintType --> XAllDifferent(XAllDifferent)-.scope.-> Scope
    XConstraintType --> XAllDifferentExcept(XAllDifferentExcept)-.scope.-> Scope
    XConstraintType --> XInstantiation(XInstantiation)-.scope.-> Scope
    XConstraintType --> XAllEqual(XAllEqual)-.scope.-> Scope
    XConstraintType --> XOrdered(XOrdered)-.scope.-> Scope
    XConstraintType --> XRegular(XRegular)-.scope.-> Scope
    XConstraintType -->XMdd(XMdd)-.scope.-> Scope
    XConstraintType -->XIntention(XIntention)-.scope.-> Scope
    XConstraintType -->XGroup(XGroup)-.scope.-> Scope
    XConstraintType -->XSum(XSum)-.scope.-> Scope
    XConstraintType -->XMaximum(XMaxMin)-.scope.-> Scope
    XConstraintType -->XMinimum(XMaxMin)-.scope.-> Scope
    XConstraintType -->XElement(XElement)-.scope.-> Scope
    XConstraintType -->XSlide(XSlide)-.scope.-> Scope
    XConstraintType -->XCount(XCount)-.scope.-> Scope
    XConstraintType -->XNValues(XNValues)-.scope.-> Scope
    XConstraintType -->XCardinality(XCardinality)-.scope.-> Scope
    XConstraintType -->XChannel(XChannel)-.scope.-> Scope
    XConstraintType -->XCumulative(XCumulative)-.scope.-> Scope
    XConstraintType -->XNoOverlap(XNoOverlap)-.scope.-> Scope
    XConstraintType --> XStretch(XStretch)-.scope.-> Scope
    Scope -->IntVar(IntVar is a variable)
    Scope -->IntVal(IntVal is a value)
```
### XObjectivesSet
```mermaid
graph LR
    E([XObjectivesSet]) -.-> T(XObjectivesType)
%%    XVariableArray(XVariableArray)
    T --> Minimize(Minimize)
    T --> Maximize(Maximize)
    Minimize --> XObjective(XObjective)
    Maximize --> XObjective(XObjective)
    XObjective --> XObjectiveElement(XObjectiveElement)
    XObjective --> XObjectiveExpression(XObjectiveExpression)
```

## License
> MIT License

## Author
> luhan zhen

> tip: Maybe my code is not the best, but I will keep improving it to better build our 'CP' community.


