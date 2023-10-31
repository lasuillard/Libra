use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum IPCError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl Serialize for IPCError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
