use crate::{mock::*, AiAgentsExecutions, Error, Event};

use env_logger::Builder;
use frame_support::{
    assert_err, assert_noop, assert_ok, sp_runtime::DispatchError::BadOrigin,
    traits::OffchainWorker,
};
use log::LevelFilter;
use sp_core::offchain::{testing, OffchainWorkerExt};
use std::{
    io::Write,
    panic,
    sync::{Arc, Mutex},
};

// RUN TESTS

#[test]
fn test_run_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);
        // Dispatch a signed extrinsic.
        let nft_id = 1;
        let input_uri = "https://storage.gregoriogalante.com/uomi_example_input.txt".as_bytes().to_vec();
        let who = 1;

        assert_ok!(TemplateModule::run(
            RuntimeOrigin::signed(who),
            nft_id,
            input_uri.clone()
        ));
        // Read pallet storage and assert an expected result
        assert_eq!(
            AiAgentsExecutions::<Test>::get(),
            (nft_id, input_uri.clone())
        );
        // Assert that the correct event was deposited
        System::assert_last_event(RuntimeEvent::TemplateModule(
            Event::AiAgentExecutionRequested {
                nft_id,
                input_uri,
                who,
            },
        ));
    });
}

#[test]
fn test_run_fails_with_unsigned_origin() {
    new_test_ext().execute_with(|| {
        let nft_id = 1;
        let input_uri = b"https://storage.gregoriogalante.com/uomi_example_input.txt".to_vec();

        // Сall the `run` function with an unsigned origin
        assert_err!(
            TemplateModule::run(RuntimeOrigin::none(), nft_id, input_uri.clone()),
            BadOrigin
        );

        // Ensure storage is still empty
        assert_eq!(AiAgentsExecutions::<Test>::get(), (0, Vec::new()));
    });
}

#[test]
fn test_run_fails_if_nft_id_is_zero() {
    new_test_ext().execute_with(|| {
        let nft_id = 0;
        let input_uri = b"https://storage.gregoriogalante.com/uomi_example_input.txt".to_vec();
        let who = 1;

        assert_noop!(
            TemplateModule::run(RuntimeOrigin::signed(who), nft_id, input_uri),
            Error::<Test>::NoneValue
        );
    });
}

#[test]
fn test_run_fails_if_input_uri_is_empty() {
    new_test_ext().execute_with(|| {
        let nft_id = 1;
        let input_uri = Vec::new();
        let who = 1;

        assert_noop!(
            TemplateModule::run(RuntimeOrigin::signed(who), nft_id, input_uri),
            Error::<Test>::NoneValue
        );
    });
}

// OFFCHAIN WORKER TESTS

#[test]
fn test_offchain_worker_success_execution() {
    // Initialize the logger
    let log_counter = Arc::new(Mutex::new(0_u16));
    let log_counter_ref = Arc::clone(&log_counter);

    Builder::new()
        .filter_level(LevelFilter::Info)
        .format(move |buf, record| {
            {
                let mut counter = log_counter_ref.lock().unwrap();
                *counter += 1;
            }
            writeln!(buf, "{} - {}", record.level(), record.args())
        })
        .is_test(true)
        .try_init()
        .ok();

    new_test_ext().execute_with(|| {
        // Insert a valid execution request
        AiAgentsExecutions::<Test>::put((0, Vec::<u8>::new()));

        // Execute the offchain worker
        TemplateModule::offchain_worker(0);

        // Ensure the execution request is removed from storage
        assert_eq!(AiAgentsExecutions::<Test>::get(), (0, Vec::new()));
    });

    // Check the log count
    let log_count = *log_counter.lock().unwrap();
    println!("LOGS count: {}", log_count);
}

