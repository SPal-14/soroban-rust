# Rental Contract Smart Contract

This is a smart contract written in Rust using the `soroban_sdk` for managing rental agreements.

## Overview

The `RentalContract` manages rental agreements between asset owners and renters. It defines two main structs: `Asset` representing an asset to be rented and `Rental` representing a rental agreement.

## Structs

### Asset

- `owner: Bytes`: The owner of the asset, represented as bytes.
- `asset_id: Symbol`: The identifier for the asset.
- `description:
