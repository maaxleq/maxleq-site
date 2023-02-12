use axum::Router;
use dotenv::dotenv;
use maxleq_site::database::init_client;
use tower::BoxError;

static DEFAULT_HTTP_HOST: &str = "127.0.0.1";
static DEFAULT_HTTP_PORT: &str = "8000";

fn main() -> Result<(), BoxError> {
    config_env();

    let mut async_runtime_builder = tokio::runtime::Builder::new_multi_thread();
    async_runtime_builder.enable_all();

    if let Ok(thread_count) = dotenv::var("THREADS") {
        async_runtime_builder.worker_threads(thread_count.parse()?);
    }

    async_runtime_builder.build()?.block_on(launch())
}

async fn launch() -> Result<(), BoxError> {
    init_client().await?;

    let app = Router::new();
    // .route("/all", get(find_all))
    // .route("/:id", get(find_by_id))
    // .route("/city", get(find_by_city))
    // .route("/country", get(find_by_country))
    // .route("/name", get(find_by_name));

    let host =
        normalize_host(dotenv::var("HTTP_HOST").unwrap_or_else(|_| DEFAULT_HTTP_HOST.to_string()));
    let port = dotenv::var("HTTP_PORT").unwrap_or_else(|_| DEFAULT_HTTP_PORT.to_string());

    let address = format!("{}:{}", host, port);

    let server = axum::Server::bind(&address.parse()?).serve(app.into_make_service());

    println!("Server listening on {}", address);

    server.await?;

    Ok(())
}

fn config_env() {
    dotenv().ok();
}

fn normalize_host(host: String) -> String {
    match host.as_str() {
        "localhost" => String::from("127.0.0.1"),
        _ => host,
    }
}
