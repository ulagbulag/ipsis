use bytecheck::CheckBytes;
use ipiis_api::{client::IpiisClient, common::Ipiis, server::IpiisServer};
use ipis::{
    class::Class,
    core::{anyhow::Result, signed::IsSigned},
    env::{infer, Infer},
    tokio,
};
use ipsis_api::{
    common::{Ipsis, KIND},
    server::IpsisServer,
};
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Class, Clone, Debug, PartialEq, Eq, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct MyData {
    name: String,
    age: u32,
}

impl IsSigned for MyData {}

#[tokio::main]
async fn main() -> Result<()> {
    // deploy a server
    let server = IpsisServer::genesis(9801).await?;
    let server_account = {
        let server: &IpiisServer = server.as_ref();
        *server.account_ref()
    };
    tokio::spawn(async move { server.run().await });

    // create a client
    let client = IpiisClient::genesis(None).await?;
    client
        .set_account_primary(KIND.as_ref(), &server_account)
        .await?;
    client
        .set_address(
            KIND.as_ref(),
            &server_account,
            &infer("ipiis_client_account_primary_address")?,
        )
        .await?;

    // let's make a data we want to store
    let mut data = MyData {
        name: "Alice".to_string(),
        age: 24,
    };

    // CREATE
    let path_create = client.put(&data).await?;
    assert!(client.contains(&path_create).await?);

    // UPDATE (identity)
    let path_update_identity = client.put(&data).await?;
    assert_eq!(&path_create, &path_update_identity); // SAME Path

    // let's modify the data so that it has a different path
    data.name = "Bob".to_string();

    // UPDATE (changed)
    let path_update_changed = client.put(&data).await?;
    assert_ne!(&path_create, &path_update_changed); // CHANGED Path

    // READ
    let data_from_storage: MyData = client.get(&path_update_changed).await?;
    assert_eq!(&data, &data_from_storage);

    // DELETE
    client.delete(&path_create).await?;
    client.delete(&path_update_changed).await?;

    // data is not exist after DELETE
    #[cfg(not(feature = "ipfs"))]
    {
        match client.get::<MyData>(&path_update_changed).await {
            Ok(_) => ::ipis::core::anyhow::bail!("data not deleted!"),
            Err(_) => {
                assert!(!client.contains(&path_create).await?);
                assert!(!client.contains(&path_update_changed).await?);
            }
        }
    }
    Ok(())
}
