// TODO: something todo here...

use std::collections::HashMap;
use json;
use strsim::sorensen_dice;

// ===== The following are core modules. ===== //
#[path="suggestions/time.rs"] mod time_mod;
#[path="suggestions/corrections.rs"] mod corrections;

pub fn parse(input: &str, dict: Vec<String>) -> String {
    //! Returns a JSON str of suggestions
    let mut datasrc: Vec<json::JsonValue> = Vec::new();
    let query_set = vec!["what time is it"];
    let mut result_set: HashMap<&str, f64> = HashMap::new();
    for item in query_set {
        let diff = sorensen_dice(&input.to_lowercase(), item);
        if diff > 0.5 {result_set.insert(item, diff);}
    }
    let results = result_set.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap_or((&"", &0.0));
    match *results.0 {
        "what time is it" => {datasrc.insert(datasrc.len(), time_mod::time());}
        _ => {}
    };
    datasrc.append(&mut corrections::corrections_dict(input.split(' ').last().unwrap_or_default(), dict));
    let rtn = json::stringify(datasrc);
    return rtn;
}