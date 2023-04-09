use axum::Router;
use std::{future::Future, net::SocketAddr, sync::Arc};
use tower_http::trace;

use crate::{
    application::use_cases::{auth::AuthUseCaseTrait, profile::ProfileUseCaseTrait},
    config::Config,
};

mod error;
mod middleware;
mod routes;

#[derive(Clone)]
pub struct State {
    auth: Arc<dyn AuthUseCaseTrait>,
    profile: Arc<dyn ProfileUseCaseTrait>,
}

impl State {
    pub fn new(auth: Arc<dyn AuthUseCaseTrait>, profile: Arc<dyn ProfileUseCaseTrait>) -> Self {
        State { auth, profile }
    }
}

pub async fn run(config: Config, state: State, shutdown_signal: impl Future<Output = ()>) {
    let addr = format!("0.0.0.0:{0}", config.port)
        .parse::<SocketAddr>()
        .expect("Invalid port");

    let app = Router::new()
        .nest("/api/v1/auth", routes::auth::router())
        .nest("/api/v1/profile", routes::profile::router())
        .with_state(state)
        .layer(config.get_cors_layer())
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(tracing::Level::ERROR)),
        );

    tracing::info!("Listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal)
        .await
        .unwrap();
}

#[cfg(test)]
pub fn get_mock_state(
    auth: crate::application::use_cases::auth::MockAuthUseCase,
    profile: crate::application::use_cases::profile::MockProfileUseCase,
) -> State {
    State {
        auth: Arc::new(auth),
        profile: Arc::new(profile),
    }
}
