use std::error::Error;
use std::fs::{self};
use std::os::windows::fs::MetadataExt;
use std::path::PathBuf;
use colored::Colorize;

///the part responsible of cleaning artifacts after confirmation or without 
/// if `--sure` is passed 
/// symbolic links will be ignored for saftey purposes.

pub fn clean_batch(paths: &Vec<PathBuf>) -> Result<f64, Box<dyn Error>> {
    let mut size_reclaimed = 0;
    for p in paths {
        //if symbolic link reached the deletion section,
        //it will be skipped for safety
        //symbolic links can do very bad things
        //symbolic links can be processed in search routine
        //but i have left it for clarification if you have 
        //some weird nested files .
        if p.is_dir() {
            let symlink_meta=p.symlink_metadata()?;
            if symlink_meta.file_type().is_symlink(){
                let skip=format!("⚠️ Skipping symlink for safety: {:?}",p);
                eprintln!("{}",skip.yellow());
                continue;
            }

            for dir in walkdir::WalkDir::new(p) {
                match dir {
                    Ok(di) => {
                        let sub = di.path();
                        if sub.is_file() {
                            if let Ok(meta) = sub.metadata() {
                                size_reclaimed += meta.file_size();
                            }
                        }
                    }
                    _ => {}
                }
            }
            fs::remove_dir_all(p)?;
        }
    }
    let size_reclaimed = (size_reclaimed as f64) / (1024.0 * 1024.0);
    Ok(size_reclaimed)
}
