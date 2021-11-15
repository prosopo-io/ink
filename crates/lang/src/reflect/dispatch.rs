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

use core::fmt::Display;

/// Reflects the number of dispatchable ink! messages and constructors respectively.
///
/// # Note
///
/// - This is automatically implemented by all ink! smart contracts.
/// - All ink! constructors and ink! messages of an ink! smart contract are dispatchables.  
///   This explicitly includes ink! messages from ink! trait implementations.
///
/// # Usage
///
/// ```
/// use ink_lang as ink;
/// # use ink_lang::reflect::ContractAmountDispatchables;
///
/// #[ink::contract]
/// pub mod contract {
///     #[ink(storage)]
///     pub struct Contract {}
///
///     impl Contract {
///         #[ink(constructor)]
///         pub fn constructor1() -> Self { Contract {} }
///
///         #[ink(constructor)]
///         pub fn constructor2() -> Self { Contract {} }
///
///         #[ink(message)]
///         pub fn message1(&self) {}
///
///         #[ink(message)]
///         pub fn message2(&self) {}
///
///         #[ink(message)]
///         pub fn message3(&self) {}
///     }
/// }
///
/// use contract::Contract;
///
/// fn main() {
///     assert_eq!(<Contract as ContractAmountDispatchables>::CONSTRUCTORS, 2);
///     assert_eq!(<Contract as ContractAmountDispatchables>::MESSAGES, 3);
/// }
/// ```
pub trait ContractAmountDispatchables {
    /// The number of dispatchable ink! messages.
    const MESSAGES: usize;
    /// The number of dispatchable ink! constructors.
    const CONSTRUCTORS: usize;
}

/// Reflects the sequence of all dispatchable ink! messages of the ink! smart contract.
///
/// # Note
///
/// This is automatically implemented by all ink! smart contracts.
///
/// # Usage
///
/// ```
/// use ink_lang as ink;
/// # use ink_lang::reflect::ContractAmountDispatchables;
/// # use ink_lang::reflect::ContractDispatchableMessages;
/// # use ink_lang::selector_id;
///
/// #[ink::contract]
/// pub mod contract {
///     #[ink(storage)]
///     pub struct Contract {}
///
///     impl Contract {
///         #[ink(constructor)]
///         pub fn constructor1() -> Self { Contract {} }
///
///         #[ink(message, selector = 1234)]
///         pub fn message1(&self) {}
///
///         #[ink(message, selector = 0xC0DECAFE)]
///         pub fn message2(&self) {}
///
///         #[ink(message)]
///         pub fn message3(&self) {}
///     }
/// }
///
/// use contract::Contract;
///
/// fn main() {
///     assert_eq!(
///         <Contract as ContractDispatchableMessages<{
///             <Contract as ContractAmountDispatchables>::MESSAGES
///         }>>::IDS,
///         [1234, 0xC0DECAFE, selector_id!("message3")],
///     );
/// }
/// ```
pub trait ContractDispatchableMessages<const AMOUNT: usize> {
    /// The sequence stores selector IDs of all ink! messages dispatchable by the ink! smart contract.
    const IDS: [u32; AMOUNT];
}

/// Reflects the sequence of all dispatchable ink! constructors of the ink! smart contract.
///
/// # Note
///
/// This is automatically implemented by all ink! smart contracts.
///
/// # Usage
///
/// ```
/// use ink_lang as ink;
/// # use ink_lang::reflect::ContractAmountDispatchables;
/// # use ink_lang::reflect::ContractDispatchableConstructors;
/// # use ink_lang::selector_id;
///
/// #[ink::contract]
/// pub mod contract {
///     #[ink(storage)]
///     pub struct Contract {}
///
///     impl Contract {
///         #[ink(constructor, selector = 1234)]
///         pub fn constructor1() -> Self { Contract {} }
///
///         #[ink(constructor, selector = 0xC0DECAFE)]
///         pub fn constructor2() -> Self { Contract {} }
///
///         #[ink(constructor)]
///         pub fn constructor3() -> Self { Contract {} }
///
///         #[ink(message)]
///         pub fn message1(&self) {}
///     }
/// }
///
/// use contract::Contract;
///
/// fn main() {
///     assert_eq!(
///         <Contract as ContractDispatchableConstructors<{
///             <Contract as ContractAmountDispatchables>::CONSTRUCTORS
///         }>>::IDS,
///         [1234, 0xC0DECAFE, selector_id!("constructor3")],
///     );
/// }
/// ```
pub trait ContractDispatchableConstructors<const AMOUNT: usize> {
    /// The sequence stores selector IDs of all ink! constructors dispatchable by the ink! smart contract.
    const IDS: [u32; AMOUNT];
}

