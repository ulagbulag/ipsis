use ipis::{
    async_trait::async_trait,
    core::anyhow::{Ok, Result},
    env::Infer,
};
use ipsis_api_persistent_s3::IpsisClient;

pub struct ProtocolImpl {
    client: IpsisClient,
}

impl ProtocolImpl {
    pub async fn try_new() -> Result<Self> {
        // init client
        let client = IpsisClient::try_infer().await?;

        Ok(Self { client })
    }
}

#[async_trait]
impl super::Protocol for ProtocolImpl {
    async fn to_string(&self) -> Result<String> {
        Ok("s3".into())
    }

    async fn read(&self, ctx: super::BenchmarkCtx) -> Result<()> {
        super::read(&self.client, ctx).await
    }

    async fn write(&self, ctx: super::BenchmarkCtx) -> Result<()> {
        super::write(&self.client, ctx).await
    }

    async fn cleanup(&self, ctx: super::BenchmarkCtx) -> Result<()> {
        super::cleanup(&self.client, ctx).await
    }
}
