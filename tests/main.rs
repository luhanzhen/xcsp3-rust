// use std::fs;
// use std::time::Instant;

use quick_xml::de::from_str;
use std::fs;
use std::time::Instant;
use walkdir::WalkDir;
// use quick_xml::de::from_str;
// use walkdir::WalkDir;
use xcsp3_rust::xcsp3skeleton::xcsp3_core::Instance;

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
//
// fn main() {
//     let xml_file = ".//instances//my-example.xml";
//     let model = Instance::from_path(xml_file).unwrap();
//     model.build_variables();
//     // model.build_constraints();
// }

fn main() {
    let start = Instant::now();
    // let aa = WalkDir::new("D://XCSP3")
    // let aa = WalkDir::new("D:\\XCSP3\\AllInterval\\AllInterval-m1-s1")
    let aa = WalkDir::new("D:\\XCSP3\\Bibd")
        .into_iter()
        .filter_map(|file| file.ok());

    for file in aa {
        if file.metadata().unwrap().is_file() && file.path().display().to_string().ends_with(".xml")
        {
            // println!("{}", file.path().display());
            let xml = fs::read_to_string(file.path().display().to_string()).unwrap();
            // println!("{}", xml);
            let model: Option<Instance> = match from_str(&xml) {
                Ok(m) => Some(m),
                Err(_err) => None,
            };

            match model {
                None => eprintln!("Err {}", file.path().display()),
                Some(m) => {
                    m.build_variables();
                    println!("Done {}", file.path().display());
                }
            }
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in parse all instances is: {:?}", duration);
}

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
