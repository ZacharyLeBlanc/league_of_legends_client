use super::{
    client::{Client, ClientContext},
    dto::{MatchDTO, MatchListDTO},
    enums::Region,
    proxy::{request, Result},
};
use std::collections::HashSet;
use url::Url;

impl Client {
    pub fn r#match(self) -> Match {
        Match {
            context: self.context,
        }
    }
}

#[derive(Clone)]
pub struct Match {
    context: ClientContext,
}

impl Match {
    pub async fn get_match_by_id(self, region: Region, match_id: i64) -> Result<MatchDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/match/v4/matches/{}",
            region.to_string(),
            match_id.to_string()
        ))?;

        request::<MatchDTO>(url.as_str(), self.context).await
    }

    pub async fn get_match_list_by_account(
        self,
        region: Region,
        encrypted_account_id: String,
        champion: Option<HashSet<i32>>,
        queue: Option<HashSet<i32>>,
        end_time: Option<i64>,
        begin_time: Option<i64>,
        end_index: Option<i32>,
        begin_index: Option<i32>,
    ) -> Result<MatchListDTO> {
        let mut url = Url::parse(&format!(
            "https://{}/lol/match/v4/matchlists/by-account/{}",
            region.to_string(),
            encrypted_account_id
        ))?;

        if let Some(champion) = champion {
            for champion in champion {
                url.query_pairs_mut()
                    .append_pair("champion", &champion.to_string());
            }
        }

        if let Some(queue) = queue {
            for queue in queue {
                url.query_pairs_mut()
                    .append_pair("queue", &queue.to_string());
            }
        }

        if let Some(end_time) = end_time {
            url.query_pairs_mut()
                .append_pair("endTime", &end_time.to_string());
        }

        if let Some(begin_time) = begin_time {
            url.query_pairs_mut()
                .append_pair("beginTime", &begin_time.to_string());
        }

        if let Some(end_index) = end_index {
            url.query_pairs_mut()
                .append_pair("endIndex", &end_index.to_string());
        }

        if let Some(begin_index) = begin_index {
            url.query_pairs_mut()
                .append_pair("begin_index", &begin_index.to_string());
        }

        request::<MatchListDTO>(url.as_str(), self.context).await
    }
}
