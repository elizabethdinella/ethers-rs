use crate::{Client, ProxyResponse, Result};
use ethers_core::abi::Address;
use serde::Deserialize;
use std::collections::HashMap;



#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ResponseData<T> {
    Success(ProxyResponse<T>),
    Error { status: String, message: String, result: Option<String> },
}
impl Client {
    pub async fn get_impl_address(
        &self,
        address: &Address
    ) -> Result<String> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("address", format!("{address:?}"));
        params.insert(
            "position",
            format!("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc"),
        );
        let query = self.create_query("proxy", "eth_getStorageAt", params);
        let response: ProxyResponse<String> = self.get_json(&query).await?;

        Ok(response.result)
    }

    pub async fn get_cur_block(
        &self) -> Result<String> {
        let params: HashMap<&str, String> = HashMap::new();
        let query = self.create_query("proxy", "eth_blockNumber", params);
        let response: ProxyResponse<String> = self.get_json(&query).await?;

        Ok(response.result)
    }
}
