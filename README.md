# Rental Contract Smart Contract

This is a smart contract written in Rust using the `soroban_sdk` for managing rental agreements.

## Overview

The `RentalContract` manages rental agreements between asset owners and renters. It defines two main structs: `Asset` representing an asset to be rented and `Rental` representing a rental agreement.

## Structs

### Asset

- `owner: Bytes`: The owner of the asset, represented as bytes.
- `asset_id: Symbol`: The identifier for the asset.
- `description:

String`: A description of the asset.

### Rental

- `asset: Asset`: The asset being rented.
- `renter: Bytes`: The renter's identifier, represented as bytes.
- `start_time: i64`: The start time of the rental agreement.
- `end_time: i64`: The end time of the rental agreement.
- `rental_fee: u64`: The fee charged for renting the asset.

### RentalContract

This struct implements methods for managing rental agreements.

#### `new_agreement`

Creates a new rental agreement.

- Parameters:
  - `env:
Env`: The environment for the contract.
  - `asset: Asset`: The asset being rented.
  - `renter: Bytes`: The renter's identifier.
  - `start: i64`: The start time of the rental.
  - `end: i64`: The end time of the rental.
  - `fee: u64`: The rental fee.

#### `is_active`

Checks if a rental agreement is currently active.

- Parameters:
  - `env: Env`: The environment for the contract.
  - `agreement: Rental`: The rental agreement to check.
 
###Vision Statement:

The RentalContract on Stellar Blockchain aims to revolutionize the rental industry by providing a secure, transparent, and efficient platform for managing rental agreements. By leveraging the power of blockchain technology, this project will eliminate disputes, reduce administrative overhead, and foster trust between asset owners and renters. Our vision is to create a seamless rental experience that empowers users, promotes economic growth, and enhances the accessibility of rental services globally. With RentalContract, we strive to set a new standard for digital rental management, making it more reliable and accessible for everyone.
