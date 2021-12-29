use crate::DebugApi;

use super::*;

use alloc::{boxed::Box, vec::Vec};
use elrond_wasm::contract_base::CallableContract;
use std::{collections::HashMap, fmt};

pub type ContractCallFactory<A> = Box<dyn Fn(DebugApi) -> Box<dyn CallableContract<A>>>;

pub struct ContractMap<A> {
    contract_objs: HashMap<Vec<u8>, Box<dyn CallableContract<A>>>,
}

impl<A> fmt::Debug for ContractMap<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContractMap").finish()
    }
}

impl<A> ContractMap<A> {
    pub fn new() -> Self {
        ContractMap {
            contract_objs: HashMap::new(),
        }
    }

    pub fn new_contract_instance(
        &self,
        contract_identifier: &[u8],
        _debug_api: DebugApi,
    ) -> Box<dyn CallableContract<A>> {
        if let Some(contract_obj) = self.contract_objs.get(contract_identifier) {
            contract_obj.clone_obj()
        } else {
            unknown_contract_panic(contract_identifier)
        }
    }

    pub fn register_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        new_contract_obj: Box<dyn CallableContract<A>>,
    ) {
        let previous_entry = self.contract_objs.insert(contract_bytes, new_contract_obj);
        assert!(previous_entry.is_none(), "contract inserted twice");
    }

    pub fn contains_contract(&self, contract_bytes: &[u8]) -> bool {
        self.contract_objs.contains_key(contract_bytes)
    }
}

fn unknown_contract_panic(contract_identifier: &[u8]) -> ! {
    if let Ok(s) = std::str::from_utf8(contract_identifier) {
        panic!("Unknown contract: {}", s)
    } else {
        panic!(
            "Unknown contract of length {} bytes",
            contract_identifier.len()
        )
    }
}

impl<A> Default for ContractMap<A> {
    fn default() -> Self {
        Self::new()
    }
}
