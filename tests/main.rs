use walkdir::WalkDir;
use xcsp3_rust::constraints::xconstraint_type::xcsp3_core::XConstraintType;
use xcsp3_rust::utils::time_interval::xcsp3_utils::TimeInterval;
use xcsp3_rust::variables::xvariable_type::xcsp3_core::XVariableType;
use xcsp3_rust::xcsp_xml::xcsp_xml_model::xcsp3_xml::XcspXmlModel;

/**
* <p>@project_name: xcsp3-rust
* </p>
* <p>@author: luhan zhen
* </p>
* <p>@date:  2023/7/11 18:41
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 **/

fn main() {
    // let tree = ExpressionTree::from_str("eq( add(%0 ,mul(1, %2,x[3][4][2]), %4 ), %5 )");
    //
    // // for e in tree.unwrap().first_order_iter() {
    // //     print!("{:?}", e.to_string());
    // // }
    // // print!("\n");
    // println!("tree = {}", tree.unwrap().to_string());
    // let tree = ExpressionTree::from_str("eq(add(%0,%1),%2)");
    // // for e in tree.unwrap().first_order_iter() {
    // //     print!("{:?}", e.to_string());
    // // }
    // // print!("\n");
    // println!("tree = {}", tree.unwrap().to_string());
    // let tree = ExpressionTree::from_str("eq(add(x,y),z)");
    // // for e in tree.unwrap().first_order_iter() {
    // //     print!("{:?}", e.to_string());
    // // }
    // // print!("\n");
    // println!("tree = {}", tree.unwrap().to_string());
    // let tree = ExpressionTree::from_str("eq(%0,dist(%1, and(1,2,3,x[4])))");
    // // for e in tree.unwrap().first_order_iter() {
    // //     print!("{:?}", e.to_string());
    // // }
    // // print!("\n");
    // println!("tree = {}", tree.unwrap().to_string());
    // test_single();
    test_all();
}

fn test_single() {
    let xml_file = ".//instances//my-example.xml";
    // let xml_file = ".//instances//Bibd-sc-22-033-12-08-04.xml";
    // let xml_file = ".//instances//Allergy.xml";
    // let xml_file = ".//instances//Subisomorphism-g05-g18.xml";
    let model = XcspXmlModel::from_path(xml_file).unwrap();
    let variable = model.build_variables();
    println!("variables:");
    for v in variable.iter() {
        println!("\t{}", v)
    }
    println!("constraints:");
    for c in model.build_constraints(&variable).iter_mut() {
        println!("\t{}", c);
        // match c{
        //     XConstraintType::XConstraintNone(_) => {}
        //     XConstraintType::XExtension(_) => {}
        //     XConstraintType::XAllDifferent(_) => {}
        //     XConstraintType::XAllDifferentExcept(_) => {}
        //     XConstraintType::XInstantiation(_) => {}
        //     XConstraintType::XAllEqual(_) => {}
        //     XConstraintType::XOrdered(x) => {let r = x.get_scope();
        //     println!("{},{}",r[0].0,r[0].1.to_string());}
        //     XConstraintType::XRegular(_) => {}
        //     XConstraintType::XMdd(_) => {}
        //     XConstraintType::XIntention(_) => {}
        //     XConstraintType::XGroup(_) => {}
        //     XConstraintType::XSum(_) => {}
        //     XConstraintType::XMaximum(_) => {}
        //     XConstraintType::XMinimum(_) => {}
        // }
    }
    for o in model.build_objectives().iter() {}
}

fn test_all() {
    let start = TimeInterval::new();
    let aa = WalkDir::new("./instances")
    // let aa = WalkDir::new("D://XCSP3")
    // let aa = WalkDir::new("D:\\XCSP3\\BinPacking\\BinPacking-tab-sw100")
    // let aa = WalkDir::new("D:\\XCSP3\\Subisomorphism\\Subisomorphism-m1-LV")
        // let aa = WalkDir::new("D:\\XCSP3\\AllInterval\\AllInterval-m1-s1")
        // let aa = WalkDir::new("D:\\XCSP3\\Bibd")
        // let aa = WalkDir::new("D:\\XCSP3\\Basic\\Basic-m1-s1")
        .into_iter()
        .filter_map(|file| file.ok());

    for file in aa {
        // !file.path().display().to_string().contains("Bibd") &&
        if file.metadata().unwrap().is_file() && file.path().display().to_string().ends_with(".xml")
        {
            let model = XcspXmlModel::from_path(&file.path().display().to_string());
            match model {
                Err(_) => eprintln!("Err {}", file.path().display()),
                Ok(m) => {
                    let ti = TimeInterval::new();
                    let variable = m.build_variables();
                    // for v in variable.iter() {
                    //     println!("{}", v.to_string())
                    // }
                    for v in variable.iter() {
                        if let XVariableType::XVariableNone(e) = v {
                            eprintln!("Err {} {}", file.path().display(), e);
                            break;
                        }
                    }
                    for c in m.build_constraints(&variable).iter() {
                        if let XConstraintType::XConstraintNone(e) = c {
                            eprintln!("Err {} {}", file.path().display(), e);
                            break;
                        }
                    }
                    println!(
                        "parse the instance named {} by {:?}.",
                        file.path().display(),
                        ti.get()
                    );
                    // println!("Done {}", file.path().display());
                }
            }
        }
    }

    println!("Time elapsed in parse all instances is: {:?}", start.get());
}

// Time elapsed in parse all instances is: 346.9358413s

// let start = Instant::now();
// for entry in glob("D:\\XCSP3\\Langford\\Langford-m2-s1//**/*.xml").unwrap() {
// // println!("{}", entry.unwrap().display());
// let path = entry.unwrap().display().to_string();
// let xml = fs::read_to_string(path).unwrap();
// // println!("{}", xml);
// let model: Option<Instance> = match from_str(&xml) {
// Ok(model) => Some(model),
// Err(_err) => None,
// };
// // match model {
// //     None => eprintln!("Err {}", file.path().display()),
// //     _ => println!("Done {}", file.path().display())
// // }
// }
// let duration = start.elapsed();
// println!("Time elapsed in parse all instances is: {:?}", duration);
