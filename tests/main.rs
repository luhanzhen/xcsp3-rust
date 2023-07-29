
use walkdir::WalkDir;
use xcsp3_rust::constraints::xconstraint_type::xcsp3_core::XConstraintType;
use xcsp3_rust::utils::time_interval::xcsp3_utils::TimeInterval;
use xcsp3_rust::variables::xvariable_type::xcsp3_core::XVariableType;
use xcsp3_rust::xcsp_xml::xcsp_xml_model::xcsp3_xml::XcspXmlModel;

/*
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
    // let mut e: Bitmap<100> = Bitmap::new();
    // e.set(3, true);
    // println!("{:?}", e);
    test_single();
    // test_all();
}

fn test_single() {
    let xml_file = ".//instances//my-example.xml";
    // let xml_file = ".//instances//Hanoi-03.xml";
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
    }
    println!("objectives:");
    for o in model.build_objectives(&variable).iter() {
        println!("\t{}", o);
        // match o {
        //     XObjectivesType::XObjectiveNone(_) => {}
        //     XObjectivesType::Minimize(e) => match e {
        //         XObjective::XObjectiveElement(ee) => {}
        //         XObjective::XObjectiveExpression(ex) => {}
        //     },
        //     XObjectivesType::Maximize(_) => {}
        // }
    }
}

fn test_all() {
    let start = TimeInterval::new();
    // let aa = WalkDir::new("./instances")
    //     let aa = WalkDir::new("D://XCSP3")
    // let aa = WalkDir::new("D:\\XCSP3\\Hanoi\\Hanoi-m1-s1")
    // let aa = WalkDir::new("D:\\XCSP3\\Nonogram\\Nonogram-regular-gp")
    let aa = WalkDir::new("D:\\XCSP3\\MagicSquare\\MagicSquare-mdd-s1")
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
            let ti = TimeInterval::new();
            let model = XcspXmlModel::from_path(&file.path().display().to_string());
            match model {
                Err(_) => eprintln!("Err {}", file.path().display()),
                Ok(m) => {
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
                    if ti.get().as_secs_f32() > 0.5 {
                        println!(
                            "parse the instance named {} by {:?}.",
                            file.path().display(),
                            ti.get()
                        );
                    }

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
