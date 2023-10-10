use leptos::{leptos_config::Env, leptos_config::LeptosOptions};
use tracing_subscriber::{
    filter::{self},
    fmt::layer,
    prelude::*,
    registry,
};

pub fn init(config: &LeptosOptions) -> Result<(), crate::Error> {
    let log_level = if config.env == Env::PROD {
        filter::LevelFilter::INFO
    } else {
        filter::LevelFilter::DEBUG
    };

    let env_filter = filter::EnvFilter::new("")
        .add_directive(log_level.into())
        .add_directive("hyper=warn".parse().unwrap())
        .add_directive("reqwest=warn".parse().unwrap());

    let fmt_layer = layer().with_filter(env_filter);
    registry().with(fmt_layer).init();

    Ok(())
}
