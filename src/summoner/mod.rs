use super::{
    enums::Region,
    proxy::{request, Result},
    types::Summoner as SummonerDTO,
    Client, ClientContext,
};
use url::Url;

impl Client {
    pub fn summoner(&self) -> Summoner {
        Summoner {
            context: &self.context,
        }
    }
}

/// Namespace for the summoner related APIs.
#[derive(Clone)]
pub struct Summoner<'a> {
    context: &'a ClientContext,
}

impl Summoner<'_> {
    pub async fn get_summoner_by_encrypted_account_id(
        self,
        region: Region,
        encrypted_account_id: String,
    ) -> Result<SummonerDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/summoner/v4/summoners/by-account/{}",
            region.to_string(),
            encrypted_account_id
        ))?;

        request::<SummonerDTO>(url.as_str(), self.context).await
    }

    pub async fn get_summoner_by_summoner_name(
        self,
        region: Region,
        summoner_name: String,
    ) -> Result<SummonerDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/summoner/v4/summoners/by-name/{}",
            region.to_string(),
            summoner_name
        ))?;

        request::<SummonerDTO>(url.as_str(), self.context).await
    }

    pub async fn get_summoner_by_encrypted_puu_id(
        self,
        region: Region,
        encrypted_puu_id: String,
    ) -> Result<SummonerDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/summoner/v4/summoners/by-puuid/{}",
            region.to_string(),
            encrypted_puu_id
        ))?;

        request::<SummonerDTO>(url.as_str(), self.context).await
    }

    pub async fn get_summoner_by_encrypted_summoner_id(
        self,
        region: Region,
        encrypted_summoner_id: String,
    ) -> Result<SummonerDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/summoner/v4/summoners/{}",
            region.to_string(),
            encrypted_summoner_id
        ))?;

        request::<SummonerDTO>(url.as_str(), self.context).await
    }
}
