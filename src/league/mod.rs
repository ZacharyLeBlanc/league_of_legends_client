use super::{
    client::{Client, ClientContext},
    dto::{LeagueEntryDTO, LeagueListDTO},
    enums::{Division, Queue, Region, Tier},
    proxy::{request, Result},
};
use url::Url;

impl Client {
    pub fn league(self) -> League {
        League {
            context: self.context,
        }
    }
}

#[derive(Clone)]
pub struct League {
    context: ClientContext,
}

impl League {
    pub async fn get_challenger_league(
        self,
        region: Region,
        queue: Queue,
    ) -> Result<LeagueListDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/league/v4/challengerleagues/by-queue/{}",
            region.to_string(),
            queue.to_string()
        ))?;

        request::<LeagueListDTO>(url.as_str(), self.context).await
    }

    pub async fn get_league_entries_for_summoner(
        self,
        region: Region,
        encrypted_summoner_id: String,
    ) -> Result<Vec<LeagueEntryDTO>> {
        let url = Url::parse(&format!(
            "https://{}/lol/league/v4/entries/by-summoner/{}",
            region.to_string(),
            encrypted_summoner_id
        ))?;

        request::<Vec<LeagueEntryDTO>>(url.as_str(), self.context).await
    }

    pub async fn get_league_entries(
        self,
        region: Region,
        queue: Queue,
        tier: Tier,
        division: Division,
        page: Option<i8>,
    ) -> Result<Vec<LeagueEntryDTO>> {
        let mut url = Url::parse(&format!(
            "https://{}/lol/league/v4/entries/{}/{}/{}",
            region.to_string(),
            queue,
            tier,
            division,
        ))?;

        if let Some(page) = page {
            url.query_pairs_mut().append_pair("page", &page.to_string());
        }

        request::<Vec<LeagueEntryDTO>>(url.as_str(), self.context).await
    }

    pub async fn get_grandmaster_league(
        self,
        region: Region,
        queue: Queue,
    ) -> Result<LeagueListDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/league/v4/grandmasterleagues/by-queue/{}",
            region.to_string(),
            queue.to_string()
        ))?;

        request::<LeagueListDTO>(url.as_str(), self.context).await
    }

    pub async fn get_league(self, region: Region, league_id: String) -> Result<LeagueListDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/league/v4/leagues/{}",
            region.to_string(),
            league_id
        ))?;

        request::<LeagueListDTO>(url.as_str(), self.context).await
    }

    pub async fn get_master_league(self, region: Region, queue: Queue) -> Result<LeagueListDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/league/v4/masterleagues/by-queue/{}",
            region.to_string(),
            queue.to_string()
        ))?;

        request::<LeagueListDTO>(url.as_str(), self.context).await
    }
}
