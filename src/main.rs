use std::process::Command;

fn main() {
    let schemas = vec![
        r#"{
            "properties": {
                "name": { "type": "string" }
            }
        }"#,
        r#"{
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "integer", "minimum": 0 },
                "email": { "type": "string", "format": "email" }
            },
            "required": ["name", "age"]
        }"#,
        r#"{
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "id": { "type": "integer" },
                    "title": { "type": "string" },
                    "completed": { "type": "boolean" }
                },
                "required": ["id", "title", "completed"]
            },
            "minItems": 1,
            "maxItems": 10
        }"#,
    ];

    for schema in schemas {
        let res = Command::new("./guidance/guidance")
            .arg(schema)
            .output()
            .unwrap();

        println!("regex: {:?} \n", String::from_utf8_lossy(&res.stdout));
    }
}