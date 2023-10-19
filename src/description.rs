use serde_json::Value;

pub fn generate_description(description: &Value, indent: usize) -> String {
    let mut description_text = String::new();

    match description {
        Value::Object(map) => {
            for (key, value) in map {
                let formatted_key = key.replace("_", " ");
                match value {
                    Value::String(s) => {
                        let s = s.replace("\"", "");
                        description_text.push_str(&format!(
                            "{}- **{}**: {}\n",
                            "  ".repeat(indent),
                            formatted_key,
                            s
                        ));
                    }
                    Value::Number(n) => {
                        description_text.push_str(&format!(
                            "{}- **{}**: {}\n",
                            "  ".repeat(indent),
                            formatted_key,
                            n
                        ));
                    }
                    Value::Bool(b) => {
                        description_text.push_str(&format!(
                            "{}- **{}**: {}\n",
                            "  ".repeat(indent),
                            formatted_key,
                            b
                        ));
                    }
                    Value::Array(arr) => {
                        if arr.iter().all(|v| !matches!(v, Value::Object(_))) {
                            let arr_as_strings: Vec<String> = arr
                                .iter()
                                .map(|v| format!("{}", v).replace("\"", ""))
                                .collect();
                            description_text.push_str(&format!(
                                "{}- **{}**: [{}]\n",
                                "  ".repeat(indent),
                                formatted_key,
                                arr_as_strings.join(", ")
                            ));
                        } else {
                            description_text.push_str(&format!(
                                "{}- **{}**: \n",
                                "  ".repeat(indent),
                                formatted_key
                            ));
                            for item in arr {
                                description_text.push_str(&generate_description(item, indent + 1));
                            }
                        }
                    }
                    Value::Object(obj) if obj.is_empty() => {
                        description_text.push_str(&format!(
                            "{}- **{}**: {{}}\n",
                            "  ".repeat(indent),
                            formatted_key
                        ));
                    }
                    _ => {
                        description_text.push_str(&format!(
                            "{}- **{}**: \n",
                            "  ".repeat(indent),
                            formatted_key
                        ));
                        description_text.push_str(&generate_description(value, indent + 1));
                    }
                }
            }
        }
        _ => {}
    }

    description_text
}
