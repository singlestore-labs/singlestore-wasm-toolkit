use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::iter;
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

    app.at("/foo").post(handle_json);
    app.listen(format!("0.0.0.0:{}", port)).await?;

    Ok(())
}

type Row = Vec<serde_json::Value>;

#[derive(Serialize, Deserialize)]
struct Payload {
    data: Vec<Row>,
}

async fn handle_json(mut req: Request<State>) -> tide::Result {
    let state = req.state();
    let mut handler = state.factory.make_handler()?;

    let payload: Payload = req.body_json().await?;
    let path = req.url().path();

    let mut result = Vec::new();
    for row in payload.data {
        if row.len() == 0 {
            return Err(anyhow!("Empty row").into());
        }

        let row_id = row[0].clone();
        let row_input = row[1..].to_vec();

        let row_output_raw = handler.handle_json(path.into(), serde_json::to_vec(&row_input)?)?;
        let row_output: Row = serde_json::from_slice(&row_output_raw)?;

        let row_final: Row = iter::once(row_id).chain(row_output.into_iter()).collect();
        result.push(row_final);
    }

    Ok(Response::from(Body::from_json(&Payload { data: result })?))
}
