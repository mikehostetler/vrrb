use thiserror::Error;
use tracing_subscriber::{
    fmt::MakeWriter,
    util::{SubscriberInitExt, TryInitError},
};

#[derive(Debug, Error)]
pub enum TelemetryError {
    #[error("failed to initialize: {0}")]
    Init(#[from] TryInitError),

    #[error("{0}")]
    Other(String),

    #[error("unknown error occurred")]
    Unknown,
}

type Result<T> = std::result::Result<T, TelemetryError>;

// TODO: figure out the proper generic sig to export a telemetry builder instead
#[derive(Debug)]
pub struct TelemetrySubscriber {}

impl TelemetrySubscriber {
    pub fn init<W>(out: W) -> Result<()>
    where
        W: for<'s> MakeWriter<'s> + 'static + Sync + Send,
    {
        let sub = tracing_subscriber::fmt()
            .with_writer(out)
            .with_file(true)
            .with_line_number(true)
            .json()
            .with_current_span(false)
            .flatten_event(true)
            .with_span_list(false)
            .finish();

        sub.try_init()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use tracing_subscriber::fmt::TestWriter;

    use super::*;

    #[test]
    fn logs_to_stdout() {
        let tw = TestWriter::new();

        TelemetrySubscriber::init(tw).unwrap();

        tracing::info!("hello world 2");
    }
}
