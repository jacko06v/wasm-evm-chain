//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
pub mod weights;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;
    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    ///
    /// All our types and constants a pallet depends on must be declared here.
    /// These types are defined generically and made concrete when the pallet is declared in the
    /// `runtime/src/lib.rs` file of your chain.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
    }

    /// In this template, we are declaring a storage item called `AiAgentsExecutions` that stores the couple nft_id (u32) and the input_uri (Vec<u32>).
    #[pallet::storage]
    pub type AiAgentsExecutions<T: Config> = StorageValue<_, (u32, Vec<u8>), ValueQuery>;

    /// Events that functions in this pallet can emit.
    ///
    /// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
    /// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
    /// documentation for each event field and its parameters is added to a node's metadata so it
    /// can be used by external interfaces or tools.
    ///
    ///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
    /// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
    /// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A user has successfully set a new value.
        AiAgentExecutionRequested {
            /// The nft_id.
            nft_id: u32,
            /// The input_uri.
            input_uri: Vec<u8>,
            /// The account who set the new value.
            who: T::AccountId,
        },
    }

    /// Errors that can be returned by this pallet.
    ///
    /// Errors tell users that something went wrong so it's important that their naming is
    /// informative. Similar to events, error documentation is added to a node's metadata so it's
    /// equally important that they have helpful documentation associated with them.
    ///
    /// This type of runtime error can be up to 4 bytes in size should you want to return additional
    /// information.
    #[pallet::error]
    pub enum Error<T> {
        /// The value retrieved was `None` as no value was previously set.
        NoneValue,
        /// There was an attempt to increment the value in storage over `u32::MAX`.
        StorageOverflow,
    }

    /// The pallet's dispatchable functions ([`Call`]s).
    ///
    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// They must always return a `DispatchResult` and be annotated with a weight and call index.
    ///
    /// The [`call_index`] macro is used to explicitly
    /// define an index for calls in the [`Call`] enum. This is useful for pallets that may
    /// introduce new dispatchables over time. If the order of a dispatchable changes, its index
    /// will also change which will break backwards compatibility.
    ///
    /// The [`weight`] macro is used to assign a weight to each call.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::run())]
        pub fn run(origin: OriginFor<T>, nft_id: u32, input_uri: Vec<u8>) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;

            // Be sure that nft_id is not 0.
            ensure!(nft_id != 0, Error::<T>::NoneValue);

            // Be sure that input_uri is a valid URI.
            ensure!(!input_uri.is_empty(), Error::<T>::NoneValue);

            // Add the nft_id and input_uri to the storage.
            AiAgentsExecutions::<T>::put((nft_id, input_uri.clone()));

            // Emit an event.
            Self::deposit_event(Event::AiAgentExecutionRequested {
                nft_id,
                input_uri,
                who,
            });

            // Return a successful `DispatchResult`
            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: BlockNumberFor<T>) {
            log::info!(
                "PALLET UOMI ENGINE offchain_worker | Block number is {:?}",
                block_number
            );

            let (nft_id, input_uri) = AiAgentsExecutions::<T>::get();

            // we check nft_id is not 0 to be sure there is an execution to be done
            if nft_id == 0 {
                log::info!("PALLET UOMI ENGINE offchain_worker | No execution to be done");
                return;
            }

            // we check input_uri is not empty to be sure there is an execution to be done
            if input_uri.is_empty() {
                log::info!("PALLET UOMI ENGINE offchain_worker | No execution to be done");
                return;
            }

            let nft_wasm = match Self::download_wasm_from_nft_id(nft_id) {
                Ok(wasm) => wasm,
                Err(e) => {
                    log::error!(
                        "PALLET UOMI ENGINE offchain_worker | Error downloading wasm: {:?}",
                        e
                    );
                    return;
                }
            };

            let input = match Self::download_input_from_input_uri(input_uri) {
                Ok(input) => input,
                Err(e) => {
                    log::error!(
                        "PALLET UOMI ENGINE offchain_worker | Error downloading input: {:?}",
                        e
                    );
                    return;
                }
            };

            let _output = match Self::execute_wasm(nft_wasm, input) {
                Ok(output) => output,
                Err(e) => {
                    log::error!(
                        "PALLET UOMI ENGINE offchain_worker | Error executing wasm: {:?}",
                        e
                    );
                    return;
                }
            };

            // TODO: Here we should store the output in the storage and clean the nft_id and input_uri from the AiAgentsExecutions
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn download_wasm_from_nft_id(
            _nft_id: u32,
        ) -> Result<Vec<u8>, sp_runtime::offchain::http::Error> {
            // TODO: Here we should download the metadata from the NFT ID and get the wasm URI

            // BACKUP EXAMPLE TO DOWNLOAD FROM A URL
            //
            // let deadline = sp_io::offchain::timestamp().add(sp_runtime::offchain::Duration::from_millis(5_000));
            // let request = sp_runtime::offchain::http::Request::get("https://storage.gregoriogalante.com/uomi_example_agent3.wasm");
            // let pending = request.deadline(deadline).send().map_err(|_| sp_runtime::offchain::http::Error::IoError)?;
            // let response = pending.try_wait(deadline).map_err(|_| sp_runtime::offchain::http::Error::DeadlineReached)??;
            // if response.code != 200 {
            // 	log::error!("PALLET UOMI ENGINE download_wasm_from_nft_id | Error downloading wasm: {:?}", response.code);
            // 	return Err(sp_runtime::offchain::http::Error::Unknown);
            // }
            // wasm = response.body().collect::<Vec<u8>>();
            // log::info!("PALLET UOMI ENGINE download_wasm_from_nft_id | Downloaded wasm of length: {:?}", wasm.len());
            //

            let wasm = include_bytes!("./test.wasm").to_vec();
            Ok(wasm)
        }

        pub fn download_input_from_input_uri(
            input_uri: Vec<u8>,
        ) -> Result<Vec<u8>, sp_runtime::offchain::http::Error> {
            let input_uri_str = sp_std::str::from_utf8(&input_uri)
                .map_err(|_| sp_runtime::offchain::http::Error::Unknown)?;
            log::info!(
                "PALLET UOMI ENGINE download_input_from_input_uri | Downloading input from: {:?}",
                input_uri_str
            );
            let deadline = sp_io::offchain::timestamp()
                .add(sp_runtime::offchain::Duration::from_millis(5_000));
            let request = sp_runtime::offchain::http::Request::get(input_uri_str);
            let pending = request
                .deadline(deadline)
                .send()
                .map_err(|_| sp_runtime::offchain::http::Error::IoError)?;
            let response = pending
                .try_wait(deadline)
                .map_err(|_| sp_runtime::offchain::http::Error::DeadlineReached)??;
            log::info!(
                "PALLET UOMI ENGINE download_input_from_input_uri | Response code is: {:?}",
                response.code
            );
            if response.code != 200 {
                log::error!("PALLET UOMI ENGINE download_input_from_input_uri | Error downloading input: {:?}", response.code);
                return Err(sp_runtime::offchain::http::Error::Unknown);
            }

            let input = response.body().collect::<Vec<u8>>();
            log::info!("PALLET UOMI ENGINE download_input_from_input_uri | Downloaded input of length: {:?}", input.len());
            Ok(input)
        }

        pub fn execute_wasm(wasm: Vec<u8>, input: Vec<u8>) -> Result<Vec<u8>, wasmi::Error> {
            let engine = wasmi::Engine::default();
            let module = wasmi::Module::new(&engine, &wasm[..])?;

            type HostState = Vec<u8>;
            let mut store = wasmi::Store::new(&engine, input);

            let host_set_output = wasmi::Func::wrap(
                &mut store,
                |mut caller: wasmi::Caller<'_, HostState>, ptr: i32, len: i32| {
                    log::info!(
                        "Host function set_output called with ptr: {:?} and len: {:?}",
                        ptr,
                        len
                    );

                    // Get a reference to the memory
                    let memory = caller
                        .get_export("memory")
                        .and_then(wasmi::Extern::into_memory)
                        .expect("Failed to get memory export");

                    // Read the data from memory
                    let mut buffer = sp_std::vec![0u8; len as usize];
                    memory
                        .read(&caller, ptr as usize, &mut buffer)
                        .expect("Failed to read memory");

                    log::info!("Set output data from memory: {:?}", buffer);
                    *caller.data_mut() = buffer;
                },
            );

            let get_input = wasmi::Func::wrap(
                &mut store,
                |mut caller: wasmi::Caller<'_, HostState>, ptr: i32, _len: i32| {
                    let input = caller.data().clone(); // Clone the data to avoid immutable borrow conflict
                    let memory = caller
                        .get_export("memory")
                        .and_then(wasmi::Extern::into_memory)
                        .expect("Failed to get memory export");

                    memory
                        .write(&mut caller, ptr as usize, &input)
                        .expect("Failed to write memory");

                    log::info!("Input data written to memory: {:?}", input);
                },
            );

            let mut linker = wasmi::Linker::new(&engine);

            linker.define("env", "set_output", host_set_output)?;
            linker.define("env", "get_input", get_input)?;

            let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
            let wasm_function = instance.get_typed_func::<(), ()>(&store, "wasm_function")?;

            wasm_function.call(&mut store, ())?;

            Ok(store.into_data())
        }
    }
}
