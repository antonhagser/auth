//! String based tokens for use in ex. URLs.

use base64::engine::general_purpose;
use std::io::Write;
use uuid::Uuid;

/// Converts a UUID to a base64 string.
pub fn uuid_to_base64(uuid: Uuid) -> String {
    // generate a random UUID
    let uuid_bytes = uuid.as_bytes();

    // create a base64 encoder that writes to a String
    let mut enc = base64::write::EncoderStringWriter::new(&general_purpose::URL_SAFE_NO_PAD);
    enc.write_all(uuid_bytes).unwrap();

    // get the resulting String
    enc.into_inner()
}

/// Generates a random UUID and converts it to a base64 string.
pub fn random_token() -> String {
    let uuid = Uuid::new_v4();
    uuid_to_base64(uuid)
}
