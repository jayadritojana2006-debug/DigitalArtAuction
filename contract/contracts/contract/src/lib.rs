#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Auction {
    pub seller: Address,
    pub highest_bid: i128,
    pub highest_bidder: Address,
    pub is_active: bool,
}

#[contract]
pub struct DigitalArtAuction;

#[contractimpl]
impl DigitalArtAuction {

    // Create a new auction
    pub fn create_auction(env: Env, seller: Address) {
        let auction = Auction {
            seller: seller.clone(),
            highest_bid: 0,
            highest_bidder: seller.clone(),
            is_active: true,
        };

        env.storage().instance().set(&Symbol::short("auction"), &auction);
    }

    // Place a bid
    pub fn bid(env: Env, bidder: Address, amount: i128) {
        let key = Symbol::short("auction");
        let mut auction: Auction = env.storage().instance().get(&key).unwrap();

        if !auction.is_active {
            panic!("Auction is not active");
        }

        if amount <= auction.highest_bid {
            panic!("Bid must be higher than current highest bid");
        }

        auction.highest_bid = amount;
        auction.highest_bidder = bidder;

        env.storage().instance().set(&key, &auction);
    }

    // End auction (only seller)
    pub fn end_auction(env: Env, seller: Address) {
        let key = Symbol::short("auction");
        let mut auction: Auction = env.storage().instance().get(&key).unwrap();

        if seller != auction.seller {
            panic!("Only seller can end the auction");
        }

        auction.is_active = false;

        env.storage().instance().set(&key, &auction);
    }

    // Get auction details
    pub fn get_auction(env: Env) -> Auction {
        env.storage()
            .instance()
            .get(&Symbol::short("auction"))
            .unwrap()
    }
}