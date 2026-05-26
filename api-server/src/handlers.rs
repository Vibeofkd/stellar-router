use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tracing::{error, info};

use crate::{
    state::AppState,
    types::{
        FeeEstimate, RouteBreakdown, RouteDetails, SimulateRequest, SimulateResponse,
        TransactionStatus,
    },
};

/// Health check endpoint
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "ok"})))
}

/// Transaction simulation endpoint
///
/// Accepts a simulation request and returns estimated fees, expected outputs,
/// and route breakdown without executing the transaction.
pub async fn simulate(
    State(state): State<AppState>,
    Json(req): Json<SimulateRequest>,
) -> Result<Json<SimulateResponse>, (StatusCode, String)> {
    info!(
        "Simulating transaction: target={}, function={}",
        req.target, req.function
    );

    // Validate inputs
    if req.target.is_empty() || req.function.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "target and function are required".to_string(),
        ));
    }

    // Extract route details or use defaults
    let route_details = req.route_details.unwrap_or_else(|| RouteDetails {
        name: "default".to_string(),
        version: Some(1),
        expected_outputs: None,
    });

    // Simulate fee estimation
    // In a real implementation, this would call the router-execution contract
    let fee_estimate = FeeEstimate {
        base_fee: 100,
        resource_fee: 1000,
        total_fee: 1100,
        surge_multiplier: 100,
        high_load: false,
    };

    let expected_outputs = route_details
        .expected_outputs
        .unwrap_or_else(|| vec!["output_amount".to_string()]);

    let route_breakdown = RouteBreakdown {
        route_name: route_details.name.clone(),
        version: route_details.version.unwrap_or(1),
        target_contract: req.target.clone(),
        function: req.function.clone(),
    };

    let response = SimulateResponse {
        success: true,
        estimated_fees: fee_estimate,
        expected_outputs,
        route_breakdown,
        message: "Simulation successful".to_string(),
    };

    info!("Simulation completed successfully");
    Ok(Json(response))
}
