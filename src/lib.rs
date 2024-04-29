// src/lib.rs

pub mod methods;
pub use methods::{log_message, pipeline_finish_run, pipeline_new_run, tool_process, Tool};

pub mod setup;

pub use setup::{setup_tracer, TracerConfig};
pub mod cli;
