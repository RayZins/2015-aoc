use serde_json::Value;

fn get_all_numbers(json: Value) -> Vec<i64> {
    fn recur_get(child: &Value, numbers: &mut Vec<i64>) {
        match child {
            Value::Number(_) => numbers.push(child.as_i64().unwrap()),
            Value::Array(_) => {
                for item in child.as_array().unwrap().iter() {
                    recur_get(item, numbers);
                }
            }
            Value::Object(_) => {
                // part2
                if child.as_object().unwrap().iter().any(|(_, v)| v == "red") {
                    return;
                }

                for (_, v) in child.as_object().unwrap() {
                    recur_get(v, numbers);
                }
            }
            _ => (),
        };
    }

    let mut nums = Vec::new();
    recur_get(&json, &mut nums);

    nums
}

fn main() -> Result<(), serde_json::Error> {
    let input = include_str!("../inputs/12.txt");
    let serialized: Value = serde_json::from_str(input)?;

    let sum = get_all_numbers(serialized).iter().sum::<i64>();
    println!("{}", sum);
    Ok(())
}
