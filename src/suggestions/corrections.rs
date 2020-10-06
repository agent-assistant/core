use std::fs::File;
use std::io::prelude::*;
use std::convert::TryInto;
use json;

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
    if filelist.len() > 0 && filelist.first().unwrap_or(&"".to_owned()) != "" {
        for file in filelist {
            let file = File::open(file);
            let mut fc = file.unwrap();
            content.insert_str(content.len(), "\n");
            fc.read_to_string(content).unwrap_or_default();
        }
    }
    wordlist.append(&mut content.split('\n').collect::<Vec<_>>());
    let mut nwl: Vec<String> = vec![];
    for word in wordlist {nwl.insert(nwl.len(), word.to_owned())};
    nwl.sort(); nwl.dedup();
    return nwl;
}

fn _corrections(input: &str, dict: Vec<String>, limit: i32) -> Vec<json::JsonValue> {
    //let mut result_set: HashMap<String, f64> = HashMap::new();
    let mut result_set: Vec<(String, f64, i32)> = vec![];
    for item in dict {
        let diff = strsim::sorensen_dice(&input.to_lowercase(), &item);
        let diff2 = strsim::levenshtein(&input.to_lowercase(), &item);
        if diff > 0.6 {result_set.insert(result_set.len(), (item, diff, diff2 as i32));}
    }
    //todo!("TODO: Sort results better");
    //let results = result_set.iter().cmp(|a: (&str, &f64), b: (&str, &f64)| a.1.partial_cmp(&b.1)).unwrap_or((&String::default(), &0.0));
    // let _results = result_set.iter();
    // let mut results: Vec<(&String, &f64)> = Vec::new();
    // let resx = _results.collect();
    result_set.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap()); //Sorensen-Dice
    result_set.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap()); //Levenshtein
    result_set.truncate(limit.try_into().unwrap());
    //let results = result_set.iter().b(|(a, b)| a.1.partial_cmp(&b.1).unwrap()).unwrap_or((&String::default(), &0.0));
    let mut objs: Vec<json::JsonValue> = vec![];
    for result in result_set {
        let res = result.0.as_str();
        objs.append(&mut vec![json::object!{
            "type": "correction",
            "text": res,
            "replace_with": res
        }]);
    }
    return objs
}

/// Return JSON formatted corrections with a dictionary.
#[allow(unused)]
pub fn corrections_dict(input: &str, dict: Vec<String>) -> Vec<json::JsonValue> {
    return _corrections(input, dict, 7)
}
/// Return JSON formatted corrections with a dictionary and custom limit.
#[allow(unused)]
pub fn corrections_dict_limited(input: &str, dict: Vec<String>, limit: i32) -> Vec<json::JsonValue> {
    return _corrections(input, dict, limit)
}