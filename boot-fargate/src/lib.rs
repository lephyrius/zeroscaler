use std::collections::HashMap;
use std::env;
use aws_config::BehaviorVersion;
use aws_sdk_elasticloadbalancingv2 as elbv2;

const HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head>
  <meta http-equiv="refresh" content="{delay}">
  <title>Booting Fargate...</title>
</head>
<body>
  <h1>Booting {name}, please wait...</h1>
</body>
</html>
"#;

pub struct ScaleResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub async fn scale_containers_with_params(target_group_arn: &str, fargate_arn: &str, delay: &str, html_template: &str, fargate_name: &str) -> ScaleResponse {
    // Register Fargate task/container to the target group
    let config = aws_config::load_defaults(BehaviorVersion::latest() ).await;
    let elbv2_client = elbv2::Client::new(&config);

    // You may need to look up the Fargate ENI or IP to register
    // For demo, we assume FARGATE_ARN is a target id (e.g., IP or ENI)
    elbv2_client
        .register_targets()
        .target_group_arn(target_group_arn)
        .targets(elbv2::types::TargetDescription::builder().id(fargate_arn).build())
        .send()
        .await
        .expect("Failed to register target");

    let html = html_template.replace("{delay}", &delay).replace("{name}", fargate_name);

    ScaleResponse {
        status: 200,
        headers: [("content-type".to_string(), "text/html".to_string())].into(),
        body: html.into(),
    }
}

pub async fn scale_containers() -> ScaleResponse {
    let target_group_arn = env::var("TARGET_GROUP_ARN")
        .expect("TARGET_GROUP_ARN env var must be set");
    let fargate_arn = env::var("FARGATE_ARN")
        .expect("FARGATE_ARN env var must be set");
    let delay = env::var("REFRESH_DELAY").unwrap_or_else(|_| "5".to_string());
    
    scale_containers_with_params(&target_group_arn, 
                                 &fargate_arn, 
                                 &delay, 
                                 &env::var("HTML_TEMPLATE").unwrap_or_else(|_| HTML_TEMPLATE.to_string()), 
                                 &env::var("FARGATE_NAME").unwrap_or_else(|_| "Fargate container".to_string()) 
    ).await
}





#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use lambda_http::{Request, RequestExt};

    #[tokio::test]
    async fn test_generic_http_handler() {
        let request = Request::default();

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello world, this is an AWS Lambda HTTP request"
        );
    }

    #[tokio::test]
    async fn test_http_handler_with_query_string() {
        let mut query_string_parameters: HashMap<String, String> = HashMap::new();
        query_string_parameters.insert("name".into(), "boot-fargate-lambda".into());

        let request = Request::default()
            .with_query_string_parameters(query_string_parameters);

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello boot-fargate-lambda, this is an AWS Lambda HTTP request"
        );
    }
}