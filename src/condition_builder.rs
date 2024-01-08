pub fn equal_builder(equal: &[&str]) -> (Vec<String>, Vec<String>) {
    if equal.len() % 2 != 0 {
        print!("{}\n", "equal length must be even");
        return (Vec::new(), Vec::new());
    }
    let mut conditions = Vec::new();
    let mut params = Vec::new();
    for i in (0..equal.len()).step_by(2) {
        let c = format!("{} = ?", equal[i]);
        conditions.push(c);
        params.push(equal[i+1].to_string());
    }
    (conditions, params)
}

pub fn object_contain_builder(object_contain: &[&str]) -> (Vec<String>, Vec<String>) {
    if object_contain.len() % 3 != 0 {
        print!("{}\n", "object_contain length must be multiple of 3");
        return (Vec::new(), Vec::new())
    }
    let mut conditions = Vec::new();
    let mut params = Vec::new();
    for i in (0..object_contain.len()).step_by(2) {
        let c = format!("json_contains({}, json_quote(?))", object_contain[i]);
        conditions.push(c);
        params.push(object_contain[i+1].to_string());
    }
    (conditions, params)
}
