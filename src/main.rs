use rayon::prelude::*;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let (from, extension) = if env::args().count() == 3 {
        (env::args().nth(1).unwrap(), env::args().nth(2).unwrap())
    } else {
        eprintln!("Error: Missing Args!\nUsage: ./img_seq_cvtr [folderpath] [extension]\ne.g. ./img_seq_cvtr ~/tiff_seq png");
        std::process::exit(1);
    };

    if !Path::new(&from).is_dir() {
        eprintln!("Error: given input path is invalid");
        std::process::exit(1);
    }

    //scans input directory and converts to collection of paths
    let paths: Vec<PathBuf> = std::fs::read_dir(&from)
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect();
    //construct output dir string
    let output_dir = format!("{}_converted_to_{}", &from, &extension);
    match std::fs::create_dir(&output_dir) {
        Ok(_) => {
            let now = std::time::Instant::now();
            paths.into_par_iter().for_each(|in_path| {
                let mut out_path = PathBuf::from(&output_dir); //parent path
                out_path.push(in_path.file_stem().unwrap().to_str().unwrap()); //read and append filename to parent
                out_path.set_extension(&extension);

                if let Ok(tiff) = image::open(&in_path) {
                    tiff.save(out_path).unwrap();
                }
            });
            println!("{}", now.elapsed().as_millis());
        }
        Err(e) => {
            eprintln!("Unable to create output directory\nError: {:?}", e);
        }
    }
}
