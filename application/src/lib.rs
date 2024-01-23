use spin_sdk::http::{IntoResponse, Response, Json};
use spin_sdk::http_component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Operands {
    operands: Vec<f64>,
}

#[derive(Serialize)]
struct Resp {
    result: f64
}

/// A simple Spin HTTP component.
#[http_component]
fn math_power_off(req: http::Request<Json<Operands>>) -> anyhow::Result<impl IntoResponse> {
    
    let mut r: Resp = Resp {result: 0.00};
    let o = req.body();
    o.operands.iter().for_each(|x| { r.result = if r.result == 0.0 { *x } else { f64::powf(r.result, *x) } });
    
    let res_str = serde_json::to_string(&r)?;

    Ok(Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .header("Content-Length", res_str.len().to_string())
    .body(res_str)
    .build())
}
