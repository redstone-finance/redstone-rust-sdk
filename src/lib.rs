use std::str;

pub fn get_oracle_value(redstone_payload: &[u8], data_feed_id: &[u8; 32]) -> u128 {
    println!("Redstone payload byte size: {0}", redstone_payload.len());
    let data_feed_id_to_print: &[u8] = &data_feed_id.to_vec();
    println!(
        "Requested data feed: {0}",
        str::from_utf8(data_feed_id_to_print).unwrap()
    );
    42
}
