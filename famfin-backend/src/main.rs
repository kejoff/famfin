use axum::{
    middleware,
    routing::{get, post},
    Router,
    Json,
    http::{StatusCode, header},
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use tower_http::cors::{CorsLayer, AllowOrigin};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::limit::RequestBodyLimitLayer;
use tracing::info;
use serde_json::json;

use axum::extract::State;

use famfin_backend::{
    api::{
        auth::{create_household, get_me, login, logout},
        transactions::{create_transaction, get_transaction, list_transactions, update_transaction},
        categories::{create_category, get_category, list_categories},
        goals::{create_goal, delete_goal, get_goal, list_goals, update_goal},
        dashboard::{get_monthly_dashboard, get_spending_trends, get_goals_progress},
        ml::predict_category,
        import::{import_transactions, import_file},
        AppState,
    },
    auth::middleware::auth_middleware,
    db,
    ml::onnx::OnnxModel,
};

#[tokio::main(worker_threads = 2)]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "famfin.db".to_string());
    info!("Initializing database at: {}", db_path);
    let conn = db::init_db(&db_path).expect("Failed to initialize database");
    info!("Database initialized and ready");

    // Load ML model (before server starts accepting requests)
    let model_path = std::env::var("MODEL_PATH").unwrap_or_else(|_| "models/model.onnx".to_string());
    let ml_model = match OnnxModel::load(&model_path) {
        Ok(model) => Arc::new(model),
        Err(e) => {
            info!("Failed to load ML model: {}. Continuing with degraded service.", e);
            Arc::new(OnnxModel::placeholder())
        }
    };

    let state = AppState {
        db: Arc::new(Mutex::new(conn)),
        ml_model,
    };

    // Protected routes — auth_middleware injects AuthSession into extensions
    let protected = Router::new()
        .route("/api/me", get(get_me))
        .route("/api/households/{household_id}/logout", post(logout))
        .route("/api/households/{household_id}/transactions", post(create_transaction).get(list_transactions))
        .route("/api/households/{household_id}/transactions/{transaction_id}", get(get_transaction).put(update_transaction))
        .route("/api/households/{household_id}/categories", post(create_category).get(list_categories))
        .route("/api/households/{household_id}/categories/{category_id}", get(get_category))
        .route("/api/households/{household_id}/goals", post(create_goal).get(list_goals))
        .route("/api/households/{household_id}/goals/{goal_id}", get(get_goal).put(update_goal).delete(delete_goal))
        .route("/api/households/{household_id}/dashboard", get(get_monthly_dashboard))
        .route("/api/households/{household_id}/dashboard/trends", get(get_spending_trends))
        .route("/api/households/{household_id}/dashboard/goals", get(get_goals_progress))
        .route("/api/households/{household_id}/predict-category", post(predict_category))
        .route("/api/households/{household_id}/import", post(import_transactions))
        .route("/api/households/{household_id}/import-file", post(import_file))
        .route_layer(middleware::from_fn({
            let s = state.clone();
            move |req, next| {
                let s = s.clone();
                async move { auth_middleware(State(s), req, next).await }
            }
        }));

    // Static files directory (environment variable or default)
    let static_dir = std::env::var("STATIC_DIR")
        .unwrap_or_else(|_| "../famfin-frontend/dist".to_string());
    // CORS: allow localhost for dev, restrict origin in production
    let cors_origin = std::env::var("CORS_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());

    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(move |origin, _| {
            let origin_str = origin.to_str().unwrap_or("");
            origin_str == cors_origin || origin_str == "http://localhost:3000"
        }))
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);

    let app = Router::new()
        // Public routes
        .route("/health", get(health_check))
        .route("/api/households", post(create_household))
        .route("/api/households/{household_id}/login", post(login))
        .merge(protected)
        .with_state(state)
        // Serve static files — SPA fallback to index.html for client-side routing
        .fallback_service(
            ServeDir::new(&static_dir)
                .not_found_service(ServeFile::new(
                    PathBuf::from(&static_dir).join("index.html")
                )),
        )
        .layer(cors_layer)
        .layer(RequestBodyLimitLayer::new(50 * 1024 * 1024)); // 50MB max

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn health_check(State(state): State<AppState>) -> (StatusCode, Json<serde_json::Value>) {
    // Check database connectivity with 5-second timeout
    let db_healthy = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        async {
            let db = state.db.lock().await;
            db.query_row("SELECT 1", [], |_| Ok(())).is_ok()
        }
    )
    .await
    .unwrap_or(false);

    if !db_healthy {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({ "status": "unhealthy", "reason": "database unavailable" })));
    }

    (StatusCode::OK, Json(json!({ "status": "healthy" })))
}
