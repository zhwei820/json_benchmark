use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::time::{Duration, Instant};
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    // println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
fn main() {
    let now = Instant::now();
    let N = 100000;
    for i in 0..N {
        typed_example().unwrap();
    }

    let elapsed_time = now.elapsed();
    println!("total {} in ms", elapsed_time.as_millis());
    // println!("avg {} in us", elapsed_time.as_micros() / N);
}
