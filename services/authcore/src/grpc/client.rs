use tonic::transport::Channel;

use self::email::email_service_client::EmailServiceClient;

/// Tonic-generated gRPC bindings
pub mod email {
    tonic::include_proto!("messaging.email");
}

pub type EmailClient = EmailServiceClient<Channel>;

pub async fn connect_to_email_grpc_server() -> Result<EmailClient, Box<dyn std::error::Error>> {
    let client =
        email::email_service_client::EmailServiceClient::connect("http://0.0.0.0:50051").await?;

    Ok(client)
}
