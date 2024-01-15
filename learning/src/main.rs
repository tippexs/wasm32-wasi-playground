use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Operands {
    operands: Vec<f64>,
}

#[derive(Serialize)]
struct Response {
    result: f64
}

fn main() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "operands": [5,2,3]
        }"#;
    let mut r: Response = Response {result: 0.00};
    let o: Operands = serde_json::from_str(data)?;

    o.operands.iter().for_each(|x| { r.result = if r.result == 0.0 { *x } else { f64::powf(r.result, *x) } });
    println!("{}", serde_json::to_string(&r)?);
    Ok(())
}