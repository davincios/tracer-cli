// src/lib.rs

pub mod methods;
pub use methods::{
    log_message, pipeline_finish_run, pipeline_new_run, send_event, tool_process, Tool,
};

pub mod setup;

pub use setup::{setup_tracer, ConfigPaths, TracerAppConfig};
pub mod cli;

pub mod metrics;
