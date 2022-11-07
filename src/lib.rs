use bitmaps::Bitmap;
use std::fmt::Write;
use std::str;

const REDSTONE_MARKER_BS: usize = 9;
const UNSIGNED_METADATA_BYTE_SIZE_BS: usize = 3;
const DATA_PACKAGES_COUNT_BS: usize = 2;
const DATA_POINTS_COUNT_BS: usize = 3;
const SIGNATURE_BS: usize = 65;
const MAX_SIGNERS_COUNT: usize = 256;
const DATA_POINT_VALUE_BYTE_SIZE_BS: usize = 4;
const DATA_FEED_ID_BS: usize = 32;
const TIMESTAMP_BS: usize = 6;
const REDSTONE_MARKER: [u8; 9] = [0, 0, 2, 237, 87, 1, 30, 0, 0]; // 0x000002ed57011e0000

struct DataPackageExtractionResult {
    contains_requested_data_feed: bool,
    value_for_requested_data_feed: u128,
    signer_index: usize,
    data_package_byte_size: usize,
}

pub fn get_oracle_value(
    data_feed_id: &[u8; 32],
    unique_signers_threshold: u8,
    authorised_signers: &[[u8; 33]],
    redstone_payload: &[u8],
) -> u128 {
    do_helpful_logging(redstone_payload, data_feed_id);
    assert_valid_redstone_marker(redstone_payload);
    let mut negative_offset = extract_unsigned_metadata_offset(redstone_payload);
    let number_of_data_packages =
        extract_number_of_data_packages(redstone_payload, negative_offset);
    negative_offset += DATA_PACKAGES_COUNT_BS;
    let mut unique_signers_bitmap = Bitmap::<MAX_SIGNERS_COUNT>::new();
    let mut values: Vec<u128> = vec![];

    for _data_package_index in 0..number_of_data_packages {
        let DataPackageExtractionResult {
            contains_requested_data_feed,
            value_for_requested_data_feed,
            signer_index,
            data_package_byte_size,
        } = extract_data_package(
            data_feed_id,
            redstone_payload,
            negative_offset,
            authorised_signers,
        );

        // Shifting negative offset to the next package
        negative_offset += data_package_byte_size;

        // Collect value if needed
        if contains_requested_data_feed && !unique_signers_bitmap.get(signer_index) {
            unique_signers_bitmap.set(signer_index, true);
            values.push(value_for_requested_data_feed)
        }
    }

    if values.len() < usize::from(unique_signers_threshold) {
        panic!("Insufficient number of unique signers");
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
    let unsigned_metadata_bs =
        extract_usize_num_from_redstone_payload(redstone_payload, start_index, end_index);

    unsigned_metadata_bs + UNSIGNED_METADATA_BYTE_SIZE_BS + REDSTONE_MARKER_BS
}

fn extract_number_of_data_packages(
    redstone_payload: &[u8],
    unsigned_metadata_offset: usize,
) -> usize {
    let end_index = redstone_payload.len() - unsigned_metadata_offset;
    let start_index = end_index - DATA_PACKAGES_COUNT_BS;
    extract_usize_num_from_redstone_payload(redstone_payload, start_index, end_index)
}

fn extract_data_package(
    requested_data_feed_id: &[u8; 32],
    redstone_payload: &[u8],
    negative_offset_to_package: usize,
    authorised_signers: &[[u8; 33]],
) -> DataPackageExtractionResult {
    let mut value_for_requested_data_feed: u128 = 0;
    let mut contains_requested_data_feed = false;
    let mut signer_index: usize = 0;

    // Extracting signature
    let mut end_index = redstone_payload.len() - negative_offset_to_package;
    let mut start_index = end_index - SIGNATURE_BS;
    let signature = &redstone_payload[start_index..end_index];

    // Extracting number of data points
    start_index -= DATA_POINTS_COUNT_BS;
    end_index = start_index + DATA_POINTS_COUNT_BS;
    let data_points_count =
        extract_usize_num_from_redstone_payload(redstone_payload, start_index, end_index);

    // Extracting data points value byte size
    start_index -= DATA_POINT_VALUE_BYTE_SIZE_BS;
    end_index = start_index + DATA_POINT_VALUE_BYTE_SIZE_BS;
    let data_points_value_bs =
        extract_usize_num_from_redstone_payload(redstone_payload, start_index, end_index);

    // Calculating total data package byte size
    let data_package_byte_size = (data_points_value_bs + DATA_FEED_ID_BS) * data_points_count
        + TIMESTAMP_BS
        + DATA_POINT_VALUE_BYTE_SIZE_BS
        + DATA_POINTS_COUNT_BS
        + SIGNATURE_BS;

    // Extracting and validating timestamp
    start_index -= TIMESTAMP_BS;
    end_index = start_index + TIMESTAMP_BS;
    let timestamp = bytes_arr_to_number(&redstone_payload[start_index..end_index]);
    validate_timestamp(timestamp);

    // Going through data points
    for _data_point_index in 0..data_points_count {
        // Extracting value
        start_index -= data_points_value_bs;
        end_index = start_index + data_points_value_bs;
        let data_point_value = bytes_arr_to_number(&redstone_payload[start_index..end_index]);

        // Extracting data feed id
        start_index -= DATA_FEED_ID_BS;
        end_index = start_index + DATA_FEED_ID_BS;
        let data_feed_id = &redstone_payload[start_index..end_index];

        if data_feed_id == requested_data_feed_id {
            value_for_requested_data_feed = data_point_value;
            contains_requested_data_feed = true;
            break;
        }
    }

    // Message construction, hashing and signer recovering
    // TODO: implement
    if signer_index == 0 {
        signer_index = 1;
    }

    println!("\n\nSignature: {0}", encode_hex(signature));

    DataPackageExtractionResult {
        contains_requested_data_feed,
        value_for_requested_data_feed,
        data_package_byte_size,
        signer_index,
    }
}

// TODO: implement median aggregation
fn aggregate_values(values: &Vec<u128>) -> u128 {
    values[0]
}

// TODO: make it configurable
fn validate_timestamp(received_timestamp: u128) {
    if received_timestamp == 0 {
        panic!("Timestamp is invalid");
    }
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

// TODO: implement
fn extract_usize_num_from_redstone_payload(
    redstone_payload: &[u8],
    start: usize,
    end: usize,
) -> usize {
    let number_bytes = &redstone_payload[start..end];
    usize::try_from(bytes_arr_to_number(number_bytes)).unwrap()
}

fn bytes_arr_to_number(number_bytes: &[u8]) -> u128 {
    let mut result_number = 0;
    let mut multiplier = 1;

    for i in (0..number_bytes.len()).rev() {
        // To prevent overflow error
        if i == 16 {
            break;
        }
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
