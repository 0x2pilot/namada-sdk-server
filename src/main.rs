use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;
use namada_light_sdk::namada_sdk::types::address::Address;
use namada_light_sdk::reading::asynchronous::account;
use namada_light_sdk::reading::asynchronous::governance::query_governance_parameters;
use namada_light_sdk::reading::asynchronous::governance::query_proposal_by_id;
use namada_light_sdk::reading::asynchronous::governance::query_proposal_votes;
use namada_light_sdk::reading::asynchronous::pos::query_epoch;
use namada_light_sdk::reading::asynchronous::pos::get_all_validators;
use namada_light_sdk::reading::asynchronous::pos::get_validator_stake;
use namada_light_sdk::reading::asynchronous::pos::get_validator_state;
use namada_light_sdk::reading::asynchronous::pos::query_metadata;
use namada_proof_of_stake::types::ValidatorState;
use namada_light_sdk::reading::asynchronous::tx;
use namada_light_sdk::reading::asynchronous::tx::query_tx_events;
use namada_light_sdk::reading::asynchronous::tx::query_tx_status;
use namada_sdk::governance::ProposalType;
use namada_sdk::state::Epoch;
use serde::Serialize;
use std::collections::BTreeMap;
use std::process::Command;
use serde_json::json;
use std::env;
use std::str::FromStr;

const NAAN_TOKEN_ADDERSS: &str = "tnam1qxvg64psvhwumv3mwrrjfcz0h3t3274hwggyzcee";

