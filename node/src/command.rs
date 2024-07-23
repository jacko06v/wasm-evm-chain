// This file is part of Astar.

// Copyright (C) Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

//! Astar collator CLI handlers.
use crate::{
    cli::{Cli, Subcommand},
    local::{self, development_config},
};
use sc_cli::{
       Result, SubstrateCli,
};
use sc_service::PartialComponents;

#[cfg(feature = "runtime-benchmarks")]
use frame_benchmarking_cli::{BenchmarkCmd, ExtrinsicFactory, SUBSTRATE_REFERENCE_HARDWARE};

trait IdentifyChain {
    fn is_dev(&self) -> bool;
}

impl IdentifyChain for dyn sc_service::ChainSpec {
    fn is_dev(&self) -> bool {
        self.id().starts_with("dev")
    }
}

impl<T: sc_service::ChainSpec + 'static> IdentifyChain for T {
    fn is_dev(&self) -> bool {
        <dyn sc_service::ChainSpec>::is_dev(self)
    }
}

fn load_spec(id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
    Ok(match id {
        "dev" => Box::new(development_config()),
        &_ => todo!(),
    })
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Astar Collator".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Astar Collator\n\nThe command-line arguments provided first will be \
        passed to the chain node, while the arguments provided after -- will be passed \
        to the relaychain node.\n\n\
        {} [chain-args]",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/AstarNetwork/Astar/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2019
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(id)
    }
}


/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					local::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
        Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = local::new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
        Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = local::new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					local::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		// Some(Subcommand::PurgeChain(cmd)) => {
        //     let runner = cli.create_runner(cmd)?;
		// 	runner.sync_run(|config| cmd.run(config.database))
			
		// },
        Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, backend, .. } =
					local::new_partial(&config)?;
				let aux_revert = Box::new(|client, _, blocks| {
					sc_consensus_grandpa::revert(client, blocks)?;
					Ok(())
				});
				Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
			})
		},
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::Sign(cmd)) => cmd.run(),
        Some(Subcommand::Verify(cmd)) => cmd.run(),
        Some(Subcommand::Vanity(cmd)) => cmd.run(),
        #[cfg(feature = "runtime-benchmarks")]
        Some(Subcommand::Benchmark(cmd)) => {
            use crate::benchmarking::*;
            use sp_keyring::Sr25519Keyring;

            let runner = cli.create_runner(cmd)?;
            let chain_spec = &runner.config().chain_spec;

            match cmd {
                BenchmarkCmd::Pallet(cmd) => {
                    
                        runner.sync_run(|config| {
                            cmd.run::<local_runtime::Block, local::HostFunctions>(config)
                        })
                    
                }
                BenchmarkCmd::Block(cmd) => {
                   
                        runner.sync_run(|config| {
                            let params = local::new_partial(&config)?;
                            cmd.run(params.client)
                        })
                    
                }
                BenchmarkCmd::Storage(cmd) => {
                    
                        runner.sync_run(|config| {
                            let params = local::new_partial(&config)?;
                            let db = params.backend.expose_db();
                            let storage = params.backend.expose_storage();

                            cmd.run(config, params.client, db, storage)
                        })
                    
                }
                BenchmarkCmd::Overhead(cmd) => {
                    
                        runner.sync_run(|config| {
                            let params = local::new_partial(&config)?;
                            let ext_builder = RemarkBuilder::new(params.client.clone());
                            let inherent_data = local_benchmark_inherent_data()
                                .map_err(|e| format!("generating inherent data: {:?}", e))?;

                            cmd.run(
                                config,
                                params.client,
                                inherent_data,
                                Vec::new(),
                                &ext_builder,
                            )
                        })
                    
                }
                BenchmarkCmd::Extrinsic(cmd) => {
                    
                        runner.sync_run(|config| {
                            let params = local::new_partial(&config)?;
                            let remark_builder = RemarkBuilder::new(params.client.clone());
                            let tka_builder = TransferKeepAliveBuilder::new(
                                params.client.clone(),
                                Sr25519Keyring::Alice.to_account_id(),
                                params.client.existential_deposit(),
                            );
                            let ext_factory = ExtrinsicFactory(vec![
                                Box::new(remark_builder),
                                Box::new(tka_builder),
                            ]);
                            let inherent_data = local_benchmark_inherent_data()
                                .map_err(|e| format!("generating inherent data: {:?}", e))?;

                            cmd.run(params.client, inherent_data, Vec::new(), &ext_factory)
                        })
                    
                }
                BenchmarkCmd::Machine(cmd) => {
                    runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()))
                }
            }
        }
        Some(Subcommand::TryRuntime) => Err("The `try-runtime` subcommand has been migrated to a \
        standalone CLI (https://github.com/paritytech/try-runtime-cli). It is no longer \
        being maintained here and will be removed entirely some time after January 2024. \
        Please remove this subcommand from your runtime and use the standalone CLI."
            .into()),
        None => {
            let runner = cli.create_runner(&cli.run)?;
      

            #[cfg(feature = "evm-tracing")]
            let evm_tracing_config = crate::evm_tracing_types::EvmTracingConfig {
                ethapi: cli.eth_api_options.ethapi,
                ethapi_max_permits: cli.eth_api_options.ethapi_max_permits,
                ethapi_trace_max_count: cli.eth_api_options.ethapi_trace_max_count,
                ethapi_trace_cache_duration: cli.eth_api_options.ethapi_trace_cache_duration,
                eth_log_block_cache: cli.eth_api_options.eth_log_block_cache,
                eth_statuses_cache: cli.eth_api_options.eth_statuses_cache,
                max_past_logs: cli.eth_api_options.max_past_logs,
                tracing_raw_max_memory_usage: cli.eth_api_options.tracing_raw_max_memory_usage,
            };

            runner.run_node_until_exit(|config| async move {
               
                    return local::start_node(config, #[cfg(feature = "evm-tracing")] evm_tracing_config).map_err(Into::into);
                
               
            })
        },
        _ => todo!()
    }
    
}

