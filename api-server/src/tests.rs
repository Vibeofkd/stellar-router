#[cfg(test)]
mod tests {
    use crate::types::{SimulateRequest, RouteDetails, TransactionStatus, TransactionStatusEvent};
    use chrono::Utc;

    #[test]
    fn test_simulate_request_serialization() {
        let req = SimulateRequest {
            target: "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4".to_string(),
            function: "transfer".to_string(),
            route_details: Some(RouteDetails {
                name: "swap".to_string(),
                version: Some(1),
                expected_outputs: Some(vec!["1000000".to_string()]),
            }),
        };

        let json = serde_json::to_string(&req).unwrap();
        let deserialized: SimulateRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.target, req.target);
        assert_eq!(deserialized.function, req.function);
    }

    #[test]
    fn test_transaction_status_event_serialization() {
        let event = TransactionStatusEvent {
            tx_id: "tx_12345".to_string(),
            status: TransactionStatus::Pending,
            timestamp: Utc::now().to_rfc3339(),
            message: Some("Transaction queued".to_string()),
        };

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: TransactionStatusEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.tx_id, event.tx_id);
        assert_eq!(deserialized.status, TransactionStatus::Pending);
    }

    #[test]
    fn test_transaction_status_enum() {
        assert_eq!(
            serde_json::to_string(&TransactionStatus::Pending).unwrap(),
            "\"PENDING\""
        );
        assert_eq!(
            serde_json::to_string(&TransactionStatus::Submitted).unwrap(),
            "\"SUBMITTED\""
        );
        assert_eq!(
            serde_json::to_string(&TransactionStatus::Confirmed).unwrap(),
            "\"CONFIRMED\""
        );
        assert_eq!(
            serde_json::to_string(&TransactionStatus::Failed).unwrap(),
            "\"FAILED\""
        );
    }

    #[test]
    fn test_fee_estimate_calculation() {
        use crate::types::FeeEstimate;

        let fee = FeeEstimate {
            base_fee: 100,
            resource_fee: 1000,
            total_fee: 1100,
            surge_multiplier: 100,
            high_load: false,
        };

        assert_eq!(fee.base_fee + fee.resource_fee, 1100);
        assert!(!fee.high_load);
    }

    #[test]
    fn test_fee_estimate_with_surge() {
        use crate::types::FeeEstimate;

        let fee = FeeEstimate {
            base_fee: 100,
            resource_fee: 1000,
            total_fee: 2200,
            surge_multiplier: 200,
            high_load: true,
        };

        assert_eq!(fee.total_fee, (fee.base_fee + fee.resource_fee) * 2);
        assert!(fee.high_load);
    }
}
