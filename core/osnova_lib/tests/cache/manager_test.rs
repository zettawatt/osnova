//! Tests for component cache manager

use osnova_lib::cache::CacheManager;
use tempfile::TempDir;

#[tokio::test]
async fn test_cache_new() {
    // Test creating a new cache manager
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().to_path_buf();

    let cache = CacheManager::new(cache_dir.clone(), 1024 * 1024).unwrap(); // 1MB max
    assert_eq!(cache.max_size(), 1024 * 1024);
    assert_eq!(cache.current_size(), 0);
}

#[tokio::test]
async fn test_cache_store_and_get() {
    // Test storing and retrieving data
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().to_path_buf(), 10 * 1024 * 1024).unwrap();

    let key = "test-component-v1.0.0";
    let data = b"Test component data";

    // Store data
    cache.store(key, data).await.unwrap();

    // Retrieve data
    let retrieved = cache.get(key).await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), data);
}

#[tokio::test]
async fn test_cache_get_nonexistent() {
    // Test retrieving nonexistent data
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().to_path_buf(), 10 * 1024 * 1024).unwrap();

    let result = cache.get("nonexistent-key").await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_cache_size_tracking() {
    // Test that cache size is tracked correctly
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().to_path_buf(), 10 * 1024 * 1024).unwrap();

    let data1 = vec![0u8; 1000];
    let data2 = vec![1u8; 2000];

    cache.store("key1", &data1).await.unwrap();
    assert_eq!(cache.current_size(), 1000);

    cache.store("key2", &data2).await.unwrap();
    assert_eq!(cache.current_size(), 3000);
}

#[tokio::test]
async fn test_cache_eviction_when_full() {
    // Test LRU eviction when cache is full
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().to_path_buf(), 2500).unwrap(); // Small cache

    let data1 = vec![0u8; 1000];
    let data2 = vec![1u8; 1000];
    let data3 = vec![2u8; 1000];

    // Store three items that exceed cache size
    cache.store("key1", &data1).await.unwrap();
    cache.store("key2", &data2).await.unwrap();
    cache.store("key3", &data3).await.unwrap();

    // key1 should have been evicted
    let result1 = cache.get("key1").await.unwrap();
    assert!(result1.is_none());

    // key2 and key3 should still be present
    let result2 = cache.get("key2").await.unwrap();
    assert!(result2.is_some());

    let result3 = cache.get("key3").await.unwrap();
    assert!(result3.is_some());
}

#[tokio::test]
async fn test_cache_lru_update_on_access() {
    // Test that accessing an item updates its LRU position
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().to_path_buf(), 2500).unwrap();

    let data = vec![0u8; 1000];

    cache.store("key1", &data).await.unwrap();
    cache.store("key2", &data).await.unwrap();

    // Access key1 to make it recently used
    let _ = cache.get("key1").await.unwrap();

    // Store key3, which should evict key2 (least recently used)
    cache.store("key3", &data).await.unwrap();

    // key1 should still be present
    let result1 = cache.get("key1").await.unwrap();
    assert!(result1.is_some());

    // key2 should have been evicted
    let result2 = cache.get("key2").await.unwrap();
    assert!(result2.is_none());
}

#[tokio::test]
async fn test_cache_clear() {
    // Test clearing the entire cache
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().to_path_buf(), 10 * 1024 * 1024).unwrap();

    let data = b"Test data";
    cache.store("key1", data).await.unwrap();
    cache.store("key2", data).await.unwrap();

    assert_eq!(cache.current_size(), data.len() * 2);

    cache.clear().await.unwrap();

    assert_eq!(cache.current_size(), 0);
    assert!(cache.get("key1").await.unwrap().is_none());
    assert!(cache.get("key2").await.unwrap().is_none());
}

#[tokio::test]
async fn test_cache_remove() {
    // Test removing a specific item
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().to_path_buf(), 10 * 1024 * 1024).unwrap();

    let data = b"Test data";
    cache.store("key1", data).await.unwrap();
    cache.store("key2", data).await.unwrap();

    cache.remove("key1").await.unwrap();

    assert!(cache.get("key1").await.unwrap().is_none());
    assert!(cache.get("key2").await.unwrap().is_some());
    assert_eq!(cache.current_size(), data.len());
}

#[tokio::test]
async fn test_cache_large_file() {
    // Test caching large files (>1MB)
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().to_path_buf(), 10 * 1024 * 1024).unwrap();

    let large_data = vec![0xAB; 2 * 1024 * 1024]; // 2MB

    cache.store("large-component", &large_data).await.unwrap();

    let retrieved = cache.get("large-component").await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().len(), large_data.len());
}
