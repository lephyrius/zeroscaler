use crate::elbv2::operation::register_targets::RegisterTargetsOutput;
use std::collections::HashMap;
use std::env;
use aws_config::BehaviorVersion;
use aws_sdk_elasticloadbalancingv2 as elbv2;
#[cfg(test)]
use mockall::automock;

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

/// Represents the response from scaling operations
/// 
/// # Fields
/// 
/// * `status` - HTTP status code
/// * `headers` - HTTP headers as key-value pairs
/// * `body` - Response body content
#[derive(Debug, PartialEq)]
pub struct ScaleResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

/// Implementation of the ELBv2 operations
#[allow(dead_code)]
pub struct Elbv2Impl {
    inner: elbv2::Client,
}

#[cfg_attr(test, automock)]
impl Elbv2Impl {
    #[allow(dead_code)]
    pub fn new(inner: elbv2::Client) -> Self {
        Self { inner }
    }

    #[allow(dead_code)]
    pub async fn register_target(
        &self,
        target_group_arn: &str,
        target_id: &str,
    ) -> Result<RegisterTargetsOutput, elbv2::Error> {
        self.inner
            .register_targets()
            .target_group_arn(target_group_arn)
            .targets(elbv2::types::TargetDescription::builder().id(target_id).build())
            .send()
            .await
            .map_err(|e| elbv2::Error::from(e))
    }
}

// Select the implementation based on whether we're testing
#[cfg(test)]
pub use MockElbv2Impl as Elbv2;
#[cfg(not(test))]
pub use Elbv2Impl as Elbv2;

/// Registers a Fargate container with an Elastic Load Balancer target group and returns a response
/// 
/// # Arguments
/// 
/// * `target_group_arn` - The ARN of the target group to register with
/// * `fargate_arn` - The ARN or identifier of the Fargate container to register
/// * `delay` - The refresh delay in seconds for the HTML page
/// * `html_template` - The HTML template to use for the response
/// * `fargate_name` - The name of the Fargate container to display
/// * `elbv2_client` - The ELBv2 client to use for the operation
/// 
/// # Returns
/// 
/// A `ScaleResponse` containing the HTTP response details
///
pub async fn scale_containers_with_params(
    target_group_arn: &str,
    fargate_arn: &str,
    delay: &str,
    html_template: &str,
    fargate_name: &str,
    elbv2_client: &Elbv2,
) -> ScaleResponse {
    if elbv2_client
        .register_target(target_group_arn, fargate_arn)
        .await.is_ok() {
        let html = html_template.replace("{delay}", &delay).replace("{name}", fargate_name);

        ScaleResponse {
            status: 200,
            headers: [("content-type".to_string(), "text/html".to_string())].into(),
            body: html.into(),
        }
    } else {  
        ScaleResponse {
            status: 500,
            headers: [("content-type".to_string(), "text/html".to_string())].into(),
            body: "<!DOCTYPE html><html lang=en><meta charset=utf-8><p>Failed to register target</p>".to_string(),
        }
    } 
}

pub async fn scale_containers() -> ScaleResponse {
    let target_group_arn = env::var("TARGET_GROUP_ARN")
        .expect("TARGET_GROUP_ARN env var must be set");
    let fargate_arn = env::var("FARGATE_ARN")
        .expect("FARGATE_ARN env var must be set");
    let delay = env::var("REFRESH_DELAY").unwrap_or_else(|_| "5".to_string());
    
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let elbv2_client = Elbv2::new(elbv2::Client::new(&config));
    
    scale_containers_with_params(
        &target_group_arn,
        &fargate_arn,
        &delay,
        &env::var("HTML_TEMPLATE").unwrap_or_else(|_| HTML_TEMPLATE.to_string()),
        &env::var("FARGATE_NAME").unwrap_or_else(|_| "Fargate container".to_string()),
        &elbv2_client,
    ).await
}

