mod handler;
mod route;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _worker_guard = axum_kit::bootstrap::Application::default("config.toml")?
        .with_router(route::api::init)
        .run()
        .await?;
    Ok(())
}
