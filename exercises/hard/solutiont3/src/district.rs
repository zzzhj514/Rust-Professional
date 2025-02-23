use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

type JsonInnerData = HashMap<String, HashMap<String, Vec<String>>>;

fn find(a: &str, parent: &mut HashMap<String, String>) -> String {
    let mut a = a.to_string();
    while let Some(parent_a) = parent.get(&a) {
        if parent_a == &a {
            break;
        }
        a = parent_a.clone();
    }
    a
}

fn merge(a: &str, b: &str, parent: &mut HashMap<String, String>) {
    let pa = find(a, parent);
    let pb = find(b, parent);
    if pa != pb {
        parent.insert(pa, pb);
    }
}

fn dsu(data: &HashMap<String, Vec<String>>) -> i32 {
    // 建立并查集并返回集合数量
    let mut st = std::collections::HashSet::new();
    for (key, value) in data.iter() {
        st.insert(key);
        for i in value.iter() {
            st.insert(i);
        }
    }
    let mut parent = HashMap::new();
    for i in st.iter() {
        // 初始指向自己
        parent.insert((*i).clone(), (*i).clone());
    }
    for (key, value) in data.iter() {
        for i in value.iter() {
            merge(key, i, &mut parent);
        }
    }
    let mut result = std::collections::HashSet::new();
    for i in st.iter() {
        result.insert(find(i, &mut parent));
    }
    result.len() as i32
}

fn analyze_districts() -> String {
    let file = File::open("district.json");
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return "".to_string(),
    };
    let mut json_str = String::new();
    if let Err(_) = file.read_to_string(&mut json_str) {
        return "".to_string();
    }

    let cleaned_str = json_str.replace(|c: char| c.is_whitespace() || c == '"', "");
    let cleaned_str = cleaned_str.trim_start_matches('{').trim_end_matches('}');

    let mut data: JsonInnerData = HashMap::new();

    let parts: Vec<&str> = cleaned_str.split("},").collect();
    for part in parts {
        let part_with_brace = format!("{}{}", part, "}");
        let sub_parts: Vec<&str> = part_with_brace.splitn(2, ':').collect();
        if sub_parts.len() == 2 {
            let mut inner_map: HashMap<String, Vec<String>> = HashMap::new();
            let value = sub_parts[1].trim_start_matches('{').trim_end_matches('}');
            let value_parts: Vec<&str> = value.split("],").collect();
            for value_part in value_parts {
                let value_part_with_bracket = if !value_part.ends_with(']') {
                    format!("{}{}", value_part, ']')
                } else {
                    value_part.to_string()
                };
                let sub_value_parts: Vec<&str> = value_part_with_bracket.splitn(2, ':').collect();
                if sub_value_parts.len() == 2 {
                    let key = sub_value_parts[0].to_string();
                    let array_str = sub_value_parts[1]
                        .trim_start_matches('[')
                        .trim_end_matches(']');
                    let array: Vec<String> =
                        array_str.split(',').map(|s| s.trim().to_string()).collect();

                    if inner_map.contains_key(&key) {
                        let existing_array = inner_map.get_mut(&key).unwrap();
                        existing_array.extend(array);
                        existing_array.sort();
                        existing_array.dedup();
                    } else {
                        inner_map.insert(key, array);
                    }
                }
            }

            data.insert(sub_parts[0].to_string(), inner_map);
        }
    }

    let mut data: Vec<_> = data.into_iter().collect();
    data.sort_by(|a, b| a.0.cmp(&b.0));

    let mut result = Vec::new();

    for (_key, value) in data.iter() {
        let siz: i32 = dsu(value);
        result.push(siz);
    }

    let mut res = String::new();
    for i in result.iter() {
        res.push_str(&i.to_string());
        res.push_str(",");
    }
    if !res.is_empty() {
        res.pop();
    }
    res
}

pub fn count_provinces() -> String {
    analyze_districts()
}