#[cfg(test)]
mod tests {
    
use super::*;
    use std::env;
    use aws_sdk_elasticloadbalancingv2::types::error::InvalidTargetException;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_scale_containers_with_params() {
        let mut mock = MockElbv2Impl::default();
        
        // Set up the mock expectation
        mock.expect_register_target()
            .with(eq("test-target-group"), eq("test-fargate"))
            .times(1)
            .returning(|_, _| Ok(RegisterTargetsOutput::builder().build()));

        let response = scale_containers_with_params(
            "test-target-group",
            "test-fargate",
            "3",
            "<html><body>Booting {name}, please wait...</body></html>",
            "TestFargate",
            &mock,
        ).await;

        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("content-type"), Some(&"text/html".to_string()));
        assert!(response.body.contains("TestFargate"));
    }

    #[tokio::test]
    async fn test_scale_containers_with_env_vars() {
        let mut mock = MockElbv2Impl::default();
        
        mock.expect_register_target()
            .with(eq("test-target-group"), eq("test-fargate"))
            .times(1)
            .returning(|_, _| Ok(RegisterTargetsOutput::builder().build()));
unsafe {
        env::set_var("TARGET_GROUP_ARN", "test-target-group");
        env::set_var("FARGATE_ARN", "test-fargate");
        env::set_var("REFRESH_DELAY", "2");
        env::set_var("FARGATE_NAME", "TestFargate");
}
        // Create a wrapper function that uses our mock
        async fn scale_containers_with_mock(mock: &Elbv2) -> ScaleResponse {
            let target_group_arn = env::var("TARGET_GROUP_ARN").unwrap();
            let fargate_arn = env::var("FARGATE_ARN").unwrap();
            let delay = env::var("REFRESH_DELAY").unwrap_or_else(|_| "5".to_string());
            
            scale_containers_with_params(
                &target_group_arn,
                &fargate_arn,
                &delay,
                &env::var("HTML_TEMPLATE").unwrap_or_else(|_| HTML_TEMPLATE.to_string()),
                &env::var("FARGATE_NAME").unwrap_or_else(|_| "Fargate container".to_string()),
                mock,
            ).await
        }

        let response = scale_containers_with_mock(&mock).await;

        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("content-type"), Some(&"text/html".to_string()));
        assert!(response.body.contains("TestFargate"));
    }

    #[tokio::test]
    async fn test_scale_containers_with_defaults() {
        let mut mock = MockElbv2Impl::default();
        
        mock.expect_register_target()
            .with(eq("test-target-group"), eq("test-fargate"))
            .times(1)
            .returning(|_, _| Ok(RegisterTargetsOutput::builder().build()));
unsafe {
    env::set_var("TARGET_GROUP_ARN", "test-target-group");
    env::set_var("FARGATE_ARN", "test-fargate");
}
        async fn scale_containers_with_mock(mock: &Elbv2) -> ScaleResponse {
            let target_group_arn = env::var("TARGET_GROUP_ARN").unwrap();
            let fargate_arn = env::var("FARGATE_ARN").unwrap();
            let delay = env::var("REFRESH_DELAY").unwrap_or_else(|_| "5".to_string());
            
            scale_containers_with_params(
                &target_group_arn,
                &fargate_arn,
                &delay,
                &env::var("HTML_TEMPLATE").unwrap_or_else(|_| HTML_TEMPLATE.to_string()),
                &env::var("FARGATE_NAME").unwrap_or_else(|_| "Fargate container".to_string()),
                mock,
            ).await
        }

        let response = scale_containers_with_mock(&mock).await;

        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("content-type"), Some(&"text/html".to_string()));
        assert!(response.body.contains("<h1>Booting TestFargate, please wait...</h1>"));
    }

    #[tokio::test]
    async fn test_register_target_failure() {
        let mut mock = MockElbv2Impl::default();
        
        mock.expect_register_target()
            .with(eq("test-target-group"), eq("test-fargate"))
            .times(1)
            .returning(|_, _| Err(elbv2::Error::InvalidTargetException(InvalidTargetException::builder().build())
            ));

        let result = scale_containers_with_params(
            "test-target-group",
            "test-fargate",
            "3",
            "<html><body>Booting {name}, please wait...</body></html>",
            "TestFargate",
            &mock,
        ).await;

        // The function should panic with the expect message
        assert_eq!(result.status , 500);
        assert_eq!(result.headers.get("content-type"), Some(&"text/html".to_string()));
        assert!(result.body.contains("Failed to register target"));
    }
}