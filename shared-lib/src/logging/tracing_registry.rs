use sentry_tracing::EventFilter;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    config::{env::Environment, service_config::ServiceConfig},
    server::middlewares::request_logger::LOGGER_TARGET,
};

use super::tracing_filter;

pub fn setup_tracing_subscriber_registry(config: &impl ServiceConfig) {
    let trace_filter = tracing_filter::get_trace_filter();

    if config.get_environment() == Environment::Local {
        let fmt_layer = tracing_subscriber::fmt::layer().with_ansi(true);
        let sentry_filter = sentry_tracing::layer().event_filter(|md| {
            // only report warnings and errors
            match *md.level() {
                Level::TRACE | Level::DEBUG | Level::INFO => {
                    return EventFilter::Ignore;
                }
                _ => {}
            }

            // don't report from request logger
            if md.target() == LOGGER_TARGET {
                return EventFilter::Ignore;
            }
            EventFilter::Event
        });
        tracing_subscriber::registry()
            .with(trace_filter)
            .with(fmt_layer)
            .with(sentry_filter)
            .init();
        return;
    }

    let fmt_layer = tracing_subscriber::fmt::layer().json().with_span_list(true);
    let sentry_filter = sentry_tracing::layer().event_filter(|md| {
        // only report warnings and errors
        match *md.level() {
            Level::TRACE | Level::DEBUG | Level::INFO => {
                return EventFilter::Ignore;
            }
            _ => {}
        }

        // don't report from request logger
        if md.target() == LOGGER_TARGET {
            return EventFilter::Ignore;
        }
        EventFilter::Event
    });
    tracing_subscriber::registry()
        .with(trace_filter)
        .with(fmt_layer)
        .with(sentry_filter)
        .init();
}
