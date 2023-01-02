#![no_std]
use gstd::{prelude::*, ActorId};
use primitive_types::{H256, U256};

pub type ContractId = ActorId;
pub type TokenId = U256;
pub type Price = u128;
pub type TransactionId = u64;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitMarket {
    pub admin_id: ActorId,
    pub treasury_id: ActorId,
    pub treasury_fee: u8,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct Offer {
    pub hash: H256,
    pub id: ActorId,
    pub ft_contract_id: Option<ActorId>,
    pub price: u128,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
pub struct Auction {
    pub bid_period: u64,
    pub started_at: u64,
    pub ended_at: u64,
    pub current_price: Price,
    pub current_winner: ActorId,
    pub transaction: Option<(ActorId, Price, TransactionId)>,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone, Default)]
pub struct Item {
    pub owner: ActorId,
    pub ft_contract_id: Option<ContractId>,
    pub price: Option<Price>,
    pub auction: Option<Auction>,
    pub offers: BTreeMap<(Option<ContractId>, Price), ActorId>,
    pub bids: BTreeMap<(Option<ContractId>, Price), ActorId>,
    pub transaction_id: Option<TransactionId>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MarketAction {
    /// Adds NFT contract addresses that can be listed on marketplace.
    ///
    /// # Requirements:
    /// Only admin can add approved NFT accounts.
    ///
    /// On success replies [`MarketEvent::NftContractAdded`].
    AddNftContract(
        /// the NFT contract address
        ContractId,
    ),

    /// Adds the contract addresses of fungible tokens with which users can pay for NFTs.
    ///
    /// # Requirements:
    /// Only admin can add approved fungible-token accounts.
    ///
    ///
    /// On success replies [`MarketEvent::FtContractAdded`].
    AddFTContract(
        /// the FT contract address
        ContractId,
    ),

    /// Adds data on market item.
    /// If the item of that NFT does not exist on the marketplace then it will be listed.
    /// If the item exists then that action is used to change the price or suspend the sale.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be the NFT owner
    /// * `nft_contract_id` must be in the list of `approved_nft_contracts`
    /// * if item already exists, then it cannot be changed if there is an active auction
    ///
    /// On success replies [`MarketEvent::MarketDataAdded`].
    AddMarketData {
        /// the NFT contract address
        nft_contract_id: ContractId,
        /// the fungible token contract address (If it is `None` then the item is traded for the native value)
        ft_contract_id: Option<ContractId>,
        /// the NFT id
        token_id: TokenId,
        /// the NFT price (if it is `None` then the item is not on the sale)
        price: Option<u128>,
    },

    /// Sells the NFT.
    ///
    /// # Requirements:
    /// * The NFT item must exists and be on sale.
    /// * If the NFT is sold for a native Gear value, then a buyer must attach value equals to the price.
    /// * If the NFT is sold for fungible tokens then a buyer must have enough tokens in the fungible token contract.
    /// * There must be no an opened auction on the item.
    ///
    /// On success replies [`MarketEvent::ItemSold`].
    BuyItem {
        /// NFT contract address
        nft_contract_id: ContractId,
        /// the token ID
        token_id: TokenId,
    },

    /// Creates an auction for selected item.
    /// If the NFT item doesn't exist on the marketplace then it will be listed
    ///
    /// Requirements:
    /// * Only the item owner can start auction.
    /// * `nft_contract_id` must be in the list of `approved_nft_contracts`
    /// *  There must be no active auction.
    ///
    /// On success replies [`MarketEvent::AuctionCreated`].
    CreateAuction {
        /// the NFT contract address
        nft_contract_id: ContractId,
        /// the fungible token contract address (If it is `None` then the item is traded for the native value)
        ft_contract_id: Option<ContractId>,
        /// the NFT id
        token_id: TokenId,
        /// the starting price
        min_price: u128,
        /// the time interval the auction is extended if bid is made if the auction ends before `exec::blocktimestamp() + bid_period`
        bid_period: u64,
        /// the auction duration
        duration: u64,
    },

    /// Adds a bid to an ongoing auction.
    ///
    /// # Requirements:
    /// * The item must extsts.
    /// * The auction must exists on the item.
    /// * If the NFT is sold for a native Gear value, then a buyer must attach value equals to the price indicated in the arguments.
    /// * If the NFT is sold for fungible tokens then a buyer must have   enough tokens in the fungible token contract.
    /// * `price` must be greater then the current offered price for that item.
    ///
    /// On success replies [`MarketEvent::BidAdded`].
    AddBid {
        /// the NFT contract address.
        nft_contract_id: ContractId,
        /// * `token_id`: the NFT id.
        token_id: TokenId,
        /// the offered price.
        price: u128,
    },

    /// Settles the auction.
    ///
    /// Requirements:
    /// * The auction must be over.
    ///
    /// On successful auction replies [`MarketEvent::AuctionSettled`].
    /// If no bids were made replies [`MarketEvent::AuctionCancelled`].
    SettleAuction {
        /// the NFT contract address
        nft_contract_id: ContractId,
        /// the NFT id
        token_id: U256,
    },

    /// Adds a price offer to the item.
    ///
    /// Requirements:
    /// * NFT item must exists and be listed on the marketplace.
    /// * There must be no an ongoing auction on the item.
    /// * If a user makes an offer in native Gear value, then he must attach value equals to the price indicated in the arguments.
    /// * If a user makes an offer in fungible tokens then he must have  enough tokens in the fungible token contract.
    /// * The price can not be equal to 0.
    /// * There must be no identical offers on the item.
    ///
    /// On success replies [`MarketEvent::OfferAdded`].
    AddOffer {
        /// the NFT contract address
        nft_contract_id: ContractId,
        /// the FT contract address (if it is `None, the offer is made for the native value)
        ft_contract_id: Option<ContractId>,
        /// the NFT id
        token_id: TokenId,
        /// the offer price
        price: u128,
    },

    /// Withdraws tokens.
    ///
    /// Requirements:
    /// * NFT item must exists and be listed on the marketplace.
    /// * Only the offer creator can withdraw his tokens.
    /// * The offer with indicated hash must exist.
    ///
    /// On success replies [`MarketEvent::Withdrawn`].
    Withdraw {
        /// the NFT contract address
        nft_contract_id: ContractId,
        /// the NFT id
        token_id: TokenId,
        /// The offered price (native value)
        price: Price,
    },

    /// Accepts an offer.
    ///
    /// Requirements:
    /// * NFT item must exists and be listed on the marketplace.
    /// * Only owner can accept offer.
    /// * There must be no ongoing auction.
    /// * The offer with indicated hash must exist.
    ///
    /// On success replies [`MarketEvent::ItemSold`].
    AcceptOffer {
        /// the NFT contract address
        nft_contract_id: ContractId,
        /// the NFT id
        token_id: TokenId,
        /// the fungible token contract address
        ft_contract_id: Option<ContractId>,
        /// the offer price
        price: Price,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MarketEvent {
    NftContractAdded(ContractId),
    FtContractAdded(ContractId),
    MarketDataAdded {
        nft_contract_id: ContractId,
        owner: ActorId,
        token_id: TokenId,
        price: Option<u128>,
    },
    ItemSold {
        owner: ActorId,
        nft_contract_id: ContractId,
        token_id: TokenId,
    },
    BidAdded {
        nft_contract_id: ContractId,
        token_id: TokenId,
        price: u128,
    },
    AuctionCreated {
        nft_contract_id: ContractId,
        token_id: TokenId,
        price: u128,
    },
    AuctionSettled {
        nft_contract_id: ContractId,
        token_id: TokenId,
        price: u128,
    },
    AuctionCancelled {
        nft_contract_id: ContractId,
        token_id: TokenId,
    },
    NFTListed {
        nft_contract_id: ContractId,
        owner: ActorId,
        token_id: TokenId,
        price: Option<u128>,
    },
    OfferAdded {
        nft_contract_id: ContractId,
        ft_contract_id: Option<ActorId>,
        token_id: TokenId,
        price: u128,
    },
    OfferAccepted {
        nft_contract_id: ContractId,
        token_id: TokenId,
        new_owner: ActorId,
        price: u128,
    },
    Withdraw {
        nft_contract_id: ActorId,
        token_id: TokenId,
        price: u128,
    },
    TransactionFailed,
    RerunTransaction,
    TransferValue,
}
