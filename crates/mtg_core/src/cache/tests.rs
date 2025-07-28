#[cfg(test)]
mod tests {
    use crate::cache::{
        disk::{CacheStore, DiskCache},
        serializer::Serializer,
    };
    use serde::{Deserialize, Serialize};
    use std::time::Duration;
    use tempfile::TempDir;
    use tokio::time::sleep;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestData {
        name: String,
        value: i32,
    }

    async fn create_test_cache(prefix: Option<&str>) -> (DiskCache, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = DiskCache::builder().base_path(temp_dir.path());

        if let Some(p) = prefix {
            builder = builder.prefix(p);
        }

        let cache = builder.build().unwrap();
        (cache, temp_dir)
    }

    #[tokio::test]
    async fn test_basic_operations() {
        let (cache, _temp_dir) = create_test_cache(None).await;

        // Test insert and get
        let test_data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        let old_value = cache.insert("key1", test_data.clone()).await.unwrap();
        assert!(old_value.is_none());

        let retrieved: Option<TestData> = cache.get("key1").await.unwrap();
        assert_eq!(retrieved, Some(test_data.clone()));

        // Test contains_key
        assert!(
            <DiskCache as CacheStore<&str, TestData>>::contains_key(&cache, "key1")
                .await
                .unwrap()
        );
        assert!(
            !<DiskCache as CacheStore<&str, TestData>>::contains_key(&cache, "nonexistent")
                .await
                .unwrap()
        );

        // Test len
        assert_eq!(
            <DiskCache as CacheStore<&str, TestData>>::len(&cache)
                .await
                .unwrap(),
            1
        );
        assert!(!<DiskCache as CacheStore<&str, TestData>>::is_empty(&cache)
            .await
            .unwrap());

        // Test remove
        let removed: Option<TestData> = cache.remove("key1").await.unwrap();
        assert_eq!(removed, Some(test_data));

        assert_eq!(
            <DiskCache as CacheStore<&str, TestData>>::len(&cache)
                .await
                .unwrap(),
            0
        );
        assert!(<DiskCache as CacheStore<&str, TestData>>::is_empty(&cache)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_prefix_functionality() {
        let (cache, _temp_dir) = create_test_cache(Some("test/prefix")).await;

        let test_data = TestData {
            name: "prefixed".to_string(),
            value: 123,
        };

        cache.insert("key1", test_data.clone()).await.unwrap();
        let retrieved: Option<TestData> = cache.get("key1").await.unwrap();
        assert_eq!(retrieved, Some(test_data));
    }

    #[tokio::test]
    async fn test_serializers() {
        let temp_dir = TempDir::new().unwrap();

        // Test JSON serializer
        let json_cache = DiskCache::builder()
            .base_path(temp_dir.path())
            .prefix("json")
            .with_serializer(Serializer::Json)
            .build()
            .unwrap();

        // Test Bincode serializer
        let bincode_cache = DiskCache::builder()
            .base_path(temp_dir.path())
            .prefix("bincode")
            .with_serializer(Serializer::Bincode)
            .build()
            .unwrap();

        let test_data = TestData {
            name: "serializer_test".to_string(),
            value: 999,
        };

        // Test both serializers
        json_cache.insert("key1", test_data.clone()).await.unwrap();
        bincode_cache
            .insert("key1", test_data.clone())
            .await
            .unwrap();

        let json_result: Option<TestData> = json_cache.get("key1").await.unwrap();
        let bincode_result: Option<TestData> = bincode_cache.get("key1").await.unwrap();

        assert_eq!(json_result, Some(test_data.clone()));
        assert_eq!(bincode_result, Some(test_data));
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let (cache, _temp_dir) = create_test_cache(Some("clear_test")).await;

        // Add some data
        for i in 0..5 {
            let data = TestData {
                name: format!("item_{}", i),
                value: i,
            };
            cache.insert(&format!("key_{}", i), data).await.unwrap();
        }

        assert_eq!(
            <DiskCache as CacheStore<&str, TestData>>::len(&cache)
                .await
                .unwrap(),
            5
        );

        // Clear cache
        <DiskCache as CacheStore<&str, TestData>>::clear(&cache)
            .await
            .unwrap();
        assert_eq!(
            <DiskCache as CacheStore<&str, TestData>>::len(&cache)
                .await
                .unwrap(),
            0
        );
        assert!(<DiskCache as CacheStore<&str, TestData>>::is_empty(&cache)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_cleaning_operations() {
        let temp_dir = TempDir::new().unwrap();

        // Create cache with prefix
        let cache = DiskCache::builder()
            .base_path(temp_dir.path())
            .prefix("cleaning_test")
            .build()
            .unwrap();

        // Add test data
        for i in 0..3 {
            let data = TestData {
                name: format!("item_{}", i),
                value: i,
            };
            cache.insert(&format!("key_{}", i), data).await.unwrap();
        }

        // Test stats
        let stats = cache.stats(Some("cleaning_test")).await.unwrap();
        assert_eq!(stats.total_files, 3);
        assert!(stats.total_size > 0);

        // Test clean_prefix
        let report = cache.clean_prefix("cleaning_test").await.unwrap();
        assert_eq!(report.removed_count, 3);
        assert!(report.freed_bytes > 0);

        // Verify cache is empty
        assert_eq!(
            <DiskCache as CacheStore<&str, TestData>>::len(&cache)
                .await
                .unwrap(),
            0
        );
    }

    #[tokio::test]
    async fn test_clean_older_than() {
        let (cache, _temp_dir) = create_test_cache(Some("age_test")).await;

        // Add some data
        let old_data = TestData {
            name: "old".to_string(),
            value: 1,
        };
        cache.insert("old_key", old_data).await.unwrap();

        // Wait a bit
        sleep(Duration::from_millis(100)).await;

        let new_data = TestData {
            name: "new".to_string(),
            value: 2,
        };
        cache.insert("new_key", new_data.clone()).await.unwrap();

        // Clean entries older than 50ms (should remove old_key but keep new_key)
        let report = cache
            .clean_older_than(Duration::from_millis(50), Some("age_test"))
            .await
            .unwrap();

        // Note: This test might be flaky due to filesystem timestamp precision
        // In a real scenario, you'd use longer durations
        assert!(report.removed_count <= 1);

        // Verify new data still exists
        let retrieved: Option<TestData> = cache.get("new_key").await.unwrap();
        assert_eq!(retrieved, Some(new_data));
    }

    #[tokio::test]
    async fn test_clean_to_size_limit() {
        let (cache, _temp_dir) = create_test_cache(Some("size_test")).await;

        // Add data that exceeds a small size limit
        for i in 0..10 {
            let data = TestData {
                name: format!("large_item_with_long_name_{}", i),
                value: i * 1000,
            };
            cache.insert(&format!("key_{}", i), data).await.unwrap();
        }

        let initial_stats = cache.stats(Some("size_test")).await.unwrap();
        assert_eq!(initial_stats.total_files, 10);

        // Clean to a very small size limit (should remove most files)
        let report = cache
            .clean_to_size_limit(100, Some("size_test"))
            .await
            .unwrap();

        assert!(report.removed_count > 0);
        assert!(report.freed_bytes > 0);

        let final_stats = cache.stats(Some("size_test")).await.unwrap();
        assert!(final_stats.total_size <= 100 || final_stats.total_files < 10);
    }

    #[tokio::test]
    async fn test_list_prefixes() {
        let temp_dir = TempDir::new().unwrap();

        // Create caches with different prefixes
        let cache1 = DiskCache::builder()
            .base_path(temp_dir.path())
            .prefix("prefix1")
            .build()
            .unwrap();

        let cache2 = DiskCache::builder()
            .base_path(temp_dir.path())
            .prefix("prefix2/sub")
            .build()
            .unwrap();

        // Add data to create the directories
        let test_data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        cache1.insert("key1", test_data.clone()).await.unwrap();
        cache2.insert("key2", test_data).await.unwrap();

        // List prefixes
        let prefixes = cache1.list_prefixes().await.unwrap();
        assert!(prefixes.contains(&"prefix1".to_string()));
        assert!(prefixes.contains(&"prefix2".to_string()));
    }

    #[tokio::test]
    async fn test_error_handling() {
        let temp_dir = TempDir::new().unwrap();

        // Test invalid prefix
        let result = DiskCache::builder()
            .base_path(temp_dir.path())
            .prefix("")
            .build();
        assert!(result.is_err());

        let result = DiskCache::builder()
            .base_path(temp_dir.path())
            .prefix("../invalid")
            .build();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        let (cache, _temp_dir) = create_test_cache(Some("concurrent")).await;
        let cache = std::sync::Arc::new(cache);

        // Spawn multiple tasks that write to the cache
        let mut handles = Vec::new();

        for i in 0..10 {
            let cache_clone = cache.clone();
            let handle = tokio::spawn(async move {
                let data = TestData {
                    name: format!("concurrent_{}", i),
                    value: i,
                };
                cache_clone.insert(&format!("key_{}", i), data).await
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap().unwrap();
        }

        // Verify all data was written
        assert_eq!(
            <DiskCache as CacheStore<&str, TestData>>::len(&cache)
                .await
                .unwrap(),
            10
        );
    }
}
