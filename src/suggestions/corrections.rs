use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn load_dictionary(dir: bool, path: &str) -> Vec<String> {
    let mut filelist: Vec<String> = vec![];
    if dir {
        // ==== Extract the directory ====
        // 1. Get the list of entries.
        let entries = std::fs::read_dir(path).unwrap();
        for entry in entries {
            let x = entry.unwrap();
            let y = x.path();
            let z = String::from(y.to_str().unwrap());
            // 2. Put the current entry into the list.
            filelist.insert(filelist.len(), z);
        }
    } //dir
    else {filelist.insert(filelist.len(), path.to_owned())}
    let mut wordlist: Vec<&str> = vec![];
    let content: &mut String = &mut String::default();
    for file in filelist {
        let file = File::open(file);
        let mut fc = file.unwrap();
        content.insert_str(content.len(), "\n");
        fc.read_to_string(content).unwrap_or_default();
    }
    wordlist.append(&mut content.split('\n').collect::<Vec<_>>());
    let mut nwl: Vec<String> = vec![];
    for word in wordlist {nwl.insert(nwl.len(), word.to_owned())};
    nwl.sort(); nwl.dedup();
    return nwl;
}