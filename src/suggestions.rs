// TODO: something todo here...

use std::collections::HashMap;
use json;
use json::object;
use chrono;
use chrono::Timelike;
use strsim::sorensen_dice;

pub fn parse(input: &str) -> String {
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
        "what time is it" => {
            let time = chrono::Local::now();
            datasrc.insert(datasrc.len(), object!{
                "type": "time",
                "icon": "clock",
                "text": format!("It is {}:{} {}", time.hour12().1, time.minute(), if time.hour12().0 == false {"AM"} else {"PM"})
            })
        }
        _ => {}
    };
    let rtn = json::stringify(datasrc);
    return rtn;
}