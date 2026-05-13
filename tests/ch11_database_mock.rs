//! ch11：`mockall` + `async-trait` 模拟异步数据库接口（书中 mock 思路的稳定写法）。
use async_trait::async_trait;
use mockall::predicate::eq;
use mockall::automock;

#[automock]
#[async_trait]
pub trait AsyncDb {
    async fn get_result(&self, key: i32) -> Result<i32, String>;
}

#[tokio::test]
async fn test_database_mock() {
    let mut mock = MockAsyncDb::new();
    mock.expect_get_result()
        .with(eq(4))
        .times(1)
        .returning(|_| Ok(11));

    let result = mock.get_result(4).await.unwrap();
    assert_eq!(result, 11);
}
