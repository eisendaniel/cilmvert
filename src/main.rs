use rayon::prelude::*;
use std::env;
use std::path::PathBuf;

fn main() {
    //reading in dir to convert
    let (from, extension) = if env::args().count() == 3 {
        (env::args().nth(1).unwrap(), env::args().nth(2).unwrap())
    } else {
        panic!("Error: Missing Args!\nUsage: climvert [folderpath] [extension]\ne.g. climvert ~/tiff_seq png")
    };

    if !PathBuf::from(&from).is_dir() {
        panic!("Error: given input path is invalid")
     }

    //scans input directory and converts to collection of paths
    let paths: Vec<PathBuf> = std::fs::read_dir(&from)
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect();
    //construct output dir string
    let output_dir = format!("{}_converted_to_{}", &from, &extension);
    match std::fs::create_dir(&output_dir) {
        //did the dir get created?
        Ok(_) => {
            // yes?
            paths.into_par_iter().for_each(|in_path| {
                //parallel convert each image
                let mut out_path = PathBuf::from(&output_dir); //parent path
                out_path.push(in_path.file_stem().unwrap().to_str().unwrap()); //read and append filename to parent
                out_path.set_extension(&extension);

                if let Ok(image) = image::open(&in_path) {
                    image.save(out_path).unwrap();
                }
            });
        }
        Err(e) => {
            // no?
            eprintln!("Unable to create output directory\nError: {:?}", e);
        }
    }
}
