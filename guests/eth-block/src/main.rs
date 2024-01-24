// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use risc0_zkvm::guest::env;
use zeth_lib::{
    builder::{BlockBuilderStrategy, EthereumStrategy},
    consts::ETH_MAINNET_CHAIN_SPEC,
};
use zeth_lib::output::BlockBuildOutput;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // Read the input previous block and transaction data
    let input = env::read();
    // Build the resulting block
    let mut output = EthereumStrategy::build_from(&ETH_MAINNET_CHAIN_SPEC, input)
        .expect("Failed to build the resulting block");
    // Abridge successful construction results
    if let BlockBuildOutput::SUCCESS { new_block_hash, new_block_head, new_block_state } = &mut output {
        let trie_root = core::mem::replace(new_block_state, new_block_head.state_root.into());
        // Leak memory, save cycles
        core::mem::forget(trie_root);
    }
    // Output the construction result
    env::commit(&output);
    // Leak memory, save cycles
    core::mem::forget(output);
}
