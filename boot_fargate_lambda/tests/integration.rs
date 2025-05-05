#[tokio::test]
async fn test_html_response() {
    use boot_fargate_lambda::handler;
    use serde_json::json;
    std::env::set_var("TARGET_GROUP_ARN", "dummy");
    std::env::set_var("FARGATE_ARN", "dummy");
    std::env::set_var("REFRESH_DELAY", "3");
    let event = lambda_runtime::LambdaEvent::new(json!({}), Default::default());
    let resp = handler(event).await.unwrap();
    let body = resp.get("body").unwrap().as_str().unwrap();
    assert!(body.contains("refresh"));
}