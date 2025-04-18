use axum::{middleware::from_fn, Router};
pub mod auth;
pub mod docs;
pub mod users;

pub use auth::*;
pub use docs::*;
pub use users::*;

pub async fn routes() -> Router {
	let public_routes = Router::new().nest("/auth", auth_router());

	let protected_routes =
		Router::new().layer(from_fn(auth::auth_middleware::auth_middleware));

	Router::new().merge(public_routes).merge(protected_routes)
}
