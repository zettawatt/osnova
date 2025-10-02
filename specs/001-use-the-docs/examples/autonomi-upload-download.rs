// Example: Autonomi Upload and Download Operations
// This example demonstrates how to use the osnova-autonomi component
// to upload and download data to/from the Autonomi network.

use serde_json::json;

/// Example 1: Upload and download a chunk (immutable data)
async fn example_chunk_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Chunk Operations Example ===\n");
    
    // Sample data to upload
    let data = b"Hello, Autonomi Network!";
    let data_base64 = base64::encode(data);
    
    // 1. Upload chunk (requires payment)
    println!("Uploading chunk...");
    let upload_response = openrpc_call("autonomi.chunk.upload", json!({
        "data": data_base64,
        "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
    })).await?;
    
    let chunk_address = upload_response["result"]["address"].as_str().unwrap();
    println!("Chunk uploaded successfully!");
    println!("Address: {}", chunk_address);
    println!("Cost: {} AUTONOMI + {} ETH", 
        upload_response["result"]["cost"]["autonomi"],
        upload_response["result"]["cost"]["eth"]
    );
    
    // 2. Download chunk (free operation)
    println!("\nDownloading chunk...");
    let download_response = openrpc_call("autonomi.chunk.download", json!({
        "address": chunk_address
    })).await?;
    
    let downloaded_data = base64::decode(
        download_response["result"]["data"].as_str().unwrap()
    )?;
    println!("Downloaded: {}", String::from_utf8(downloaded_data)?);
    
    Ok(())
}

/// Example 2: Create and update a pointer (mutable reference)
async fn example_pointer_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Pointer Operations Example ===\n");
    
    // Upload initial chunk
    let data_v1 = b"Version 1 of my data";
    let chunk_v1_response = openrpc_call("autonomi.chunk.upload", json!({
        "data": base64::encode(data_v1),
        "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
    })).await?;
    let chunk_v1_address = chunk_v1_response["result"]["address"].as_str().unwrap();
    
    // 1. Create pointer to first chunk (requires payment)
    println!("Creating pointer...");
    let create_response = openrpc_call("autonomi.pointer.create", json!({
        "target": chunk_v1_address,
        "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
    })).await?;
    
    let pointer_address = create_response["result"]["address"].as_str().unwrap();
    let secret_key = create_response["result"]["secretKey"].as_str().unwrap();
    println!("Pointer created!");
    println!("Address: {}", pointer_address);
    println!("Secret Key: {} (store this securely!)", secret_key);
    
    // 2. Get pointer target
    println!("\nGetting pointer target...");
    let get_response = openrpc_call("autonomi.pointer.get", json!({
        "address": pointer_address
    })).await?;
    println!("Current target: {}", get_response["result"]["target"]);
    
    // 3. Upload new version and update pointer (update is free!)
    let data_v2 = b"Version 2 of my data - updated!";
    let chunk_v2_response = openrpc_call("autonomi.chunk.upload", json!({
        "data": base64::encode(data_v2),
        "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
    })).await?;
    let chunk_v2_address = chunk_v2_response["result"]["address"].as_str().unwrap();
    
    println!("\nUpdating pointer to new version...");
    let update_response = openrpc_call("autonomi.pointer.update", json!({
        "address": pointer_address,
        "newTarget": chunk_v2_address,
        "secretKey": secret_key
    })).await?;
    println!("Pointer updated! Counter: {}", update_response["result"]["counter"]);
    
    Ok(())
}

/// Example 3: Create and update a scratchpad (mutable data storage)
async fn example_scratchpad_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Scratchpad Operations Example ===\n");
    
    // 1. Create scratchpad with initial data (requires payment)
    let initial_data = b"Initial scratchpad content";
    println!("Creating scratchpad...");
    let create_response = openrpc_call("autonomi.scratchpad.create", json!({
        "data": base64::encode(initial_data),
        "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
    })).await?;
    
    let scratchpad_address = create_response["result"]["address"].as_str().unwrap();
    let secret_key = create_response["result"]["secretKey"].as_str().unwrap();
    println!("Scratchpad created!");
    println!("Address: {}", scratchpad_address);
    println!("Secret Key: {} (store this securely!)", secret_key);
    
    // 2. Get scratchpad data
    println!("\nGetting scratchpad data...");
    let get_response = openrpc_call("autonomi.scratchpad.get", json!({
        "address": scratchpad_address
    })).await?;
    let data = base64::decode(get_response["result"]["data"].as_str().unwrap())?;
    println!("Current data: {}", String::from_utf8(data)?);
    
    // 3. Update scratchpad (update is free!)
    let updated_data = b"Updated scratchpad content - much longer now!";
    println!("\nUpdating scratchpad...");
    let update_response = openrpc_call("autonomi.scratchpad.update", json!({
        "address": scratchpad_address,
        "data": base64::encode(updated_data),
        "secretKey": secret_key
    })).await?;
    println!("Scratchpad updated! Counter: {}", update_response["result"]["counter"]);
    
    Ok(())
}

