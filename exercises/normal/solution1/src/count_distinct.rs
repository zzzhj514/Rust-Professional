pub fn new_count_distinct(input_str: &str) -> usize {
    let parts: Vec<&str> = input_str.split(',').collect();
    let mut new = Vec::new();
    for part in parts{
        if !new.contains(&part){
            new.push(part);
        }
    }
    new.len()
}
