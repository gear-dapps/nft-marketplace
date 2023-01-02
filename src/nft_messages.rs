use crate::{ContractId, TokenId, TransactionId};
use gstd::{msg, prelude::*, ActorId};
use primitive_types::U256;
pub type Payout = BTreeMap<ActorId, u128>;
use nft_io::*;

pub async fn nft_transfer(
    transaction_id: TransactionId,
    nft_program_id: &ActorId,
    to: &ActorId,
    token_id: U256,
) -> Result<(), ()> {
    let reply: Result<NFTEvent, _> = msg::send_for_reply_as(
        *nft_program_id,
        NFTAction::Transfer {
            transaction_id,
            to: *to,
            token_id,
        },
        0,
    )
    .expect("Error in sending a message `NFTAction::Transfer`")
    .await;

    match reply {
        Ok(_) => Ok(()),
        _ => Err(()),
    }
}

pub async fn payouts(nft_program_id: &ActorId, owner: &ActorId, amount: u128) -> Payout {
    let reply = msg::send_for_reply_as::<_, NFTEvent>(
        *nft_program_id,
        NFTAction::NFTPayout {
            owner: *owner,
            amount,
        },
        0,
    )
    .expect("Error in sending a message `NFTAction::NFTPayout`")
    .await
    .expect("Unable to decode `NFTEvent`");
    match reply {
        NFTEvent::NFTPayout(payout) => payout,
        _ => panic!("Wrong received reply"),
    }
}

pub async fn get_owner(nft_contract_id: &ContractId, token_id: TokenId) -> ActorId {
    let reply: NFTEvent =
        msg::send_for_reply_as(*nft_contract_id, NFTAction::Owner { token_id }, 0)
            .expect("Error in sending a message `NFTAction::Owner`")
            .await
            .expect("Unable to decode `NFTEvent`");
    match reply {
        NFTEvent::Owner { owner, token_id } => owner,
        _ => panic!("Wrong received message"),
    }
}
