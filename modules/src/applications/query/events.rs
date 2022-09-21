use crate::applications::query::packet::CrossChainQueryPacket;
use crate::core::ics04_channel::error::Error;
use crate::prelude::*;
use core::fmt::{Display, Formatter};
use serde::Serialize;
use tendermint::abci::Event as AbciEvent;
use crate::events::IbcEventType;

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct SendPacket {
    pub packet: CrossChainQueryPacket,
}

impl Display for SendPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.packet)
    }
}

impl TryFrom<SendPacket> for AbciEvent {
    type Error = Error;

    fn try_from(_value: SendPacket) -> Result<Self, Self::Error> {
        Ok(AbciEvent {
            type_str: IbcEventType::CrossChainQuery.as_str().to_string(),
            attributes: vec![],
        })
    }
}
