use std::{
    collections::HashMap, error::Error, os::windows::fs::MetadataExt, path::PathBuf, time::{Duration, SystemTime}
};
use walkdir::WalkDir;

///`Cargos` holds informations about found crates
pub struct Cargos {

    ///cargos names and sizes(MB)
    pub crates: HashMap<String, u64>,

    ///cargos last build
    pub last_build: HashMap<String, u64>,


    ///paths that will be erased after confirmation or if `--sure` is flagged.
    pub target_path: Vec<PathBuf>,
}

impl Cargos {

    ///search and classify cargos in path
    pub fn from_dir(path: &PathBuf, criteria: &SearchCriteria) -> Result<Self, Box<dyn Error>> {
        if !path.is_dir() {
            return Err("Path is not valid".into());
        }

        let mut crates = HashMap::new();
        let mut last_build = HashMap::new();
        let mut target_path = Vec::new();
        for entry in WalkDir::new(path) {
            let dir_name = match entry {
                Ok(dn) => dn,
                Err(_) => continue, //Access to some OS folder is restricted, just ignore it
            };

            let file_name = dir_name.file_name().to_str().unwrap().to_string();
            //std::io::stdout().write(b"")?;
            //let searching=format!("Searching -> {:?}",path);
            //println!("{}",searching.blue());

            if file_name.eq("Cargo.toml") {
                let path = dir_name.path().parent();
                if let Some(p) = path {
                    let meta = p.metadata()?;
                    let modified = meta.modified()?;
                    let now = SystemTime::now();
                    let crate_name = p.iter().last().unwrap().to_str().unwrap().to_string();
                    let mut last = now.duration_since(modified)?;
                    let mut build_size = 0;
                    let mut temp_path: PathBuf = PathBuf::new().join("RESTRICTED_AREA");

                    
                    for dir in WalkDir::new(p) {
                        let entry = dir?;
                        if entry.path().is_dir() {
                            let folder_path = entry.path();

                            let folder_name = folder_path
                                .iter()
                                .last()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_ascii_lowercase();

                            if folder_name.eq("target") {
                                temp_path = folder_path.to_path_buf();
                                println!("temp: {:?}",temp_path);
                                for dir in WalkDir::new(folder_path) {
                                    let entry = dir?;
                                    if entry.path().is_file() {
                                        let filen = entry.path().metadata()?.file_size();
                                        build_size += filen;
                                    }

                                    //find timestamp file
                                    if entry.path().is_file() {
                                        let file_path = entry.path().to_path_buf();
                                        if let Some(path_str) = file_path.to_str() {
                                            if path_str.ends_with("invoked.timestamp") {
                                                //println!("timestamp found");
                                                if let Ok(meta) = entry.metadata() {
                                                    last = meta.modified()?.elapsed()?;
                                                }
                                            } else {
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    //println!("build-size: {}",build_size);
                    if build_size >= criteria.build_size && last >= criteria.last_mod {
                        crates.insert(crate_name.clone(), build_size / (1024 * 1024));
                        let last_mod_in_hours = last.as_secs() / 3600;
                        last_build.insert(crate_name, last_mod_in_hours);
                        target_path.push(temp_path);

                    }
                }
            }
        }

        let mut out=Self{
            crates,
            last_build,
            target_path
        };
        out.filter();
        Ok(out)
    }

    
    pub fn cargo_count(&self) -> u16 {
        self.crates.len() as u16
    }

    pub fn filter(&mut self) {
        self.crates.retain(|_, s| *s > 0);

    }

}


///`SearchCriteria` applies conditions to the search process.
pub struct SearchCriteria {
    ///last modified in hours
    
    last_mod: Duration,

    ///build size in MB
    build_size: u64,
}

impl SearchCriteria {
    pub fn new(last_mod: Duration, build_size: u64) -> Self {
        SearchCriteria {
            last_mod,
            build_size,
        }
    }
}
