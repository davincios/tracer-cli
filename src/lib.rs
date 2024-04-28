// src/lib.rs

pub mod methods;
pub use methods::{
    log_message, pipeline_finish_run, pipeline_new_run, tool_process, AppConfig, Tool,
};

pub mod setup;

pub use setup::{setup_tracer, TracerProjectConfig};
pub mod cli;
