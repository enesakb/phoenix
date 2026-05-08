use phoenix_core::logging::init_for_tests;
use tracing::info;

#[test]
fn logging_init_does_not_panic() {
    let _guard = init_for_tests();
    info!("phoenix logging initialized");
}
