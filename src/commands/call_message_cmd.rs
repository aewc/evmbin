use ethereum_types::{H160, U256};
use evm::{
    backend::{ApplyBackend, MemoryAccount, MemoryBackend, MemoryVicinity},
    executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata},
    Config,
};
use hex;
use std::collections::BTreeMap;
use structopt::StructOpt;

// ./target/debug/evmbin call --from 0000000000000000000000000000000000000001 --to 0000000000000000000000000000000000000002 --value 0 --gas_limit 100000 --gas_price 0 --input 6000
#[derive(Debug, StructOpt, Clone)]
pub struct CallMessageCmd {
    #[structopt(long = "from")]
    pub from: String,
    #[structopt(long = "to")]
    pub to: String,
    #[structopt(long = "value")]
    pub value: String,
    #[structopt(long = "gas_limit")]
    pub gas_limit: u32,
    #[structopt(long = "gas_price")]
    pub gas_price: U256,
    #[structopt(long = "input")]
    pub input: String,
}

impl CallMessageCmd {
    pub fn run(&self) {
        let from: H160 = self.from.parse().expect("From should be a valid address");
        let to = self.to.parse().expect("To should be a valid address");
        let value: U256 = self.value.parse().expect("Value is invalid");
        let gas_limit = self.gas_limit;
        let gas_price = self.gas_price;
        let input = hex::decode(self.input.as_str()).expect("Input is invalid");

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
        let mut backend = MemoryBackend::new(&vicinity, state);
        let stack_substate_metadata = StackSubstateMetadata::new(gas_limit as u64, &config);
        let memory_stack_state = MemoryStackState::new(stack_substate_metadata, &backend);
        let mut executor = StackExecutor::new_with_precompiles(memory_stack_state, &config, &());

        let nonce = Some(executor.nonce(from.clone()));

        let total_fee = gas_price.checked_mul(U256::from(gas_limit)).unwrap();
        let total_payment = value.checked_add(total_fee).unwrap();
        let state_account = executor.state_mut().account_mut(from.clone());
        let source_account = state_account.basic.clone();
        assert!(source_account.balance >= total_payment);
        executor
            .state_mut()
            .withdraw(from.clone(), total_fee)
            .unwrap();

        if let Some(nonce) = nonce {
            assert!(source_account.nonce == nonce);
        }

        let retv = executor.transact_call(from, to, value, input, gas_limit as u64, vec![]);
        println!("Call message successful{:?}", retv);

        let actual_fee = executor.fee(gas_price);
        executor
            .state_mut()
            .deposit(from, total_fee.saturating_sub(actual_fee));

        let (values, logs) = executor.into_state().deconstruct();
        backend.apply(values, logs, true);

        println!("{:?}", &backend);

        println!("Call message successful");
    }
}
