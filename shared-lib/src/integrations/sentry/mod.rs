use sentry::ClientInitGuard;

pub fn connect_sentry(dsn_url: &Option<String>) -> Option<ClientInitGuard> {
    match dsn_url {
        None => {
            tracing::info!("No sentry config detected, skipping...");
            None
        }
        Some(dsn_url) => {
            tracing::info!("Connecting to Sentry.io...");
            let guard = sentry::init((
                dsn_url.to_string(),
                sentry::ClientOptions {
                    release: sentry::release_name!(),
                    sample_rate: 1.0,
                    traces_sample_rate: 0.2,
                    ..Default::default()
                },
            ));

            sentry::configure_scope(|scope| scope.set_level(Some(sentry::Level::Warning)));
            Some(guard)
        }
    }
}