/// Example 4: Upload and download a public archive (file collection)
async fn example_archive_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Archive Operations Example ===\n");
    
    // 1. Upload public archive
    println!("Uploading public archive...");
    let upload_response = openrpc_call("autonomi.archive.uploadPublic", json!({
        "files": [
            {
                "path": "README.md",
                "data": base64::encode(b"# My Project\n\nWelcome!")
            },
            {
                "path": "config.json",
                "data": base64::encode(b"{\"version\": \"1.0\"}")
            }
        ],
        "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
    })).await?;
    
    let archive_address = upload_response["result"]["archiveAddress"].as_str().unwrap();
    println!("Archive uploaded!");
    println!("Address: {}", archive_address);
    println!("Files: {}", upload_response["result"]["fileCount"]);
    
    // 2. Download archive
    println!("\nDownloading archive...");
    let download_response = openrpc_call("autonomi.archive.download", json!({
        "archiveAddress": archive_address,
        "accessKey": null
    })).await?;
    
    println!("Downloaded {} files:", download_response["result"]["fileCount"]);
    for file in download_response["result"]["files"].as_array().unwrap() {
        println!("  - {}: {} bytes", file["path"], file["size"]);
    }
    
    Ok(())
}

/// Example 5: Upload and download a private archive (encrypted)
async fn example_private_archive() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Private Archive Example ===\n");
    
    // 1. Upload private archive
    println!("Uploading private archive...");
    let upload_response = openrpc_call("autonomi.archive.uploadPrivate", json!({
        "files": [
            {
                "path": "secrets.txt",
                "data": base64::encode(b"My secret data")
            }
        ],
        "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
    })).await?;
    
    let archive_address = upload_response["result"]["archiveAddress"].as_str().unwrap();
    let access_key = upload_response["result"]["accessKey"].as_str().unwrap();
    println!("Private archive uploaded!");
    println!("Address: {}", archive_address);
    println!("Access Key: {} (store this securely!)", access_key);
    
    // 2. Download private archive (requires access key)
    println!("\nDownloading private archive...");
    let download_response = openrpc_call("autonomi.archive.download", json!({
        "archiveAddress": archive_address,
        "accessKey": access_key
    })).await?;
    
    println!("Downloaded private archive successfully!");
    
    Ok(())
}

/// Example 6: Payment flow integration with wallet
async fn example_payment_flow() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Payment Flow Example ===\n");
    
    // This demonstrates how the autonomi component integrates with the wallet
    // for payment processing
    
    let data = b"Data to upload";
    let data_size_mb = data.len() as f64 / 1_048_576.0;
    
    // 1. Estimate cost (internal to autonomi component)
    println!("Estimating upload cost...");
    let estimated_cost = json!({
        "eth": "0.001",
        "autonomi": "10.0"
    });
    
    // 2. Request payment from wallet component
    println!("Requesting payment from wallet...");
    let payment_response = openrpc_call("wallet.requestPayment", json!({
        "componentId": "com.osnova.autonomi",
        "fromAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
        "toAddress": "0xAUTONOMI_PAYMENT_ADDRESS",
        "amount": estimated_cost["autonomi"],
        "network": "arbitrum",
        "token": "0xAUTONOMI_TOKEN_ADDRESS",
        "purpose": format!("Upload {:.2}MB to Autonomi network", data_size_mb),
        "metadata": {
            "uploadSize": format!("{:.2}MB", data_size_mb),
            "estimatedCost": format!("{} AUTONOMI + {} ETH", 
                estimated_cost["autonomi"], estimated_cost["eth"])
        }
    })).await?;
    
    if payment_response["result"]["approved"].as_bool().unwrap() {
        println!("Payment approved!");
        println!("Transaction hash: {}", payment_response["result"]["transactionHash"]);
        
        // 3. Proceed with upload using payment proof
        println!("\nProceeding with upload...");
        // Upload would happen here with the payment transaction
        
    } else {
        println!("Payment rejected by user");
    }
    
    Ok(())
}

/// Helper function to simulate OpenRPC calls
/// In real implementation, this would use the actual OpenRPC client
async fn openrpc_call(
    method: &str, 
    params: serde_json::Value
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // This is a placeholder - actual implementation would use OpenRPC client
    println!("  [OpenRPC] Calling: {}", method);
    println!("  [OpenRPC] Params: {}", params);
    
    // Simulate response
    Ok(json!({
        "result": {
            "success": true
        }
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Autonomi Upload/Download Examples\n");
    println!("==================================\n");
    
    // Run examples
    example_chunk_operations().await?;
    example_pointer_operations().await?;
    example_scratchpad_operations().await?;
    example_archive_operations().await?;
    example_private_archive().await?;
    example_payment_flow().await?;
    
    println!("\n==================================");
    println!("All examples completed successfully!");
    
    Ok(())
}

