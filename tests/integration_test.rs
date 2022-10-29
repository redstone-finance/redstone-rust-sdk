use redstone_rust_sdk;

#[test]
fn it_gets_oracle_value() {
  assert_eq!(42, redstone_rust_sdk::get_oracle_value());
}