async fn get_epoch_info() -> impl Responder {
    match query_epoch(&get_tendermint_address()).await {
        Ok(epoch) => {
            let json_response = format!(r#"{{"epoch": "{}"}}"#, epoch.0);
            HttpResponse::Ok()
                .content_type("application/json")
                .body(json_response)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

async fn get_tx_response(info: web::Path<(String,)>) -> impl Responder {
    let tx_hash = info.into_inner().0;

    match tx::query_tx_response(&get_tendermint_address(), &tx_hash).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

async fn get_tx_status(info: web::Path<(String,)>) -> impl Responder {
    let tx_hash = info.into_inner().0;

    match query_tx_status(&get_tendermint_address(), &tx_hash).await {
        Ok(event) => {
            let json_event = format!(
                r#"{{"event_type": "{:?}", "level": "{:?}", "attributes": {:?} }}""#,
                event.event_type, event.level, event.attributes
            );
            return HttpResponse::Ok().body(json_event);
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

async fn get_tx_events(info: web::Path<(String,)>) -> impl Responder {
    let tx_hash = info.into_inner().0;

    match query_tx_events(&get_tendermint_address(), &tx_hash).await {
        Ok(event) => match event {
            Some(event) => {
                let json_event = format!(
                    r#"{{"event_type": "{:?}", "level": "{:?}", "attributes": {:?} }}""#,
                    event.event_type, event.level, event.attributes
                );
                HttpResponse::Ok().json(json_event)
            }
            None => HttpResponse::NotFound().body("Event not found"),
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

async fn get_balance(info: web::Path<(String,)>) -> impl Responder {
    let owner_str = info.into_inner().0;
    let owner_address = Address::from_str(&owner_str).expect("Invalid address format");
    let token_address = Address::from_str(NAAN_TOKEN_ADDERSS).expect("Invalid address format");

    match account::get_token_balance(&get_tendermint_address(), &token_address, &owner_address)
        .await
    {
        Ok(balance) => HttpResponse::Ok().body(balance.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

fn get_tendermint_address() -> String {
    match env::var("TENDERMINT_ADDR") {
        Ok(val) => val,
        Err(_) => panic!("Environment variable TENDERMINT_ADDR is not set"),
    }
}

fn get_tendermint_address_http() -> String {
    match env::var("TENDERMINT_ADDR_HTTP") {
        Ok(val) => val,
        Err(_) => panic!("Environment variable TENDERMINT_ADDR is not set"),
    }
}

async fn get_governance_parameters() -> impl Responder {
    match query_governance_parameters(&get_tendermint_address()).await {
        Ok(params) => {
            let json_response = format!(
                r#"{{"min_proposal_fund": "{}", "max_proposal_code_size": "{}", "min_proposal_voting_period": "{}", "max_proposal_period": "{}", "max_proposal_content_size": "{}", "min_proposal_grace_epochs": "{}"}}"#,
                params.min_proposal_fund,
                params.max_proposal_code_size,
                params.min_proposal_voting_period,
                params.max_proposal_period,
                params.max_proposal_content_size,
                params.min_proposal_grace_epochs
            );
            HttpResponse::Ok()
                .content_type("application/json")
                .body(json_response)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

#[derive(Serialize)]
struct ProposalResponse {
    id: u64,
    content: BTreeMap<String, String>,
    author: String,
    #[serde(rename = "type")]
    typee: ProposalType,
    voting_start_epoch: String,
    voting_end_epoch: String,
    grace_epoch: String,
}

async fn get_proposal_by_id(info: web::Path<(u64,)>) -> impl Responder {
    let proposal_id = info.into_inner().0;

    match query_proposal_by_id(&get_tendermint_address(), proposal_id).await {
        Ok(p) => {
            let proposal = p.expect("Error retrieving the proposal");

            let response = ProposalResponse {
                id: proposal.id,
                content: proposal.content,
                author: proposal.author.to_string(),
                typee: proposal.r#type,
                voting_start_epoch: proposal.voting_start_epoch.to_string(),
                voting_end_epoch: proposal.voting_end_epoch.to_string(),
                grace_epoch: proposal.grace_epoch.to_string(),
            };

            match serde_json::to_string(&response) {
                Ok(json_response) => HttpResponse::Ok().content_type("application/json").body(json_response),
                Err(_) => HttpResponse::InternalServerError().body("Error serializing response"),
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

async fn get_proposal_votes(info: web::Path<(u64,)>) -> impl Responder {
    let proposal_id = info.into_inner().0;

    match query_proposal_votes(&get_tendermint_address(), proposal_id).await {
        Ok(votes) => {
            let json_votes = votes
                .iter()
                .map(|vote| {
                    json!({
                        "validator": vote.validator,
                        "delegator": vote.delegator,
                        "data": vote.data
                    })
                })
                .collect::<Vec<_>>();

            HttpResponse::Ok().json(json_votes)
        }
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

async fn get_validators(info: web::Path<(u64,)>) -> impl Responder {
    let epoch = info.into_inner().0;
    match get_all_validators(&get_tendermint_address(), Epoch(epoch)).await {
        Ok(validators) => HttpResponse::Ok().json(validators),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn get_validator_stake_handler(info: web::Path<(u64, String)>) -> impl Responder {
    let (epoch, validator) = info.into_inner();
    let validator = Address::from_str(&validator).expect("Invalid address format");

    match get_validator_stake(&get_tendermint_address(), Epoch(epoch), &validator).await {
        Ok(stake) => HttpResponse::Ok().body(stake.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

async fn get_validator_state_handler(info: web::Path<(u64, String)>) -> impl Responder {
    let (epoch, validator) = info.into_inner();
    let validator = Address::from_str(&validator).expect("Invalid address format");

    match get_validator_state(&get_tendermint_address(), &validator, Some(Epoch(epoch))).await {
        Ok(Some(state)) => {
            let json_response = match state {
                ValidatorState::Consensus => json!({ "state": "Consensus" }),
                ValidatorState::BelowCapacity => json!({ "state": "BelowCapacity" }),
                ValidatorState::BelowThreshold => json!({ "state": "BelowThreshold" }),
                ValidatorState::Inactive => json!({ "state": "Inactive" }),
                ValidatorState::Jailed => json!({ "state": "Jailed" }),
            };
            HttpResponse::Ok().json(json_response)
        },
        Ok(None) => HttpResponse::NotFound().body("Validator state not found"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

async fn get_total_staked_tokens_handler(info: web::Path<(u64,)>) -> impl Responder {
    let epoch = info.into_inner().0;

    match namada_light_sdk::reading::asynchronous::pos::get_total_staked_tokens(&get_tendermint_address(), Epoch(epoch)).await {
        Ok(amount) => HttpResponse::Ok().body(amount.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

async fn get_validator_metadata(info: web::Path<(u64, String)>) -> impl Responder {
    let (epoch, validator) = info.into_inner();
    let validator = Address::from_str(&validator).expect("Invalid address format");

    match query_metadata(&get_tendermint_address(), &validator, Some(Epoch(epoch))).await {
        Ok((metadata, commission_pair)) => {
            let json_response = json!({
                "metadata": {
                    "email": metadata.as_ref().map(|m| m.email.as_str()).unwrap_or(""),
                    "description": metadata.as_ref().and_then(|m| m.description.as_deref()).unwrap_or(""),
                    "website": metadata.as_ref().and_then(|m| m.website.as_deref()).unwrap_or(""),
                    "discord_handle": metadata.as_ref().and_then(|m| m.discord_handle.as_deref()).unwrap_or(""),
                    "avatar": metadata.as_ref().and_then(|m| m.avatar.as_deref()).unwrap_or("")
                },
                "commission_pair": commission_pair.map(|cp| json!({
                    "rate": cp.commission_rate,
                    "max_rate": cp.max_commission_change_per_epoch
                })).unwrap_or(json!({})),
            });
            HttpResponse::Ok().json(json_response)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

async fn get_latest_proposal_id() -> impl Responder {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("namadac query-proposal --ledger-address {}", get_tendermint_address_http())) // Ensure this command is correct and executable
        .output();

    // Handle potential execution error more gracefully
    if let Err(e) = output {
        return HttpResponse::InternalServerError().body(format!("Failed to execute process: {}", e));
    }

    let output = output.unwrap(); // Safe due to previous return on error

    // Check for successful execution
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr); // Use lossy conversion to handle potential non-UTF-8 sequences
        return HttpResponse::InternalServerError().body(format!("Command execution failed: {}", error_message));
    }

    let output_str = String::from_utf8_lossy(&output.stdout); // Convert once, reuse

    let mut latest_proposal_id = 0;

    for line in output_str.lines() {
        if line.starts_with("Proposal Id:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(id_str) = parts.last() {
                if let Ok(id) = id_str.parse::<u32>() {
                    if id > latest_proposal_id {
                        latest_proposal_id = id;
                    }
                }
            }
        }
    }

    HttpResponse::Ok().body(latest_proposal_id.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .route("/epoch", web::get().to(get_epoch_info))
            .route("/validators/{epoch}", web::get().to(get_validators))
            .route("/validator/state/{epoch}/{validator}", web::get().to(get_validator_state_handler))
            .route("/validator/stake/{epoch}/{validator}", web::get().to(get_validator_stake_handler))
            .route("/validator/metadata/{epoch}/{validator}", web::get().to(get_validator_metadata))
            .route("/balance/{owner}", web::get().to(get_balance))
            .route("/tx/{tx_hash}", web::get().to(get_tx_response))
            .route("/tx/status/{tx_hash}", web::get().to(get_tx_status))
            .route("/tx/events/{tx_hash}", web::get().to(get_tx_events))
            .route(
                "/governance/parameters",
                web::get().to(get_governance_parameters),
            )
            .route("/proposal/{proposal_id}", web::get().to(get_proposal_by_id))
            .route(
                "/proposal/votes/{proposal_id}",
                web::get().to(get_proposal_votes),
            )
            .route("/total-staked/{epoch}", web::get().to(get_total_staked_tokens_handler))
            .service(web::resource("/latest-proposal-id").route(web::get().to(get_latest_proposal_id)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
