use std::collections::HashMap;

use anyhow::anyhow;
use opentelemetry::{global::{self},
                    trace::{Span, Status, Tracer},
                    KeyValue};
use regex::Regex;

use crate::{config,
            ipc::{error::IPCError, message::IPCMessage}};

/// Get environment variables matching given pattern.
#[tauri::command]
pub fn get_envs(
    message: IPCMessage<String>,
) -> Result<IPCMessage<HashMap<String, String>>, IPCError> {
    let cx = message.extract_cx();
    let _cx_guard = cx.attach();
    let tracer = global::tracer(config::APPLICATION_ID);
    let mut span = tracer.start("ipc/get_envs");

    // Default given expression if empty
    let expr = {
        let body = message.body().to_owned();

        if body.is_empty() {
            "^PUBLIC_".to_string()
        } else {
            body
        }
    };
    span.set_attribute(KeyValue::new("ipc.request.expression", expr.clone()));

    // Compile given regular expression
    let pattern = Regex::new(expr.as_str()).map_err(|err| {
        span.record_error(&err);
        span.set_status(Status::error(err.to_string()));

        anyhow!(err)
    })?;

    // Leave keys matching pattern only
    let vars = HashMap::<_, _>::from_iter(
        std::env::vars().filter(|(key, _)| pattern.is_match(key.as_str())),
    );

    // TODO: Internal filter for sensitive environment variables if necessary
    // ...

    // Save retrieved keys to span attribute (omit values for security reason, for now)
    let keys = vars.keys().map(|v| v.as_str()).collect::<Vec<_>>();
    span.set_attribute(KeyValue::new("ipc.response.keys", keys.join(", ")));

    Ok(IPCMessage::builder().body(vars).build())
}