/// Stores various information of the respective dispatchable ink! message.
///
/// # Note
///
/// This trait is implemented by ink! for every dispatchable ink! message
/// of the root ink! smart contract. The `ID` used in the trait reflects the
/// chosen or derived selector of the dispatchable ink! message.
///
/// # Usage
///
/// ```
/// use ink_lang as ink;
/// # use ink_lang::reflect::DispatchableMessageInfo;
/// # use ink_lang::{selector_id, selector_bytes};
///
/// #[ink::contract]
/// pub mod contract {
///     #[ink(storage)]
///     pub struct Contract {}
///
///     impl Contract {
///         #[ink(constructor)]
///         pub fn constructor() -> Self { Contract {} }
///
///         #[ink(message)]
///         pub fn message1(&self) {}
///
///         #[ink(message, payable, selector = 0xC0DECAFE)]
///         pub fn message2(&mut self, input1: i32, input2: i64) -> (bool, i32) {
///             unimplemented!()
///         }
///     }
/// }
///
/// use contract::Contract;
///
/// /// Asserts that the message with the selector `ID` has the following properties.
/// ///
/// /// # Note
/// ///
/// /// The `In` and `Out` generic parameters describe the input and output types.
/// fn assert_message_info<In, Out, const ID: u32>(
///     mutates: bool,
///     payable: bool,
///     selector: [u8; 4],
///     label: &str,
/// )
/// where
///     Contract: DispatchableMessageInfo<{ID}, Input = In, Output = Out>,
/// {
///     assert_eq!(<Contract as DispatchableMessageInfo<{ID}>>::MUTATES, mutates);
///     assert_eq!(<Contract as DispatchableMessageInfo<{ID}>>::PAYABLE, payable);
///     assert_eq!(
///         <Contract as DispatchableMessageInfo<{ID}>>::SELECTOR,
///         selector,
///     );
///     assert_eq!(
///         <Contract as DispatchableMessageInfo<{ID}>>::LABEL,
///         label,
///     );
/// }
///
/// fn main() {
///     assert_message_info::<(), (), {selector_id!("message1")}>(
///         false, false, selector_bytes!("message1"), "message1"
///     );
///     assert_message_info::<(i32, i64), (bool, i32), 0xC0DECAFE_u32>(
///         true, true, [0xC0, 0xDE, 0xCA, 0xFE], "message2"
///     );
/// }
/// ```
pub trait DispatchableMessageInfo<const ID: u32> {
    /// Reflects the input types of the dispatchable ink! message.
    type Input;
    /// Reflects the output type of the dispatchable ink! message.
    type Output;
    /// The ink! storage struct type.
    type Storage;

    /// The closure that can be used to dispatch into the dispatchable ink! message.
    ///
    /// # Note
    ///
    /// We unify `&self` and `&mut self` ink! messages here and always take a `&mut self`.
    /// This is mainly done for simplification but also because we can easily convert from
    /// `&mut self` to `&self` with our current dispatch codegen architecture.
    const CALLABLE: fn(
        &mut Self::Storage,
        &[u8],
    ) -> ::core::result::Result<Self::Output, DispatchError>;

    /// Yields `true` if the dispatchable ink! message mutates the ink! storage.
    const MUTATES: bool;
    /// Yields `true` if the dispatchable ink! message is payable.
    const PAYABLE: bool;
    /// The selectors of the dispatchable ink! message.
    const SELECTOR: [u8; 4];
    /// The label of the dispatchable ink! message.
    const LABEL: &'static str;
}

