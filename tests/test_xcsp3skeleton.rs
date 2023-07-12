/**
 * <p>@project_name: XCSP3-Rust
 * </p>
 * <p>@author: luhanzhen
 * </p>
 * <p>@date: 2023/7/7
 * </p>
 * <p>@time: 10:19
 * </p>
 * <p>@this_file_name:test_xcsp3skeleton
 * </p>
 **/
#[cfg(test)]
mod test_xcsp3skeleton {
    use xcsp3_rust::xcsp3skeleton::xcsp3_core::Instance;

    // #[should_panic(expected = "not sequence domain")]

    // fn get_all_xml_files(path: &str) {
    //     // for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
    //     //     println!("{}", file.path().display());
    //     // }
    //     let aa = WalkDir::new(path).into_iter().filter_map(|file| file.ok());
    //     for file in aa {
    //         if file.metadata().unwrap().is_file()
    //             && file.path().display().to_string().ends_with(".xml")
    //         {
    //             println!("{}", file.path().display());
    //             let xml = fs::read_to_string(file.path().display().to_string()).unwrap();
    //             println!("{}", xml);
    //             // let model: Option<Instance> = match from_str(&xml) {
    //             //     Ok(model) => Some(model),
    //             //     Err(_err) => None,
    //             // };
    //             // match model {
    //             //     None => eprintln!("{}", file.path().display()),
    //             //     _ => {}
    //             // }
    //         }
    //     }
    // for entry in WalkDir::new(path)
    //     .follow_links(true)
    //     .into_iter()
    //     .filter_map(|e| e.ok()) {
    //     let f_name = entry.file_name().to_string_lossy();
    //     if  f_name.ends_with(".xml")  {
    //
    //         println!("{}\n", entry.path().to_string_lossy());
    //         // let xml = fs::read_to_string(&f_name.to_string()).unwrap();
    //         // let model: Option<Instance> = match from_str(&xml) {
    //         //     Ok(model) => Some(model),
    //         //     Err(_err) => None,
    //         // };
    //         // match model {
    //         //     None => println!("{}", f_name),
    //         //     _ => {},
    //         // }
    //
    //
    //     }
    // }
    // }

    #[test]
    fn test_skeleton_all() {
        // get_all_xml_files("D://XCSP3");
        // let paths = fs::read_dir("D://XCSP3").unwrap();
        //
        // for path in paths {
        //     println!("Name: {}", path.unwrap().path().display())
        // }
        // for entry in fs::read_dir("D://XCSP3").unwrap() {
        //
        //     let path = entry.unwrap().path();
        //     // Get path string.
        //     let path_str = path.to_str().unwrap();
        //     println!("PATH: {}", path_str);
        // }
    }

    #[test]
    fn test_skeleton() {
        // let xml_file = ".\\instances\\radi_X2.xml";
        // let xml_file = ".\\instances\\AllInterval-009.xml";
        // let xml_file = ".\\instances\\Dubois-015.xml";

        // let xml_file = ".\\instances\\Allergy.xml";
        // let xml_file = ".\\instances\\Bibd-sc-22-033-12-08-04.xml";
        // let xml_file = ".\\instances\\Crossword-lex-h0502.xml";
        // let xml_file = ".\\instances\\Sat-flat200-00-clause.xml";
        // let xml_file = ".\\instances\\TravellingSalesman-100-50-16.xml";
        // let xml_file = "D:\\XCSP3\\Mario\\Mario-zinc-s1\\Mario-t-hard-2.xml";

        let xml_file = ".\\instances\\my-example.xml";
        let model = Instance::from_path(xml_file).unwrap();
        model.build_constraints();
        // let xml = fs::read_to_string(&xml_file).unwrap();
        // // println!("{}", xml);
        //
        // let model: Instance = match from_str(&xml) {
        //     Ok(model) => model,
        //     Err(err) => panic!("{}", err),
        // };
        // println!("{:?}\n", model);
        // for v in model.variables.unwrap().variables.iter() {
        //     println!("{:?}\n", v);
        // }
        // for e in model.constraints.constraints.iter() {
        //     println!("{:?}\n", e);
        // match e
        // {
        //     ConstraintType::Group(_) => {}
        //     ConstraintType::Block(_) => {}
        //     ConstraintType::AllDifferent { .. } => {}
        //     ConstraintType::AllEqual { .. } => {}
        //     ConstraintType::Ordered { .. } => {}
        //     ConstraintType::Intension { .. } => {}
        //     ConstraintType::Extension { .. } => {}
        //     ConstraintType::Regular { .. } => {}
        //     ConstraintType::Mdd { .. } => {}
        //     ConstraintType::Sum { .. } => {}
        //     ConstraintType::Count { .. } => {}
        //     ConstraintType::NValues { .. } => {}
        //     ConstraintType::Cardinality { .. } => {}
        //     ConstraintType::Minimum { .. } => {}
        //     ConstraintType::Maximum { .. } => {}
        //     ConstraintType::Element { .. } => {}
        // }
        // }
        // assert_eq!(42, 42);
    }
}
