use axum::{Json, Router, routing};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use shared::Data;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/client", serve_client())
        .route("/sum", routing::post(find_sum));

    let addr = "0.0.0.0:8080";

    let listener = TcpListener::bind(addr)
        .await
        .expect("Should create the listener");

    println!("Listening at: http://{addr}");

    axum::serve(listener, app)
        .await
        .expect("Should run the server");
}

fn serve_client() -> ServeDir {
    ServeDir::new("../web/dist")
}

async fn find_sum(Json(data): Json<Data>) -> Json<i32> {
    println!("Got data: {data:?}");
    Json(data.a + data.b)
}
