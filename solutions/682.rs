pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut records = vec![];
    for o in operations.iter() {
        match o.parse::<i32>() {
            Ok(parsed) => records.push(parsed),
            Err(_) => match o.as_ref() {
                "C" => {
                    records.pop();
                }
                "D" => {
                    records.push(records[records.len() - 1] * 2);
                }
                "+" => {
                    records.push(records[records.len() - 1] + records[records.len() - 2]);
                }
                _ => {
                    continue;
                }
            },
        }
    }
    return records.iter().sum();
}

fn main() {
    println!(
        "{}",
        cal_points(vec![
            String::from("5"),
            String::from("2"),
            String::from("C"),
            String::from("D"),
            String::from("+")
        ])
    )
}
