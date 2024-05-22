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
 
### Vision Statement:

The RentalContract on Stellar Blockchain aims to revolutionize the rental industry by providing a secure, transparent, and efficient platform for managing rental agreements. By leveraging the power of blockchain technology, this project will eliminate disputes, reduce administrative overhead, and foster trust between asset owners and renters. Our vision is to create a seamless rental experience that empowers users, promotes economic growth, and enhances the accessibility of rental services globally. With RentalContract, we strive to set a new standard for digital rental management, making it more reliable and accessible for everyone.

## Software Development Plan for RentalContract on Stellar Blockchain

### Define Structs and Variables

### Develop Asset Struct:
- Variables: owner (Bytes), asset_id (Symbol), description (String).
- Develop Rental Struct:
- Variables: asset (Asset), renter (Bytes), start_time (i64), end_time (i64), rental_fee (u64).

### Implement Core Functions

### new_agreement:
- Parameters: env (Env), asset (Asset), renter (Bytes), start (i64), end (i64), fee (u64).
- Functionality: Create and store a new rental agreement.

### is_active:
- Parameters: env (Env), agreement (Rental).
- Functionality: Check if a rental agreement is currently active.

### Smart Contract Testing

- Write unit tests for each function to ensure correctness.
- Conduct integration testing to verify interactions between functions and structs.

### Front-End Development

### Design User Interface:
- Develop pages for asset listing, rental agreement creation, and agreement status checking.
- Integrate with Stellar Blockchain:
- Connect the front-end to the smart contract using Stellar SDK.
- Enable users to interact with the contract (e.g., create agreements, check status).
  
### Quality Assurance
- Perform end-to-end testing to ensure the entire system works seamlessly.
- Conduct user acceptance testing to gather feedback and make necessary adjustments.

### Deployment
- Deploy the smart contract on the Stellar network.
- Launch the front-end application for public use.
- Monitor the system and provide ongoing maintenance and updates.
