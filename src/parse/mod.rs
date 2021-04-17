pub mod results;
pub mod submit;
pub mod tasks;

// todo: deserializable trait

fn deserialize(data: &str) -> Vec<String> {
    let data = remove_outer_layer(data);
    let data = split_raw(data);
    let keys = get_keys(&data);
    let values = get_values(&data, keys.len());
    map_serialized(&keys, &values)
}

fn map_serialized(keys: &[String], values: &[String]) -> Vec<String> {
    let to_usize = |x: &String| x.to_string().parse::<usize>().unwrap();
    let not_zero = |x: &usize| *x != 0usize;
    let to_value = |x: usize| (*values[x - 1]).to_string();

    keys.iter()
        .map(to_usize)
        .filter(not_zero)
        .map(to_value)
        .map(|x| x.replace("\"", ""))
        .collect()
}

fn get_values(data: &[String], keys_len: usize) -> Vec<String> {
    data.iter()
        .skip(keys_len)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn remove_outer_layer(data: &str) -> String {
    data.chars().skip(5).take(data.len() - 13).collect()
}

fn split_raw(data: String) -> Vec<String> {
    data.split(',').map(|x| x.to_owned()).collect()
}

fn get_keys(data: &[String]) -> Vec<String> {
    let is_number = |x: &&String| (**x).chars().all(|c| c.is_ascii_digit());
    data.iter()
        .take_while(is_number)
        .map(|x| x.to_owned())
        .collect()
}
