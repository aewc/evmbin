use evm::{
    backend::{Backend, MemoryBackend},
    executor::stack::{MemoryStackState, StackExecutor},
};
use structopt::StructOpt;

// ./target/debug/evmbin account --from 0000000000000000000000000000000000000001
#[derive(Debug, StructOpt, Clone)]
pub struct AccountCmd {
    #[structopt(long = "from")]
    pub from: String,
}

impl AccountCmd {
    pub fn run(&self, executor: &StackExecutor<MemoryStackState<MemoryBackend>, ()>) {
        let from = self.from.parse().expect("From should be a valid address");
        let account = executor.state().basic(from);
        println!("{:?}", account);
    }
}
