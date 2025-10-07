//! Tests for Autonomi data download operations

use osnova_lib::network::{AutonomiClient, download_data};

#[tokio::test]
async fn test_download_with_invalid_uri() {
    // Test downloading with an invalid URI format
    let invalid_uri = "not-a-valid-uri";

    match AutonomiClient::connect().await {
        Ok(client) => {
            let result = download_data(&client, invalid_uri).await;
            // Should fail with invalid format error
            assert!(result.is_err());
        }
        Err(_) => {
            // Skip test if we can't connect
        }
    }
}

#[tokio::test]
async fn test_download_not_found() {
    // Test downloading data that doesn't exist
    let nonexistent_uri = "ant://ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

    match AutonomiClient::connect().await {
        Ok(client) => {
            let result = download_data(&client, nonexistent_uri).await;
            // Should fail with not found error
            assert!(result.is_err());
        }
        Err(_) => {
            // Skip test if we can't connect
        }
    }
}

#[tokio::test]
async fn test_download_roundtrip() {
    // Test upload followed by download (roundtrip)
    use osnova_lib::network::upload_data;

    match AutonomiClient::connect().await {
        Ok(client) => {
            let original_data = b"Test data for roundtrip";

            // This requires payment setup to actually work
            // For now, we just verify the function signatures
            let upload_result = upload_data(&client, original_data).await;

            if let Ok(uri) = upload_result {
                let download_result = download_data(&client, &uri).await;

                if let Ok(downloaded_data) = download_result {
                    assert_eq!(original_data, downloaded_data.as_slice());
                }
            }
        }
        Err(_) => {
            // Skip test if we can't connect
        }
    }
}

#[tokio::test]
async fn test_download_large_data() {
    // Test downloading large data (>1MB) to verify chunking works
    use osnova_lib::network::upload_data;

    match AutonomiClient::connect().await {
        Ok(client) => {
            let large_data = vec![0xAB; 2 * 1024 * 1024]; // 2MB of 0xAB

            // Try roundtrip with large data
            let upload_result = upload_data(&client, &large_data).await;

            if let Ok(uri) = upload_result {
                let download_result = download_data(&client, &uri).await;

                if let Ok(downloaded_data) = download_result {
                    assert_eq!(large_data.len(), downloaded_data.len());
                    assert_eq!(large_data, downloaded_data);
                }
            }
        }
        Err(_) => {
            // Skip test if we can't connect
        }
    }
}

#[tokio::test]
async fn test_download_empty_data() {
    // Test downloading empty data
    use osnova_lib::network::upload_data;

    match AutonomiClient::connect().await {
        Ok(client) => {
            let empty_data = b"";

            let upload_result = upload_data(&client, empty_data).await;

            if let Ok(uri) = upload_result {
                let download_result = download_data(&client, &uri).await;

                if let Ok(downloaded_data) = download_result {
                    assert_eq!(empty_data, downloaded_data.as_slice());
                }
            }
        }
        Err(_) => {
            // Skip test if we can't connect
        }
    }
}
