use ethereum_types::{H160, U256};
use evm::{
    backend::{MemoryAccount, MemoryBackend, MemoryVicinity},
    executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata},
    Config,
};
use std::collections::BTreeMap;
use structopt::StructOpt;

// ./target/debug/evmbin deposit --from 0000000000000000000000000000000000000001 --value 1
#[derive(Debug, StructOpt, Clone)]
pub struct DepositCmd {
    #[structopt(long = "from")]
    pub from: String,
    #[structopt(long = "value")]
    pub value: String,
}

impl DepositCmd {
    pub fn run(&self) {
        let from = self.from.parse().expect("From should be a valid address");
        let value: u128 = self.value.parse().expect("Value is invalid");

        let vicinity = MemoryVicinity {
            gas_price: U256::zero(),
            origin: H160::zero(),
            chain_id: U256::zero(),
            block_hashes: Vec::new(),
            block_number: U256::zero(),
            block_coinbase: H160::zero(),
            block_timestamp: U256::zero(),
            block_difficulty: U256::zero(),
            block_gas_limit: U256::zero(),
            block_base_fee_per_gas: U256::zero(),
        };
        let config = Config::london();

        let state = BTreeMap::<H160, MemoryAccount>::new();
        let backend = MemoryBackend::new(&vicinity, state);
        let stack_substate_metadata = StackSubstateMetadata::new(10000 as u64, &config);
        let memory_stack_state = MemoryStackState::new(stack_substate_metadata, &backend);
        let mut executor = StackExecutor::new_with_precompiles(memory_stack_state, &config, &());

        executor.state_mut().deposit(from, value.into());
    }
}
