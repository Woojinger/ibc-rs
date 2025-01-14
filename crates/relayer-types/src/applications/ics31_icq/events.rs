use crate::prelude::*;
use crate::core::ics24_host::identifier::{ChainId, ConnectionId};
use crate::events::IbcEvent;

use super::error::Error;

use core::str::FromStr;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use tendermint::block::Height;
use tendermint_rpc::abci::{Event as AbciEvent, Path as TendermintPath};
use tendermint_rpc::abci::tag::Tag;

const EVENT_TYPE_PREFIX: &str = "query_request";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CrossChainQueryPacket {
    pub module: String,
    pub action: String,
    pub query_id: String,
    pub chain_id: ChainId,
    pub connection_id: ConnectionId,
    pub query_type: TendermintPath,
    pub height: Height,
    pub request: String,
}

fn find_value(key: &str, entries: &[Tag]) -> Result<String, Error> {
    entries
        .iter()
        .find_map(|entry| {
            if entry.key.as_ref() == key {
                Some(entry.value.to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| Error::attribute(key.to_string()))
}

fn new_tag(key: &str, value: &str) -> Tag {
    Tag {
        key: key.parse().unwrap(),
        value: value.parse().unwrap(),
    }
}

impl From<CrossChainQueryPacket> for AbciEvent {
    fn from(packet: CrossChainQueryPacket) -> Self {
        let attributes: Vec<Tag> = vec![
            new_tag("module", packet.module.as_str()),
            new_tag("action", packet.action.as_str()),
            new_tag("query_id", packet.query_id.as_str()),
            new_tag("chain_id", packet.chain_id.as_str()),
            new_tag("connection_id", packet.connection_id.as_str()),
            new_tag("type", &packet.query_type.to_string()),
            new_tag("request", packet.request.as_str()),
            new_tag("height", &packet.height.to_string()),
        ];

        AbciEvent {
            type_str: String::from("message"),
            attributes,
        }
    }
}

impl<'a> TryFrom<&'a Vec<Tag>> for CrossChainQueryPacket {
    type Error = Error;

    fn try_from(entries: &'a Vec<Tag>) -> Result<Self, Self::Error> {
        let module = find_value("module", &entries)?;
        let action = find_value("action", &entries)?;
        let query_id = find_value("query_id", &entries)?;
        let chain_id_str = find_value("chain_id", &entries)?;
        let connection_id_str = find_value("connection_id", &entries)?;
        let query_type_str = find_value("type", &entries)?;
        let request = find_value("request", &entries)?;
        let height_str = find_value("height", &entries)?;

        let chain_id = ChainId::from_string(&chain_id_str);
        let connection_id = ConnectionId::from_str(&connection_id_str).map_err(|_| Error::ics24())?;
        let query_type = TendermintPath::from_str(&query_type_str).map_err(|_| Error::tendermint())?;
        let height = Height::from_str(&height_str).map_err(|_| Error::tendermint())?;

        Ok(
            Self {
                module,
                action,
                query_id,
                chain_id,
                connection_id,
                query_type,
                height,
                request,
            }
        )
    }
}

fn fetch_first_element_from_events(
    block_events: &BTreeMap<String, Vec<String>>,
    key: &str,
) -> Result<String, Error> {
    let res = block_events.get(key)
        .ok_or_else(|| vec![])
        .map_err(|_: Vec<&String>| Error::parse())?
        .get(0)
        .ok_or_else(|| Err(()))
        .map_err(|_: Result<&String, ()>| Error::parse())?;

    Ok(res.clone())
}

impl CrossChainQueryPacket {
    pub fn extract_query_event(
        block_events: &BTreeMap<String, Vec<String>>,
    ) -> Result<IbcEvent, Error> {
        let chain_id_str = fetch_first_element_from_events(block_events, &format!("{}.{}", EVENT_TYPE_PREFIX, "chain_id"))?;
        let connection_id_str = fetch_first_element_from_events(block_events, &format!("{}.{}", EVENT_TYPE_PREFIX, "connection_id"))?;
        let query_type_str = fetch_first_element_from_events(block_events, &format!("{}.{}", EVENT_TYPE_PREFIX, "type"))?;
        let height_str = fetch_first_element_from_events(block_events, &format!("{}.{}", EVENT_TYPE_PREFIX, "height"))?;

        Ok(
            IbcEvent::CrossChainQueryPacket(CrossChainQueryPacket {
                module: fetch_first_element_from_events(block_events, &format!("{}.{}", EVENT_TYPE_PREFIX, "module"))?,
                action: fetch_first_element_from_events(block_events, &format!("{}.{}", EVENT_TYPE_PREFIX, "action"))?,
                query_id: fetch_first_element_from_events(block_events, &format!("{}.{}", EVENT_TYPE_PREFIX, "query_id"))?,
                chain_id: ChainId::from_string(&chain_id_str),
                connection_id: ConnectionId::from_str(&connection_id_str).map_err(|_| Error::parse())?,
                query_type: TendermintPath::from_str(&query_type_str).map_err(|_| Error::parse())?,
                height: Height::from_str(&height_str).map_err(|_| Error::parse())?,
                request: fetch_first_element_from_events(block_events, &format!("{}.{}", EVENT_TYPE_PREFIX, "request"))?,
            })
        )
    }
}