use rayon::prelude::*;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let (from, extension) = if env::args_os().count() == 3 {
        (
            env::args_os().nth(1).unwrap(),
            env::args_os().nth(2).unwrap(),
        )
    } else {
        println!("Error: Missing Args!\nUsage: ./img_seq_cvtr [folderpath] [extension]\ne.g. ./img_seq_cvtr ~/tiff_seq png");
        std::process::exit(1);
    };

    if !Path::new(&from).is_dir() {
        println!("Error: given path is invalid");
        std::process::exit(1);
    }

    //scans input directory and converts to collection of paths
    let paths: Vec<PathBuf> = std::fs::read_dir(&from)
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect();
    //add output path
    let output_dir = format!(
        "{}_converted_to_{}",
        from.to_str().unwrap(),
        extension.to_str().unwrap()
    );
    match std::fs::create_dir(&output_dir) {
        Ok(_) => {
            paths.into_par_iter().for_each(|in_path| {
                let mut out_path = PathBuf::from(&output_dir);
                out_path.push(in_path.file_stem().unwrap().to_str().unwrap());
                out_path.set_extension(extension.to_str().unwrap());

                let tiff = image::open(&in_path).unwrap();
                tiff.save(out_path).unwrap();
            });
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
