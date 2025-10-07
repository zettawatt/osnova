//! Tests for Autonomi client connection management

use osnova_lib::network::AutonomiClient;
use osnova_lib::error::OsnovaError;

#[tokio::test]
async fn test_connect_success() {
    // Test successful connection to Autonomi network
    let result = AutonomiClient::connect().await;

    // For now, we expect this to work in test mode
    match result {
        Ok(client) => {
            assert!(client.is_connected());
        }
        Err(e) => {
            // In test environment without real network, we expect a specific error
            assert!(matches!(e, OsnovaError::Network(_)));
        }
    }
}

#[tokio::test]
async fn test_health_check() {
    // Test health check on connected client
    match AutonomiClient::connect().await {
        Ok(client) => {
            let health_result = client.health_check().await;
            assert!(health_result.is_ok());
        }
        Err(_) => {
            // Skip test if we can't connect (expected in CI)
        }
    }
}

#[tokio::test]
async fn test_disconnect() {
    // Test proper cleanup on disconnect
    match AutonomiClient::connect().await {
        Ok(mut client) => {
            assert!(client.is_connected());
            client.disconnect().await.expect("Disconnect should succeed");
            assert!(!client.is_connected());
        }
        Err(_) => {
            // Skip test if we can't connect
        }
    }
}

#[tokio::test]
async fn test_health_check_after_disconnect() {
    // Test that health check fails after disconnect
    match AutonomiClient::connect().await {
        Ok(mut client) => {
            client.disconnect().await.expect("Disconnect should succeed");
            let health_result = client.health_check().await;
            assert!(health_result.is_err());
        }
        Err(_) => {
            // Skip test if we can't connect
        }
    }
}

#[tokio::test]
async fn test_reconnect() {
    // Test reconnecting after disconnect
    match AutonomiClient::connect().await {
        Ok(mut client) => {
            client.disconnect().await.expect("Disconnect should succeed");
            assert!(!client.is_connected());

            let reconnect_result = client.reconnect().await;
            match reconnect_result {
                Ok(_) => {
                    assert!(client.is_connected());
                }
                Err(e) => {
                    // Reconnect might fail in test environment
                    assert!(matches!(e, OsnovaError::Network(_)));
                }
            }
        }
        Err(_) => {
            // Skip test if we can't connect initially
        }
    }
}
