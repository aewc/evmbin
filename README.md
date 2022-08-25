# EVMBIN
Run the EVM interactively

```sh
cargo build --release

./target/release/evmbin account --from 0000000000000000000000000000000000000001
Basic { balance: 0, nonce: 0 }


./target/release/evmbin call --from 0000000000000000000000000000000000000001 --to 0000000000000000000000000000000000000002 --value 0 --gas_limit 100000 --gas_price 0 --input 6000
Call message successful(Succeed(Stopped), [])
MemoryBackend { vicinity: MemoryVicinity { gas_price: 0, origin: 0x0000000000000000000000000000000000000000, chain_id: 0, block_hashes: [], block_number: 0, block_coinbase: 0x0000000000000000000000000000000000000000, block_timestamp: 0, block_difficulty: 0, block_gas_limit: 0, block_base_fee_per_gas: 0 }, state: {0x0000000000000000000000000000000000000001: MemoryAccount { nonce: 1, balance: 0, storage: {}, code: [] }}, logs: [] }
Call message successful

./target/release/evmbin create --from 0000000000000000000000000000000000000001 --value 0 --gas_limit 100000 --gas_price 0 --code 6000

