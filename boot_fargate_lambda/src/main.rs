use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use std::env;
use aws_sdk_elasticloadbalancingv2 as elbv2;
use aws_sdk_ecs as ecs;

const HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head>
  <meta http-equiv="refresh" content="{delay}">
  <title>Booting Fargate...</title>
</head>
<body>
  <h1>Booting Fargate container, please wait...</h1>
</body>
</html>
"#;

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let target_group_arn = env::var("TARGET_GROUP_ARN")
        .expect("TARGET_GROUP_ARN env var must be set");
    let fargate_arn = env::var("FARGATE_ARN")
        .expect("FARGATE_ARN env var must be set");
    let delay = env::var("REFRESH_DELAY").unwrap_or_else(|_| "5".to_string());

    // Register Fargate task/container to the target group
    let config = aws_config::load_from_env().await;
    let elbv2_client = elbv2::Client::new(&config);

    // You may need to look up the Fargate ENI or IP to register
    // For demo, we assume FARGATE_ARN is a target id (e.g., IP or ENI)
    elbv2_client
        .register_targets()
        .target_group_arn(&target_group_arn)
        .targets(elbv2::types::TargetDescription::builder().id(&fargate_arn).build())
        .send()
        .await
        .expect("Failed to register target");

    let html = HTML_TEMPLATE.replace("{delay}", &delay);

    Ok(json!({
        "statusCode": 200,
        "headers": { "Content-Type": "text/html" },
        "body": html
    }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}
