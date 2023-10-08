use std::collections::HashMap;

use opentelemetry::{global,
                    propagation::{Extractor, Injector},
                    Context};
use serde::{Deserialize, Serialize};

/// Standardized data structure for IPC calls.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IPCMessage<T>
where
    T: Default,
{
    headers: HashMap<String, String>,
    body: T,
}

impl<T> IPCMessage<T>
where
    T: Default,
{
    /// Return reference to message headers.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Return message body reference.
    pub fn body(&self) -> &T {
        &self.body
    }

    /// Extract context from message headers.
    pub fn extract_cx(&self) -> Context {
        global::get_text_map_propagator(|propagator| propagator.extract(&HeaderExtractor(self)))
    }

    /// Inject given context to message headers.
    pub fn inject_cx(&mut self, cx: &Context) {
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(cx, &mut HeaderInjector(self));
        });
    }
}

impl<T> IPCMessage<T>
where
    T: Default,
{
    /// Create new message with defaults.
    pub fn builder() -> IPCMessageBuilder<T> {
        IPCMessageBuilder::new()
    }
}

/// Builder for `IPCMessage`.
pub struct IPCMessageBuilder<T> {
    headers: Option<HashMap<String, String>>,
    body: Option<T>,
}

impl<T> IPCMessageBuilder<T>
where
    T: Default,
{
    /// Create new builder.
    pub fn new() -> Self {
        Self {
            headers: None,
            body: None,
        }
    }

    /// Set headers.
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Set body.
    pub fn body(mut self, body: T) -> Self {
        self.body = Some(body);
        self
    }

    /// Build message from builder. Defaults will be used for fields not provided.
    pub fn build(self) -> IPCMessage<T> {
        IPCMessage {
            headers: self.headers.unwrap_or_default(),
            body: self.body.unwrap_or_default(),
        }
    }
}

struct HeaderInjector<'a, T: Default>(pub &'a mut IPCMessage<T>);

impl<'a, T> Injector for HeaderInjector<'a, T>
where
    T: Default,
{
    fn set(&mut self, key: &str, value: String) {
        self.0.headers.insert(key.to_string(), value);
    }
}

struct HeaderExtractor<'a, T: Default>(pub &'a IPCMessage<T>);

impl<'a, T> Extractor for HeaderExtractor<'a, T>
where
    T: Default,
{
    fn get(&self, key: &str) -> Option<&str> {
        self.0.headers.get(key).map(|value| value.as_str())
    }

    fn keys(&self) -> Vec<&str> {
        self.0
            .headers
            .keys()
            .map(|value| value.as_str())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use std::collections::HashMap;

    use anyhow::Result;

    use super::IPCMessage;

    #[test]
    fn test_IPCMessage() -> Result<()> {
        let message = IPCMessage::<String>::builder().build();
        assert_eq!(
            message.headers().to_owned(),
            HashMap::<String, String>::new()
        );
        assert_eq!(message.body().to_owned(), "".to_string());

        Ok(())
    }
}
