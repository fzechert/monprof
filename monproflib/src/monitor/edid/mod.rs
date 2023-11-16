use std::io::{Read, Seek, SeekFrom};

use csv::DeserializeRecordsIter;
use log::trace;
use serde::Deserialize;

#[derive(Debug)]
pub enum EdidErrorKind {
    InvalidMagicId,
    UnexpectedEndOfStream,
}

#[derive(Debug)]
pub struct EdidError {
    message: String,
    kind: EdidErrorKind,
}

impl EdidError {
    fn new(message: String, kind: EdidErrorKind) -> EdidError {
        EdidError { message, kind }
    }
}

pub type Result<T> = std::result::Result<T, EdidError>;

const EDID_MAGIC_ID: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];
const EDID_MIN_LENGTH: u64 = 128;
const EDID_MANUFACTURER_ID_OFFSET: u64 = 8;
const EDID_MANUFACTURER_PRODUCT_CODE_OFFSET: u64 = 10;
const EDID_MANUFACTURER_SERIAL_NUMBER_OFFSET: u64 = 12;
const EDID_PNP_TABLE: &str = include_str!("PNP ID Registry.csv");
const DEFAULT_MANUFACTURER_NAME: &str = "Unknown Manufacturer";

#[derive(Debug, Deserialize)]
struct PnpIdRegistryEntry {
    #[serde(rename = "Company")]
    name: String,

    #[serde(rename = "PNP ID")]
    id: String,
}

#[derive(Debug)]
pub struct Edid {}

impl Edid {
    fn assert_magic_id<S>(stream: &mut S, length: u64) -> Result<()>
    where
        S: Seek + Read,
    {
        // We want to parse the EDID Magic Id, which is 8 bytes long.
        // We cannot do this, if the stream of data is too short.
        if length < EDID_MAGIC_ID.len() as u64 {
            return Err(EdidError::new(
                format!(
                    "Expected EDID data stream to contain at least {} bytes, found {} bytes",
                    EDID_MAGIC_ID.len(),
                    length
                ),
                EdidErrorKind::UnexpectedEndOfStream,
            ));
        }

        // Parse EDID Magic ID.
        let mut edid_magic_id: [u8; 8] = [0; 8];
        stream
            .seek(SeekFrom::Start(0))
            .expect("start of data stream should be available");
        stream
            .read_exact(&mut edid_magic_id)
            .expect("reading from inside bounds of a stream should be possible");
        if edid_magic_id == EDID_MAGIC_ID {
            trace!("Found a valid EDID magic id pattern");
            Ok(())
        } else {
            Err(EdidError::new(
                String::from("Stream header does not contain valid EDID magic id pattern"),
                EdidErrorKind::InvalidMagicId,
            ))
        }
    }

    fn assert_stream_length(length: u64) -> Result<()> {
        // DID base data, without any extensions, should be 128 bytes long. If it contains extensions, it can be longer.
        if length >= EDID_MIN_LENGTH {
            Ok(())
        } else {
            Err(EdidError::new(
                format!(
                    "Expected EDID data stream to contain at least {} bytes, found {} bytes",
                    EDID_MIN_LENGTH, length
                ),
                EdidErrorKind::UnexpectedEndOfStream,
            ))
        }
    }

    fn parse_manufacturer_id<S>(stream: &mut S) -> Result<String>
    where
        S: Seek + Read,
    {
        // Try to read the Manufacturer ID from the EDID data
        let mut manufacturer_id: [u8; 2] = [0, 0];

        stream
            .seek(SeekFrom::Start(EDID_MANUFACTURER_ID_OFFSET))
            .expect("seeking inside the bounds of a stream should be possible");
        stream
            .read_exact(&mut manufacturer_id)
            .expect("reading from inside bounds of a stream should be possible");

        let letter1 = {
            let letter_index = ((manufacturer_id[0] & 0b01111100) >> 2) as u8 - 1;
            (('A' as u8) + letter_index) as char
        };
        let letter2 = {
            let letter_index = (((manufacturer_id[0] & 0b00000011) << 3)
                | ((manufacturer_id[1] & 0b11100000) >> 5)) as u8
                - 1;
            (('A' as u8) + letter_index) as char
        };
        let letter3 = {
            let letter_index = (manufacturer_id[1] & 0b00011111) as u8 - 1;
            (('A' as u8) + letter_index) as char
        };

        let manufacturer_id = String::from_iter([letter1, letter2, letter3]);
        trace!("EDID manufacturer id: {}", manufacturer_id);
        Ok(manufacturer_id)
    }

    fn get_manufacturer_name(manufacturer_id: &str) -> String {
        csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .double_quote(true)
            .from_reader(EDID_PNP_TABLE.as_bytes())
            .deserialize::<PnpIdRegistryEntry>()
            .flatten() // remove all failed records
            .filter(|entry| entry.id == manufacturer_id)
            .next()
            .map(|entry| entry.name)
            .unwrap_or(String::from(DEFAULT_MANUFACTURER_NAME))
    }

    fn parse_product_code<S>(stream: &mut S) -> u16
    where
        S: Seek + Read,
    {
        let mut product_code: [u8; 2] = [0, 0];

        stream
            .seek(SeekFrom::Start(EDID_MANUFACTURER_PRODUCT_CODE_OFFSET))
            .expect("seeking inside the bounds of a stream should be possible");
        stream
            .read_exact(&mut product_code)
            .expect("reading from inside bounds of a stream should be possible");

        let code = product_code[0] as u16 | ((product_code[1] as u16) << 8);
        trace!("EDID manufacturer product code: {}", code);
        code
    }

    fn parse_manufacturer_serial_number<S>(stream: &mut S) -> u32
    where
        S: Seek + Read,
    {
        let mut serial_number: [u8; 3] = [0, 0, 0];

        stream
            .seek(SeekFrom::Start(EDID_MANUFACTURER_SERIAL_NUMBER_OFFSET))
            .expect("seeking inside the bounds of a stream should be possible");
        stream
            .read_exact(&mut serial_number)
            .expect("reading from inside bounds of a stream should be possible");

        let serial = serial_number[0] as u32
            | ((serial_number[1] as u32) << 8)
            | ((serial_number[2] as u32) << 16);
        trace!(
            "EDID manufacturer serial number: {:?}, {}",
            serial_number,
            serial
        );
        serial
    }

    pub fn parse<S>(stream: &mut S) -> Result<Edid>
    where
        S: Seek + Read,
    {
        // Find out how much data we have to work with
        let length = stream
            .seek(SeekFrom::End(0))
            .expect("end of data stream should be available");

        trace!("Trying to parse {} bytes of potential EDID data", length);

        // Is this EDID data?
        Self::assert_magic_id(stream, length)?;
        Self::assert_stream_length(length)?;

        // Manufacturer Id
        let manufacturer_id = Self::parse_manufacturer_id(stream)?;

        // Manufacturer Name
        let manufacturer_name = Self::get_manufacturer_name(&manufacturer_id);

        // Product Code
        let product_code = Self::parse_product_code(stream);

        // Serial Number
        let manufacturer_serial_number = Self::parse_manufacturer_serial_number(stream);
        todo!()
    }
}
