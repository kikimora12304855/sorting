use std::fs::{self, ReadDir};
use std::path::Path;
use toml::Value;


fn create_dir(name_dir: &str) -> std::io::Result<()> { 

    let dir_list = fs::read_dir("..")?; 

    for file_or_dir in dir_list {
        let file_or_dir_str: String = file_or_dir?.file_name().to_string_lossy().to_string();
        let file_or_dir_str_metadata = fs::metadata(format!("../{}", &file_or_dir_str))?;

        if file_or_dir_str_metadata.is_dir() && file_or_dir_str == name_dir  {
            return Ok(());     
        }
    } 
    let str_file = format!("../{}", name_dir);
    fs::create_dir_all(Path::new(&str_file))?;

    Ok(())
}


fn file_transfer(name_file: &str, name_dir: &str) -> std::io::Result<()> {
    let _str = format!("../{name_file}");
    let name_file_pach = Path::new(&_str);

    let _str = format!("../{}/{}", name_dir, name_file);
    let dir_and_file_pach = Path::new(&_str);
    
    if name_file_pach.exists() {
        fs::copy(&name_file_pach, dir_and_file_pach)?;
        fs::remove_file(name_file_pach)?;
        return Ok(());
    }

    Ok(())
}


fn conf_read_toml(path_file: &Path) -> std::io::Result<Value>{
    let conf = std::fs::read_to_string(path_file)?; 
    let toml: Value = toml::from_str(&conf).unwrap();
    Ok(toml)
}
    
fn sord(list_dir: ReadDir, config_path: Value) -> std::io::Result<()> {
   for file in list_dir {
        let keys = config_path.as_table().unwrap().keys().cloned();
        let _str = format!("../{}", &file?.file_name().to_str().unwrap());
        let file_path = Path::new(&_str);

        for key in keys {
            let value = config_path.get(&key).unwrap().as_array().unwrap();
            for exc in value {

                if file_path.exists() {
                    if let Some(file_exc) = file_path.extension() {

                        if file_path.metadata().expect("120").is_file() {

                                if file_exc.to_str().unwrap() == exc.as_str().expect("102") {
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
    let config = Path::new("./config.toml"); 
    let config_path = conf_read_toml(config).expect("\nERROR \nThere is no configuration file\n\n");
    let list_dir = fs::read_dir("..").unwrap();

    sord(list_dir, config_path).unwrap();

    
}
