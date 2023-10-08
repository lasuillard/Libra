pub const ENV_VAR_PREFIX: &str = "LIBRA";

/// Helper macro for building prefixed environment variable key for Clap.
macro_rules! arg_env {
    ($name:literal) => {
        const_format::formatcp!(
            "{prefix}_{key}",
            prefix = $crate::cli::common::ENV_VAR_PREFIX,
            key = $name
        )
    };
}

// Export macro for import
pub(crate) use arg_env;
