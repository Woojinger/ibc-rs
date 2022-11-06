use crate::chain::requests::CrossChainQueryRequest;
use crate::chain::responses::CrossChainQueryResponse;
use reqwest::{Client, Error};

pub async fn rest_query(
    client: &Client,
    request: CrossChainQueryRequest,
) -> Result<CrossChainQueryResponse, Error> {
    let raw_path = request.decode_path_or_none();

    match raw_path {
        Some(path) => {
            let response = client
                .get(path)
                .header("x-cosmos-block-height", request.height.to_string())
                .send()
                .await?;

            let data = response.text().await;

            match data {
                Ok(res) => Ok(CrossChainQueryResponse::new(
                    request.id,
                    request.sender,
                    1,
                    res,
                    request.height,
                )),
                Err(e) => Ok(CrossChainQueryResponse::new(
                    request.id,
                    request.sender,
                    2,
                    e.to_string(),
                    request.height,
                )),
            }
        }
        None => Ok(CrossChainQueryResponse::new(
            request.id,
            request.sender,
            2,
            "".to_string(),
            request.height,
        )),
    }
}
