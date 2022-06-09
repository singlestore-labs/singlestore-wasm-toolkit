use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use tide::{Body, Request, Response};

use crate::handle;

#[derive(Clone)]
struct State {
    factory: handle::HandleFactory,
}

pub async fn listen_and_serve(port: u16, factory: handle::HandleFactory) -> Result<()> {
    tide::log::start();

    let state = State { factory };
    let mut app = tide::with_state(state);
    app.with(tide::log::LogMiddleware::new());

    app.at("/:name").post(handle_json);
    app.listen(format!("0.0.0.0:{}", port)).await?;

    Ok(())
}

type Row = Vec<Value>;

#[derive(Serialize, Deserialize)]
struct Payload {
    data: Vec<Row>,
}

async fn handle_json(mut req: Request<State>) -> tide::Result {
    let state = req.state();
    let mut handler = state.factory.make_handler()?;

    let payload: Payload = req.body_json().await?;
    let name = req.param("name")?;

    let mut result = Vec::new();
    for row in payload.data {
        if row.len() == 0 {
            return Err(anyhow!("Empty row").into());
        }

        let row_id = row[0].clone();
        let row_input = row[1..].to_vec();

        let output_raw = handler.handle_json(name.into(), serde_json::to_vec(&row_input)?)?;
        let output: Value = serde_json::from_slice(&output_raw)?;

        let encode_value = |v: &Value| match v {
            Value::Array(_) | Value::Object(_) => Value::String(serde_json::to_string(v).unwrap()),
            _ => v.clone(),
        };

        // if output is an array, then expand into multiple rows
        if output.is_array() {
            for row in output.as_array().unwrap() {
                result.push(vec![row_id.clone(), encode_value(row)]);
            }
        } else {
            result.push(vec![row_id.clone(), encode_value(&output)]);
        }
    }

    Ok(Response::from(Body::from_json(&Payload { data: result })?))
}
