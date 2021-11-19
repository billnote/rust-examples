use hello_grpc::greeter_client::GreeterClient;
use hello_grpc::HelloRequest;

pub mod hello_grpc {
    tonic::include_proto!("hellogrpc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:7777").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "bill".into(),
    });

    let response = client.say_hello(request).await?;

    print!("RESPONSE={:?}", response);

    Ok(()) 
}
