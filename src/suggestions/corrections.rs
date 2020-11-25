use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::convert::TryInto;
use json;
use regex::Regex;

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

#[allow(dead_code)]
pub fn load_dictionary_aospfile(path: &str) -> Vec<json::JsonValue> {
    let mut output: Vec<json::JsonValue> = vec![];
    //=== Load File ===//
    let _file = File::open(path);
    //#[allow(unused_mut)]
    //let mut file = &mut String::default();
    //_file.unwrap().read_to_string(file).unwrap_or_default();
    let reader = BufReader::new(_file.unwrap());
    //===== Match =====//
    let expression = Regex::new(r#"(?m)^ word=([a-zA-Z'-]*),f=(\d*).*$"#).unwrap();
    for line in reader.lines() {
        let ln = line.unwrap();
        for cap in expression.captures_iter(&ln) {
            output.append(&mut vec![json::object!{
                "word": &cap[1],
                "freq": &cap[2]
            }]);
        }
    }
    //File::create("/tmp/corrections.json").unwrap().write(json::stringify_pretty(output.clone(), 4).as_bytes()).unwrap();
    //println!("{}", output.len());
    return output;
}

fn _corrections(input: &str, dict: Vec<json::JsonValue>, limit: i32) -> Vec<json::JsonValue> {
    //let mut result_set: HashMap<String, f64> = HashMap::new();
    let mut result_set: Vec<(String, i32, f64, i32)> = vec![];
    for item in dict {
        let word = &item["word"].as_str().unwrap();
        let freq = &item["freq"].as_u32().unwrap_or(0);
        let diff = strsim::sorensen_dice(&input.to_lowercase(), word);
        let diff2 = strsim::levenshtein(&input.to_lowercase(), word);
        if diff > 0.5 {result_set.insert(result_set.len(), (word.to_string(), *freq as i32, diff, diff2 as i32));}
        //print!("{}", word);
    }
    result_set.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap()); //Sorensen-Dice
    result_set.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap()); //Levenshtein
    result_set.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap()); //Frequency
    result_set.truncate(limit.try_into().unwrap());
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
    let mut newdict = vec![];
    for entry in dict {
        newdict.append(&mut vec![json::object!{
            "word": entry,
            "freq": 1
        }]);
    }
    return _corrections(input, newdict, 7)
}
/// Return JSON formatted corrections with a dictionary.
#[allow(unused)]
pub fn corrections_dict_aosp(input: &str, dict: Vec<json::JsonValue>) -> Vec<json::JsonValue> {
    return _corrections(input, dict, 7)
}
/// Return JSON formatted corrections with a dictionary and custom limit.
#[allow(unused)]
pub fn corrections_dict_aosp_limited(input: &str, dict: Vec<json::JsonValue>, limit: i32) -> Vec<json::JsonValue> {
    return _corrections(input, dict, limit)
}
/// Return JSON formatted corrections with a dictionary and custom limit.
#[allow(unused)]
pub fn corrections_dict_limited(input: &str, dict: Vec<String>, limit: i32) -> Vec<json::JsonValue> {
    let mut newdict = vec![];
    for entry in dict {
        newdict.append(&mut vec![json::object!{
            "word": entry,
            "freq": 1
        }]);
    }
    return _corrections(input, newdict, limit)
}