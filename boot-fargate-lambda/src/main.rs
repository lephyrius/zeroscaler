use lambda_http::{run, service_fn, tracing, Error};
use zeroscaler_boot_fargate::scale_containers;



async fn function_handler(_: lambda_http::Request) -> Result<lambda_http::Response<String>, Error> {

    // Call the scale_containers function with the container name
    let result = scale_containers().await;
    
    let mut response_builder = lambda_http::Response::builder()
        .status(result.status);

    for (header,value) in result.headers {
        response_builder = response_builder.header(header, value);
    }
    
    // Return the result as a response
    Ok(response_builder
        .body(result.body)
        .map_err(Box::new)?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
