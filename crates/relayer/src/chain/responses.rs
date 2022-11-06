use crate::chain::handle::ChainHandle;
use core::fmt::{Display, Formatter};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::applications::query::v1::MsgSubmitCrossChainQueryResult;
use prost;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CrossChainQueryResponse {
    pub id: String,
    pub sender: String,
    pub result: i32,
    pub data: String,
    pub height: String,
}

impl CrossChainQueryResponse {
    pub fn new(id: String, sender: String, result: i32, data: String, height: String) -> Self {
        Self {
            id,
            sender,
            result,
            data,
            height,
        }
    }

    pub fn to_any<QueryingChain: ChainHandle>(&self, _handle: &QueryingChain) -> Any {
        let mut encoded = Vec::new();

        let msg_submit_cross_chain_query_result = MsgSubmitCrossChainQueryResult {
            id: self.id.to_string(),
            query_height: self.height.parse().unwrap(),
            result: self.result,
            data: self.data.as_bytes().to_vec(),
            sender: self.sender.clone(),
            proof_specs: vec![],
        };
        prost::Message::encode(&msg_submit_cross_chain_query_result, &mut encoded).unwrap();
        Any {
            type_url: "/ibc.applications.ibc_query.v1.MsgSubmitCrossChainQueryResult".to_string(),
            value: encoded,
        }
    }
}

impl Display for CrossChainQueryResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "id: {}, data: {}, height: {}",
            self.id, self.data, self.height
        )
    }
}
