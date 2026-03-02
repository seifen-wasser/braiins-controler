use wink::{
    GetMinerConfigurationRequest, LoginRequest, PauseMiningRequest, ResumeMiningRequest,
    actions_service_client::ActionsServiceClient,
    authentication_service_client::AuthenticationServiceClient,
    configuration_service_client::ConfigurationServiceClient,
};

use tokio::time::{Duration, sleep};
use tonic::{
    Extensions,
    metadata::{Ascii, MetadataMap, MetadataValue},
};

pub struct Braiins {
    pub ip: String,
    pub username: String,
    pub password: String,
    pub auth_token: Option<MetadataValue<Ascii>>,
}

impl Braiins {
    pub async fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client =
            AuthenticationServiceClient::connect(format!("http://{}:50051", self.ip)).await?;

        let request = tonic::Request::new(LoginRequest {
            username: self.username.clone(),
            password: self.password.clone(),
        });

        let response = client.login(request).await?;

        self.auth_token = Some(response.metadata().get("authorization").unwrap().clone());

        Ok(())
    }

    pub async fn pause_miner(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = ActionsServiceClient::connect(format!("http://{}:50051", self.ip)).await?;

        let mut metadata = MetadataMap::new();
        if let Some(token) = &self.auth_token {
            metadata.insert("authorization", token.clone());
        }

        let pause_request = tonic::Request::from_parts(metadata, Extensions::default(), PauseMiningRequest {});

        let _ = client.pause_mining(pause_request).await?;

        Ok(())
    }

    pub async fn resume_miner(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = ActionsServiceClient::connect(format!("http://{}:50051", self.ip)).await?;

        let mut metadata = MetadataMap::new();
        if let Some(token) = &self.auth_token {
            metadata.insert("authorization", token.clone());
        }

        let resume_request = tonic::Request::from_parts(metadata, Extensions::default(), ResumeMiningRequest {});

        let _ = client.resume_mining(resume_request).await?;

        Ok(())
    }
}
