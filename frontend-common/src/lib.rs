use commands::{CommandError, CommandRequest, CommandResponse};
use js_sys::Error as JsError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Display;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

pub use frontend_macros::*;

#[derive(Debug, Clone)]
pub enum FetchError {
    JsError(JsError),
    CommandError(CommandError),
}

impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::JsError(err) => format!("JS error: {:?}", err),
            Self::CommandError(err) => format!("command error: {}", err),
        })
    }
}

impl std::error::Error for FetchError {}

impl From<JsError> for FetchError {
    fn from(value: JsError) -> Self {
        Self::JsError(value)
    }
}

impl From<CommandError> for FetchError {
    fn from(value: CommandError) -> Self {
        Self::CommandError(value)
    }
}

/// Invoke the backend command and return the response.
pub async fn command_fetch<Req, Res>(command: &str, req: &Req) -> Result<Res, FetchError>
where
    Req: Serialize,
    Res: DeserializeOwned,
{
    let req_serialized = serde_json::to_string(req).unwrap();
    let command_args = CommandRequest {
        name: command.to_owned(),
        req: req_serialized,
    };
    let command_args_serialized = serde_json::to_string(&command_args).unwrap();
    let command_args_value = serde_wasm_bindgen::to_value(&command_args_serialized).unwrap();

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&command_args_value);

    let request = Request::new_with_str_and_init("/api/command", &opts).unwrap();

    let headers = request.headers();
    headers.set("Content-Type", "application/json").unwrap();
    headers.set("Accept", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let res_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|err| err.dyn_into::<JsError>().unwrap())?;
    let res = res_value.dyn_into::<Response>().unwrap();
    let res_json = JsFuture::from(
        res.json()
            .map_err(|err| err.dyn_into::<JsError>().unwrap())?,
    )
    .await
    .map_err(|err| err.dyn_into::<JsError>().unwrap())?;
    let command_res = serde_wasm_bindgen::from_value::<CommandResponse>(res_json)
        .map_err(|err| JsValue::from(err).dyn_into::<JsError>().unwrap())?;
    let command_res_inner = serde_json::from_str::<Res>(&command_res.res?).unwrap();

    Ok(command_res_inner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}
