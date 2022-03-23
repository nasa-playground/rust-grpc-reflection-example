pub mod api {
    tonic::include_proto!("grpc.reflection.v1alpha");
}

use api::message_request;
use api::server_reflection_client::ServerReflectionClient;
use api::ServerReflectionRequest;

use futures::stream::Stream;
use tokio_stream::StreamExt;

fn echo_requests_iter() -> impl Stream<Item = ServerReflectionRequest> {
    tokio_stream::iter(1..usize::MAX).map(|i| ServerReflectionRequest {
        host: "localhost".into(),
        message_request: None,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ServerReflectionClient::connect("http://127.0.0.1:50052").await?;

    let num = 1;

    let in_stream = echo_requests_iter().take(num);

    let response = client.server_reflection_info(in_stream).await.unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(recived) = resp_stream.next().await {
        let recived = recived.unwrap();
        println!("\trecived message: `{:?}`", recived);
    }

    Ok(())
}
