extern crate notify;
use std::fs::{self, ReadDir};
use std::path::Path;
use toml::Value;
use std::{error, eprintln};

use std::sync::mpsc::channel;
use std::time::Duration;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, EventKind, event};


fn divide(file_path: &Path) -> Result<(String, String), Box<dyn std::error::Error>> {
    let file_stem = file_path.file_stem().unwrap().to_str().unwrap(); 
    let file_stem = Path::new(file_stem).file_stem().unwrap().to_str().unwrap();
    let extension = file_path.to_str().unwrap().replace(file_stem, "");

    Ok((file_stem.to_string(), extension))
}


fn create_dir(name_dir: &str) -> std::io::Result<()> { 

    let dir_list = fs::read_dir("..")?; 

    for file_or_dir in dir_list { 
        let file_or_dir_str: String = file_or_dir?.file_name().to_string_lossy().to_string();
        let file_or_dir_str_path = format!("../{}", &file_or_dir_str);
        let file_or_dir_str_metadata = fs::metadata(Path::new(&file_or_dir_str_path))?;

        if file_or_dir_str_metadata.is_dir() && file_or_dir_str == name_dir  {
            return Ok(());     
        }
    } 
    let str_file = format!("../{}", name_dir);
    fs::create_dir_all(Path::new(&str_file))?;

    Ok(())
}


fn file_transfer(name_file: &str, name_dir: &str) -> std::io::Result<()> {
    let _str = format!("../{}", &name_file);
    let name_file_pach = Path::new(&_str);

    let _str = format!("../{}/{}", name_dir, name_file);
    let dir_and_file_pach = Path::new(&_str);

    let (file, exc) = divide(Path::new(name_file)).unwrap();
    let strg = format!("../{}/{}(1).{}", &name_dir, file, exc);
    let path_dir_file_rex = Path::new(&strg);
    
    if name_file_pach.exists() && !dir_and_file_pach.exists(){
        fs::copy(&name_file_pach, &dir_and_file_pach)?;
        fs::remove_file(&name_file_pach)?;
        return Ok(());

    } else if name_file_pach.exists() && dir_and_file_pach.exists() && !path_dir_file_rex.exists() { 
        fs::copy(&name_file_pach, &path_dir_file_rex)?;
        fs::remove_file(&name_file_pach)?;
        return Ok(());

    } else if name_file_pach.exists() && dir_and_file_pach.exists() && path_dir_file_rex.exists() {
        let (file_name, exc) = divide(Path::new(name_file)).unwrap();
        let mut file_name_index: u16 = 1;

        let new_file = loop {
            file_name_index += 1;
            let strg = format!("../{}/{}({}).{}", &name_dir, &file_name, &file_name_index, &exc);
            let file_clon = Path::new(&strg);

            if !file_clon.exists() {
               break strg; 
            }
        };
        fs::copy(&name_file_pach, new_file)?;
        fs::remove_file(&name_file_pach)?;
        return Ok(()); 
    }
    Ok(())
}


fn conf_read_toml(path_file: &Path) -> Result<Value, Box<dyn error::Error>>{
    let conf = std::fs::read_to_string(path_file)?; 
    let toml: Value = toml::from_str(&conf)?;
    Ok(toml)
}


fn sord(list_dir: ReadDir, config_path: Value) -> std::io::Result<()> {
    
    for file in list_dir {
        let keys = config_path.as_table().unwrap().keys().cloned();
        let _str: String = format!("../{}", &file.unwrap().file_name().to_str().unwrap());
        let file_path = Path::new(&_str);

        for key in keys {
            let value = config_path.get(&key).unwrap().as_array().unwrap();
            for exc in value {

                if file_path.exists() {
                    if let Some(file_exc) = file_path.extension() {

                        if file_path.metadata().unwrap().is_file() {

                            if file_exc.to_str().unwrap() == exc.as_str().unwrap() {
                                let file = file_path.file_name().unwrap().to_str().unwrap();

                                create_dir(&key)?;
                                file_transfer(&file, &key)?;
                            }    
                        } 
                    }    
                }                
            }
        }
    } 
   Ok(())
}

fn main() {    
    let config = Path::new("config.toml"); 
    let config_path = conf_read_toml(config).unwrap_or_else(|_err| {
        eprintln!("no configuration file");
        std::process::exit(1);
    });
   
    let dir = Path::new("..");

    let (tx, rx) = channel();

    let config = Config::default()
    .with_poll_interval(Duration::from_secs(10))
    .with_compare_contents(false);

    let mut watcher: RecommendedWatcher = Watcher::new(tx, config).unwrap();

    watcher.watch(&dir, RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(Ok(o)) => {  
                match o.kind {
                    EventKind::Create(event::CreateKind::File) => {
                        sord(fs::read_dir(&dir).unwrap(), config_path.clone()).unwrap()
                    },
                    
                    EventKind::Modify(event::ModifyKind::Data(event::DataChange::Content)) => {
                        sord(fs::read_dir(&dir).unwrap(), config_path.clone()).unwrap()
                    },
                    
                    EventKind::Modify(event::ModifyKind::Metadata(event::MetadataKind::Any)) => {
                         sord(fs::read_dir(&dir).unwrap(), config_path.clone()).unwrap()
                    },
                    
                    _ => (),
                    } 
            }, 
            
            Ok(Err(_)) => (),
            
            Err(_) => (),
        }
    }
}
