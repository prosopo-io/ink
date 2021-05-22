// Copyright 2018-2021 Parity Technologies (UK) Ltd.
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

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod my_generic_contract {
    use ink_lang as ink;
    #[ink(storage)]
    pub struct MyContract {}

    impl MyContract {
        /// Creates a new `MyContract` instance.
        #[ink(constructor)]
        pub fn new() -> Self {
            MyContract {}
        }

        /// Emits a `MyEvent`.
        #[ink(message)]
        pub fn emit_my_event(&self) {}
    }

    pub trait Bar: Default {
        fn hello_world() -> &'static str {
            "Hello world"
        }
    }

    #[ink::trait_definition]
    pub trait ITrait {
        #[ink(constructor)]
        fn new() -> Self;

        #[ink(message)]
        fn foo(&self);
    }

    #[ink(impl)]
    impl<T: Bar> ITrait for T {
        #[ink(constructor)]
        fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        fn foo(&self) {
            println!("{}", Self::hello_world())
        }
    }
}
