use std::time::Duration;

use cja::cron::{CancellationToken, CronRegistry, Worker};

use crate::{
    app_state::AppState,
    jobs::{create_checkin::BulkEnqueueCheckins, hello::Hello},
};

fn cron_registry() -> CronRegistry<AppState> {
    let mut registry = CronRegistry::new();

    registry.register_job(Hello, None, Duration::from_secs(60));
    registry.register_job(BulkEnqueueCheckins, None, Duration::from_secs(60));

    registry
}

pub(crate) async fn run_cron(
    app_state: AppState,
    shutdown_token: CancellationToken,
) -> miette::Result<()> {
    Worker::new(app_state, cron_registry())
        .run(shutdown_token)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    Ok(())
}
