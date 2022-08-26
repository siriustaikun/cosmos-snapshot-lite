use eyre::Result;
use ocular::{cosmrs::{proto::cosmos::staking::v1beta1::{Validator, Delegation, DelegationResponse}}, query::{StakingQueryClient, PageRequest}, QueryClient};


#[tokio::main]
async fn main () {
    execute_lite_snapshot().await.expect("failed");
}

pub fn cosmos_client() -> QueryClient {
    QueryClient::new(
        "https://rpc-cosmoshub-ia.notional.ventures:443",
        "https://grpc-cosmoshub-ia.notional.ventures:443",
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
        pr.offset = n * pr.limit;
        Some(pr.clone())
    })
}

async fn execute_lite_snapshot () -> Result<Vec<DelegationResponse>> {
    let mut client = cosmos_client();
    
    let mut pr = page_request_iter();
    let mut validators: Vec<Validator> = vec![];
    loop {
        println!("initialized");
        let mut vals = client.validators(&"", pr.next()).await?;
        println!("vals loaded");
        if vals.validators.is_empty() {
            break;
        } else {
            validators.append(vals.validators.as_mut());
        }
    }
    
    let mut delegations: Vec<DelegationResponse> = vec![];
    for val in validators {
        let val = val.clone();
        let mut val_dels = load_delegations(&val).await?;
        delegations.append(&mut val_dels);
    }
    Ok(delegations)
}

async fn load_delegations (val: &Validator) -> Result<Vec<DelegationResponse>> {
    let mut client = cosmos_client();
    let mut pr = page_request_iter();
    let mut delegations: Vec<DelegationResponse> = vec![];
    loop {
        println!("loading dels");
        let mut res = client.validator_delegations(&val.operator_address, pr.next()).await?;
        if res.delegation_responses.is_empty() {
            println!("exiting");
            break;
        } else {
            delegations.append(res.delegation_responses.as_mut());
            println!("total delegations: {}", delegations.len());
        }
    }
    Ok(delegations)
}