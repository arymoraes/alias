use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

const DATABASE_NAME: &str = "db.json";
const DIR_NAME: &str = "danlimas";

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    pub path: PathBuf,
    pub aliases: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        let path = dirs::config_dir();

        match path {
            Some(mut p) => {
                p.push(DIR_NAME);

                if !database_exists(&p) {
                    create_database_file(&mut p);
                } else {
                    p.push(DATABASE_NAME);
                }

                let aliases = get_database_contents(&p);

                let store = Store { path: p, aliases };

                println!("{:?}", store);

                store
            }
            None => {
                panic!("Could not get OS path")
            }
        }
    }

    pub fn get_command(&self, alias: String) -> &String {
        return self.aliases.get(&alias).unwrap();
    }

    pub fn save(&self) -> () {
        let content = serde_json::to_string(&self.aliases).unwrap();

        println!("{:?}", content);

        let mut file = OpenOptions::new().write(true).open(&self.path).unwrap();
        file.write_all(content.as_bytes())
            .expect("Could not save aliases");
        drop(file)
    }
}

fn create_database_file(path: &mut PathBuf) -> () {
    fs::create_dir_all(&path).expect("Could not create dir");
    path.push(DATABASE_NAME);
    File::create(&path).expect("Error creating database file");

    let mut file = OpenOptions::new().write(true).open(path).unwrap();
    file.write_all(b"{}").expect("Could not write to file");
    drop(file);
}

fn database_exists(path: &PathBuf) -> bool {
    let mut file_path = path.clone();
    file_path.push(DATABASE_NAME);

    Path::new(&file_path).exists()
}

fn get_database_contents(path: &PathBuf) -> HashMap<String, String> {
    let file_contents = fs::read_to_string(&path).expect("Unable to read file");
    serde_json::from_str(&file_contents).unwrap()
}
