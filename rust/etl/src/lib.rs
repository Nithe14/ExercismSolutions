use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(i, v)| v.iter().map(|value| (value.to_ascii_lowercase(), *i)))
        .collect()
}
