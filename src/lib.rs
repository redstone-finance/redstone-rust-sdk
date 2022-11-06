use bitmaps::Bitmap;
use std::fmt::Write;
use std::str;

const REDSTONE_MARKER_BS: usize = 9;
const UNSIGNED_METADATA_BYTE_SIZE_BS: usize = 3;
const DATA_PACKAGES_COUNT_BS: usize = 2;
const MAX_SIGNERS_COUNT: usize = 256;
const REDSTONE_MARKER: [u8; 9] = [0, 0, 2, 237, 87, 1, 30, 0, 0]; // 0x000002ed57011e0000

struct DataPackageExtractionResult {
    contains_requested_data_feed: bool,
    value_for_requested_data_feed: u128,
    signer_index: usize,
    data_package_byte_size: usize,
}

pub fn get_oracle_value(redstone_payload: &[u8], data_feed_id: &[u8; 32]) -> u128 {
    do_helpful_logging(redstone_payload, data_feed_id);
    assert_valid_redstone_marker(redstone_payload);
    let mut negative_offset = extract_unsigned_metadata_offset(redstone_payload);
    let number_of_data_packages =
        extract_number_of_data_packages(redstone_payload, negative_offset);
    let mut unique_signers_bitmap = Bitmap::<MAX_SIGNERS_COUNT>::new();
    let mut values: Vec<u128> = vec![];

    for _data_package_index in 0..number_of_data_packages {
        let DataPackageExtractionResult {
            contains_requested_data_feed,
            value_for_requested_data_feed,
            signer_index,
            data_package_byte_size,
        } = extract_data_package(redstone_payload, negative_offset);

        // Shifting negative offset to the next package
        negative_offset += data_package_byte_size;

        // Collect value if needed
        if contains_requested_data_feed && !unique_signers_bitmap.get(signer_index) {
            unique_signers_bitmap.set(signer_index, true);
            values.push(value_for_requested_data_feed)
        }
    }

    aggregate_values(&values)
}

fn assert_valid_redstone_marker(redstone_payload: &[u8]) {
    let marker_start_index = redstone_payload.len() - REDSTONE_MARKER_BS;
    let redstone_marker = &redstone_payload[marker_start_index..];
    println!("Marker: {0}", encode_hex(redstone_marker));
    if REDSTONE_MARKER != redstone_marker {
        panic!("Invalid redstone marker");
    }
}

fn extract_unsigned_metadata_offset(redstone_payload: &[u8]) -> usize {
    let end_index = redstone_payload.len() - REDSTONE_MARKER_BS; // not inclusive
    let start_index = end_index - UNSIGNED_METADATA_BYTE_SIZE_BS;
    let unsigned_metadata_bs_bytes = &redstone_payload[start_index..end_index];
    let unsigned_metadata_bs =
        usize::try_from(bytes_arr_to_number(unsigned_metadata_bs_bytes)).unwrap();

    unsigned_metadata_bs + UNSIGNED_METADATA_BYTE_SIZE_BS + REDSTONE_MARKER_BS
}

fn extract_number_of_data_packages(
    redstone_payload: &[u8],
    unsigned_metadata_offset: usize,
) -> usize {
    let end_index = redstone_payload.len() - unsigned_metadata_offset;
    let start_index = end_index - DATA_PACKAGES_COUNT_BS;
    let data_packages_count_bytes = &redstone_payload[start_index..end_index];

    println!(
        "data_packages_count_bytes: {0}",
        encode_hex(data_packages_count_bytes)
    );

    usize::try_from(bytes_arr_to_number(data_packages_count_bytes)).unwrap()
}

fn extract_data_package(
    redstone_payload: &[u8],
    negative_offset: usize,
) -> DataPackageExtractionResult {
    DataPackageExtractionResult {
        contains_requested_data_feed: true,
        value_for_requested_data_feed: 42,
        signer_index: 0,
        data_package_byte_size: 120,
    }
}

// TODO: implement median aggregation
fn aggregate_values(values: &Vec<u128>) -> u128 {
    values[0]
}

// TODO: remove later
fn do_helpful_logging(redstone_payload: &[u8], data_feed_id: &[u8; 32]) {
    println!("Redstone payload byte size: {0}", redstone_payload.len());
    let data_feed_id_to_print: &[u8] = &data_feed_id.to_vec();
    println!(
        "Requested data feed: {0}",
        str::from_utf8(data_feed_id_to_print).unwrap()
    );
}

fn bytes_arr_to_number(number_bytes: &[u8]) -> u128 {
    let mut result_number = 0;
    let mut multiplier = 1;
    println!("number_bytes.len(): {0}", number_bytes.len());
    for i in (0..number_bytes.len()).rev() {
        println!(
            "i: {0}, byte: {1}, multiplier: {2}",
            i, number_bytes[i], multiplier
        );
        result_number += u128::from(number_bytes[i]) * multiplier;
        multiplier *= 256;
    }
    result_number
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}