/// Stores various information of the respective dispatchable ink! constructor.
///
/// # Note
///
/// This trait is implemented by ink! for every dispatchable ink! constructor
/// of the root ink! smart contract. The `ID` used in the trait reflects the
/// chosen or derived selector of the dispatchable ink! constructor.
///
/// # Usage
///
/// ```
/// use ink_lang as ink;
/// # use ink_lang::reflect::DispatchableConstructorInfo;
/// # use ink_lang::{selector_id, selector_bytes};
///
/// #[ink::contract]
/// pub mod contract {
///     #[ink(storage)]
///     pub struct Contract {}
///
///     impl Contract {
///         #[ink(constructor)]
///         pub fn constructor1() -> Self { Contract {} }
///
///         #[ink(constructor, selector = 0xC0DECAFE)]
///         pub fn constructor2(input1: i32, input2: i64) -> Self {
///             Contract {}
///         }
///
///         #[ink(message)]
///         pub fn message(&self) {}
///     }
/// }
///
/// use contract::Contract;
///
/// /// Asserts that the constructor with the selector `ID` has the following properties.
/// ///
/// /// # Note
/// ///
/// /// The `In` and `Out` generic parameters describe the input and output types.
/// fn assert_constructor_info<In, const ID: u32>(
///     selector: [u8; 4],
///     label: &str,
/// )
/// where
///     Contract: DispatchableConstructorInfo<{ID}, Input = In>,
/// {
///     assert_eq!(
///         <Contract as DispatchableConstructorInfo<{ID}>>::SELECTOR,
///         selector,
///     );
///     assert_eq!(
///         <Contract as DispatchableConstructorInfo<{ID}>>::LABEL,
///         label,
///     );
/// }
///
/// fn main() {
///     assert_constructor_info::<(), {selector_id!("constructor1")}>(
///         selector_bytes!("constructor1"), "constructor1"
///     );
///     assert_constructor_info::<(i32, i64), 0xC0DECAFE_u32>(
///         [0xC0, 0xDE, 0xCA, 0xFE], "constructor2"
///     );
/// }
/// ```
pub trait DispatchableConstructorInfo<const ID: u32> {
    /// Reflects the input types of the dispatchable ink! constructor.
    type Input;
    /// The ink! storage struct type.
    type Storage;

    /// The closure that can be used to dispatch into the dispatchable ink! constructor.
    const CALLABLE: fn(&[u8]) -> ::core::result::Result<Self::Storage, DispatchError>;

    /// The selectors of the dispatchable ink! constructor.
    const SELECTOR: [u8; 4];
    /// The label of the dispatchable ink! constructor.
    const LABEL: &'static str;
}

pub trait ContractMessageExecutor {
    /// The ink! smart contract message executor type.
    type Type: ExecuteDispatchable;
}

pub trait ContractConstructorExecutor {
    /// The ink! smart contract constructor executor type.
    type Type: ExecuteDispatchable;
}

/// Starts the execution of the respective ink! message or constructor call.
///
/// # Note
///
/// Implemented by the ink! smart contract message or constructor decoder.
pub trait ExecuteDispatchable {
    /// Executes the ink! smart contract message or constructor.
    fn execute_dispatchable() -> Result<(), DispatchError>;
}

/// An error that can occur during dispatch of ink! dispatchables.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DispatchError {
    /// Failed to decode into a valid dispatch selector.
    InvalidSelector,
    /// The decoded selector is not known to the dispatch decoder.
    UnknownSelector,
    /// Failed to decode the parameters for the selected dispatchable.
    InvalidParameters,
    /// Failed to read execution input for the dispatchable.
    CouldNotReadInput,
    /// Invalidly paid an unpayable dispatchable.
    PaidUnpayableMessage,
}

impl Display for DispatchError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl DispatchError {
    /// Returns a string representation of the error.
    #[inline]
    fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidSelector => "unable to decode selector",
            Self::UnknownSelector => "encountered unknown selector",
            Self::InvalidParameters => "unable to decode input",
            Self::CouldNotReadInput => "could not read input",
            Self::PaidUnpayableMessage => "paid an unpayable message",
        }
    }
}

impl From<DispatchError> for scale::Error {
    #[inline]
    fn from(error: DispatchError) -> Self {
        Self::from(error.as_str())
    }
}
