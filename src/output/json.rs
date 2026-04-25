use serde::Serialize;

pub fn print_json<T: Serialize>(entries: &[T]) {
    match serde_json::to_string_pretty(entries) {
        Ok(json) => {
            println!("{}", json)
        }
        Err(e) => eprintln!("Failed to serilalize to JSON: {}", e),
    }
}