#[test]
fn test_offchain_worker_no_execution_requested() {
    // Initialize the logger
    let log_counter = Arc::new(Mutex::new(0_u16));
    let log_counter_ref = Arc::clone(&log_counter);

    Builder::new()
        .filter_level(LevelFilter::Info)
        .format(move |buf, record| {
            {
                let mut counter = log_counter_ref.lock().unwrap();
                *counter += 1;
            }
            writeln!(buf, "{} - {}", record.level(), record.args())
        })
        .is_test(true)
        .try_init()
        .ok();

    new_test_ext().execute_with(|| {
        AiAgentsExecutions::<Test>::put((0, Vec::<u8>::new()));
        TemplateModule::offchain_worker(1);
    });

    // Check the log count
    let log_count = *log_counter.lock().unwrap();
    println!("LOGS count: {}", log_count);
}

// DOWNLOAD TESTS

// #[test]
// fn test_download_wasm_from_nft_id_success_execution() {
//     let mut t = new_test_ext();

//     // Setup the offchain worker environment
//     let (offchain, state) = testing::TestOffchainExt::new();
//     t.register_extension(OffchainWorkerExt::new(offchain.clone()));

//     // Initialize the logger
//     let log_counter = Arc::new(Mutex::new(0_u16));
//     let log_counter_ref = Arc::clone(&log_counter);

//     Builder::new()
//         .filter_level(LevelFilter::Info)
//         .format(move |buf, record| {
//             {
//                 let mut counter = log_counter_ref.lock().unwrap();
//                 *counter += 1;
//             }
//             writeln!(buf, "{} - {}", record.level(), record.args())
//         })
//         .is_test(true)
//         .try_init()
//         .ok();

//     // Mock a successful HTTP response
//     state.write().expect_request(testing::PendingRequest {
//         method: "GET".into(),
//         uri: "https://storage.gregoriogalante.com/uomi_example_agent2.wasm".into(),
//         meta: vec![],
//         headers: vec![],
//         body: vec![],
//         response_headers: vec![],
//         response: Some(vec![]),
//         sent: true,
//         ..Default::default()
//     });

//     // Execute the test with the mocked environment
//     t.execute_with(|| {
//         // Call the function and assert the successful download
//         let result = TemplateModule::download_wasm_from_nft_id(1);

//         assert_eq!(result.is_ok(), true);
//     });

//     // Check the log count
//     let log_count = *log_counter.lock().unwrap();
//     println!("LOGS count: {}", log_count);
// }

#[test]
fn test_download_wasm_from_input_uri_success_execution() {
    // Initialize the logger
    let log_counter = Arc::new(Mutex::new(0_u16));
    let log_counter_ref = Arc::clone(&log_counter);

    Builder::new()
        .filter_level(LevelFilter::Info)
        .format(move |buf, record| {
            {
                let mut counter = log_counter_ref.lock().unwrap();
                *counter += 1;
            }
            writeln!(buf, "{} - {}", record.level(), record.args())
        })
        .is_test(true)
        .try_init()
        .ok();

    let mut t = new_test_ext();

    // Setup the offchain worker environment
    let (offchain, state) = testing::TestOffchainExt::new();
    t.register_extension(OffchainWorkerExt::new(offchain.clone()));

    // Mock a successful HTTP response
    state.write().expect_request(testing::PendingRequest {
        method: "GET".into(),
        uri: "https://storage.gregoriogalante.com/uomi_example_input.txt".into(),
        meta: vec![],
        headers: vec![],
        body: vec![],
        response_headers: vec![],
        response: Some(vec![]),
        sent: true,
        ..Default::default()
    });

    t.execute_with(|| {
        AiAgentsExecutions::<Test>::put((
            1,
            "https://storage.gregoriogalante.com/uomi_example_input.txt"
                .as_bytes()
                .to_vec(),
        ));

        let (_, input_uri) = AiAgentsExecutions::<Test>::get();

        let result = TemplateModule::download_input_from_input_uri(input_uri);

        assert_eq!(result.is_ok(), true);
    });

    // Check the log count
    let log_count = *log_counter.lock().unwrap();
    println!("LOGS count: {}", log_count);
}

