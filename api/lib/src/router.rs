use crate::AppState;
use axum::{Router, routing::{post,get}, Json};
use crate::node;

pub fn create_api_router(state:AppState) -> Router{

Router::new()
    .route("/test", get(root))
    .nest("/ip", create_api_ip_router(state))
}


pub fn create_api_ip_router(state:AppState) -> Router {
    Router::new()
        .route("/", get(node::apip))
        .route("/get", get(node::get_node_id))
        .route("/del", post(node::del_node_id))
        .with_state(state)
}

async fn root() -> Json<String> {
    Json("bozo".to_string())
}


