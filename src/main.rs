use soroban_sdk::{contractimpl, Env, Bytes, Symbol};

struct Asset {
    owner: Bytes,
    asset_id: Symbol,
    description: String,
}

struct Rental {
    asset: Asset,
    renter: Bytes,
    start_time: i64,
    end_time: i64,
    rental_fee: u64,
}

pub struct RentalContract;

#[contractimpl]
impl RentalContract {
    
    pub fn new_agreement(env: Env, asset: Asset, renter: Bytes, start: i64, end: i64, fee: u64) -> RentalAgreement {
        
        Rental { asset, renter, start_time: start, end_time: end, rental_fee: fee }
    }

 
    pub fn is_active(env: Env, agreement: Rental) -> bool {
        let current_time = env.ledger().timestamp();
        agreement.start_time <= current_time && agreement.end_time >= current_time
    }


}
