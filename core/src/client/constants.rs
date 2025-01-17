pub const CHAIN_ID: u64 = 1_263_227_476;

pub const STARKNET_NATIVE_TOKEN: &str =
    "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";

pub mod selectors {
    use starknet::{core::types::FieldElement, macros::selector};

    pub const GET_STARKNET_CONTRACT_ADDRESS: FieldElement =
        selector!("get_starknet_contract_address");
    pub const BYTECODE: FieldElement = selector!("bytecode");

    pub const EXECUTE_AT_ADDRESS: FieldElement = selector!("execute_at_address");
    pub const COMPUTE_STARKNET_ADDRESS: FieldElement = selector!("compute_starknet_address");

    pub const GET_EVM_ADDRESS: FieldElement = selector!("get_evm_address");

    pub const BALANCE_OF: FieldElement = selector!("balanceOf");
}
