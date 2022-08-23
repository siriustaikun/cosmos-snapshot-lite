use std::{iter::MapWhile, ops::RangeFrom};

use eyre::Result;
use ocular::{rpc::new_http_client, cosmrs::{Error, proto::cosmos::staking::v1beta1::Validator}, query::{StakingQueryClient, PageRequest}, QueryClient};



#[tokio::main]
async fn main () {
    execute_lite_snapshot().await.unwrap();
}

pub fn juno_client() -> QueryClient {
    QueryClient::new(
        "http://cosmoshub.strange.love:26657",
        "http://cosmoshub.strange.love:9090",
    )
    .expect("failed to construct Cosmos client")
}

fn page_request_iter () -> impl Iterator<Item=PageRequest> {
    let mut pr =  PageRequest {
        key: Vec::<u8>::default(),
        offset: 0,
        limit: 100,
        count_total: false,
        reverse: false,
    };
    (0..).map_while(move |n| {
        pr.offset += n * pr.limit;
        Some(pr.clone())
    })
}

async fn execute_lite_snapshot () -> Result<()> {
    let mut client = juno_client();
    
    let mut pr = page_request_iter();
    let mut validators: Vec<Validator> = vec![];
    loop {
        let mut vals = client.validators(&"", pr.next()).await?;
        validators.append(vals.validators.as_mut());
        if vals.validators.is_empty() {
            break;
        }
    }
    let all_vals: String = validators.into_iter().map(|v| v.operator_address).collect::<Vec<String>>().join("\n");
    println!("{}", all_vals);
    Ok(())
}