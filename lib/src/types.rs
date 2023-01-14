use hdk::prelude::*;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GetRankingDirection {
    Ascendent,
    Descendent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRankingCursor {
    pub from_ranking: i64,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct RankingTag {
    pub ranking: i64,
    pub custom_tag: Option<SerializedBytes>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct EntryHashWithTag {
    pub entry_hash: EntryHash,
    pub tag: Option<SerializedBytes>,
}

pub struct RankingIndex {
    pub name: &'static str,
    pub index_interval: u64,
}

pub type EntryRanking = BTreeMap<i64, Vec<EntryHashWithTag>>;
