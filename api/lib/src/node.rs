use http::StatusCode;
use std::net::SocketAddr;
use axum::{Json,extract::{ConnectInfo,State},response::IntoResponse};

use crate::AppState;

use shared::models::{Node, DeleteNode};

pub async fn get_node_id(State(state):State<AppState>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Result<Json<Node>, impl IntoResponse> {
    let node_from_db_result =match sqlx::query_as::<_,Node>
        ("select * from nodes where ip = ?")
        .bind(addr.ip().to_string())
        .fetch_one(&state.pool).await {
            Ok(res) => Ok(res),
            Err(_err) => create_node_entry(State(&state),addr.ip().to_string()).await
        };

    match node_from_db_result {
        Ok(node) => Ok(Json(node)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string().into_response()))
    }

}


//TODO add Json requirement to all api points for consistency
async fn create_node_entry(State(state):State<&AppState>,ip:String) -> Result<Node,sqlx::Error> {

    // finds the lowest free id in the db.
    match sqlx::query_as::<_,Node>("insert into nodes(id,ip) values ((SELECT (Min(id) + 1) AS NewIDToInsert 
   FROM nodes T2A
   WHERE NOT EXISTS (SELECT id FROM nodes T2B WHERE T2A.id + 1 = T2B.id)),?) returning id, ip,created_at")
        .bind(ip)
        .fetch_one(&state.pool).await {
            Ok(res) => Ok(res),
            Err(err) => Err(err)
        }

}


// requires json request with String ip of the device
pub async fn del_node_id(State(state):State<AppState>,ConnectInfo(addr): ConnectInfo<SocketAddr>, Json(del_req): Json<DeleteNode>) -> Result<Json<Node>, impl IntoResponse> {
    if addr.ip().to_string() == del_req.ip {
        return Err((StatusCode::FORBIDDEN, "You cannot request node other then yourself to be removed from registry".to_string().into_response()));
    }
    match sqlx::query_as::<_,Node>(
"delete from nodes where ip = ? returning id,ip,created_at")
        .bind(del_req.ip)
        .fetch_one(&state.pool).await
    {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string().into_response()))
    }
}


pub async fn apip(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("Hello {}", addr.ip().to_string())
}
