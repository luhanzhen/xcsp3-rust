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
// fn main() {
//     let xml_file = ".//instances//my-example.xml";
//     let model = Instance::from_path(xml_file).unwrap();
//     let variable = model.build_variables();
//     for v in variable.iter() {
//         println!("{}", v.to_string())
//     }
//     for c in model.build_constraints().iter() {
//         println!("{}", c.to_string())
//     }
// }
use std::time::Instant;
use walkdir::WalkDir;
// use xcsp3_rust::variables::xvariable_type::xcsp3_core::XVariableType;
use xcsp3_rust::xcsp_xml::xcsp_xml_model::xcsp3_xml::XcspXmlModel;

fn main() {
    let start = Instant::now();
    // let aa = WalkDir::new("D://XCSP3")
    // let aa = WalkDir::new("D:\\XCSP3\\AllInterval\\AllInterval-m1-s1")
    // let aa = WalkDir::new("D:\\XCSP3\\Bibd")
    let aa = WalkDir::new("D:\\XCSP3\\Basic\\Basic-m1-s1")
        .into_iter()
        .filter_map(|file| file.ok());

    for file in aa {
        if file.metadata().unwrap().is_file() && file.path().display().to_string().ends_with(".xml")
        {
            // println!("{}", file.path().display());

            let model = XcspXmlModel::from_path(&file.path().display().to_string());
            match model {
                Err(_) => eprintln!("Err {}", file.path().display()),
                Ok(m) => {
                    let variable = m.build_variables();
                    for v in variable.iter() {
                        println!("{}", v.to_string())
                    }
                    // for v in variable.iter() {
                    //     if let XVariableType::XVariableNone = v {
                    //         eprintln!("Err {}", file.path().display());
                    //         break;
                    //     }
                    // }
                    m.build_constraints();
                    // println!("Done {}", file.path().display());
                }
            }
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in parse all instances is: {:?}", duration);
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
