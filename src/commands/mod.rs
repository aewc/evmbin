mod account_cmd;
mod call_message_cmd;
mod create_contract_cmd;
mod deposit_cmd;

use std::collections::BTreeMap;

use account_cmd::AccountCmd;
use call_message_cmd::CallMessageCmd;
use create_contract_cmd::CreateContractCmd;
use deposit_cmd::DepositCmd;
use structopt::StructOpt;

use ethereum_types::{H160, U256};
use evm::backend::MemoryAccount;
use evm::backend::MemoryBackend;
use evm::backend::MemoryVicinity;
use evm::executor::stack::MemoryStackState;
use evm::executor::stack::StackExecutor;
use evm::executor::stack::StackSubstateMetadata;
use evm::Config;

#[derive(Debug, Clone, StructOpt)]
pub enum Subcommand {
    Account(AccountCmd),
    Call(CallMessageCmd),
    Create(CreateContractCmd),
    Deposit(DepositCmd),
}

impl Subcommand {
    pub fn run(&self) {
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
        let gas_limit = 100000;
        let stack_substate_metadata = StackSubstateMetadata::new(gas_limit, &config);
        // let memory_stack_substate = MemoryStackSubstate::new(stack_substate_metadata);
        let memory_stack_state = MemoryStackState::new(stack_substate_metadata, &backend);

        let executor = StackExecutor::new_with_precompiles(memory_stack_state, &config, &());

        match self {
            Subcommand::Account(cmd) => {
                cmd.run(&executor);
            }
            Subcommand::Call(cmd) => {
                cmd.run();
            }
            Subcommand::Create(cmd) => {
                cmd.run();
            }
            Subcommand::Deposit(cmd) => {
                cmd.run();
            }
        }
    }
}
