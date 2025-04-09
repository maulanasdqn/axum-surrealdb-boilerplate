use asb_apps::apps;
use asb_libs::axum_init;

#[tokio::main]
async fn main() {
	axum_init(|db, redis| async { apps(db, redis).await }).await;
}
