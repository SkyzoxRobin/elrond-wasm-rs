use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/set-repeat");

    blockchain
        .register_contract_builder("file:output/set-repeat.wasm", set_repeat::ContractBuilder);
    blockchain
}

#[test]
fn set_repeat_struct_rs() {
    elrond_wasm_debug::mandos_rs("mandos/set_repeat_struct.scen.json", world());
}

#[test]
fn set_repeat_rs() {
    elrond_wasm_debug::mandos_rs("mandos/set_repeat.scen.json", world());
}
