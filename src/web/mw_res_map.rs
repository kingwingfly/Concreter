use axum::response::Response;
use tracing::info;

pub async fn mw_reponse_map<B>(res: Response<B>) -> Response<B> {
    info!("Response: {}", res.status());

    res
}
