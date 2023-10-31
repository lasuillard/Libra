use opentelemetry::{global,
                    trace::{Span, Tracer},
                    KeyValue};

use crate::{config,
            ipc::{error::IPCError, message::IPCMessage},
            service};

/// Greet user.
#[tauri::command]
pub fn greet(message: IPCMessage<String>) -> Result<IPCMessage<String>, IPCError> {
    let cx = message.extract_cx();
    let _cx_guard = cx.attach();
    let tracer = global::tracer(config::APPLICATION_ID);
    let mut span = tracer.start("ipc/greet");

    let greeting = service::greet(message.body());
    span.set_attribute(KeyValue::new("ipc.response.greeting", greeting.clone()));

    Ok(IPCMessage::builder().body(greeting).build())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{greet, IPCMessage};

    #[test]
    fn test_greet() -> Result<()> {
        let message = IPCMessage::builder().body("Libra".to_string()).build();
        let greeting = greet(message);

        assert_eq!(greeting.unwrap().body(), "Hello, Libra! (good)");

        Ok(())
    }
}
