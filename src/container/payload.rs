use std::io::Read;
use std::io::Write;

use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;

use crate::Error;
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::PayloadCompressionHeader;

const COMPRESSION_FORMAT_VERSION_0: u16 = 0;
const RESERVED_BYTES_0: u16 = 0;

pub(crate) fn decode_payload(
    reader: &mut Reader<'_>,
) -> std::result::Result<(PayloadCompressionHeader, Vec<u8>), Error> {
    reader.set_section(ParseSection::Payload);

    let raw_size_offset = reader.position();
    let raw_size = reader.read_u32_be("payload raw size")?;
    let raw_size = usize::try_from(raw_size).map_err(|_| {
        reader.invalid_value(
            "payload raw size",
            raw_size_offset,
            "payload raw size does not fit in usize",
        )
    })?;

    let zip_size_offset = reader.position();
    let zip_size = reader.read_u32_be("payload zip size")?;
    let zip_size = usize::try_from(zip_size).map_err(|_| {
        reader.invalid_value(
            "payload zip size",
            zip_size_offset,
            "payload zip size does not fit in usize",
        )
    })?;

    if zip_size == 0 {
        return Err(reader.invalid_value(
            "payload zip size",
            zip_size_offset,
            "payload zip size must be non-zero",
        ));
    }

    let compression_version_offset = reader.position();
    let compression_version = reader.read_u16_be("payload compression format version")?;
    if compression_version != COMPRESSION_FORMAT_VERSION_0 {
        return Err(reader.unexpected_value(
            "payload compression format version",
            compression_version_offset,
            "0",
            compression_version.to_string(),
        ));
    }

    let reserved_offset = reader.position();
    let reserved = reader.read_u16_be("payload unused")?;
    if reserved != RESERVED_BYTES_0 {
        return Err(reader.unexpected_value(
            "payload unused",
            reserved_offset,
            "0",
            reserved.to_string(),
        ));
    }

    let zlib_bytes_offset = reader.position();
    let zlib_bytes = reader.read_bytes(zip_size, "payload zlib bytes")?;
    let mut decoder = ZlibDecoder::new(zlib_bytes);
    let mut payload = Vec::with_capacity(raw_size);
    decoder.read_to_end(&mut payload).map_err(|_| {
        reader.invalid_value(
            "payload zlib bytes",
            zlib_bytes_offset,
            "zlib decompression failed",
        )
    })?;

    if payload.len() != raw_size {
        return Err(reader.invalid_value(
            "payload decompressed size",
            raw_size_offset,
            "decompressed payload size does not match raw size",
        ));
    }

    Ok((
        PayloadCompressionHeader {
            raw_size: raw_size as u32,
            zip_size: zip_size as u32,
            compression_format_version: compression_version,
            reserved,
        },
        payload,
    ))
}

pub(crate) fn encode_payload(
    writer: &mut Writer,
    payload: &[u8],
) -> std::result::Result<PayloadCompressionHeader, Error> {
    if payload.is_empty() {
        return Err(Error::InvalidModel {
            field: "payload",
            message: "payload must not be empty",
        });
    }

    let raw_size = u32::try_from(payload.len()).map_err(|_| Error::InvalidModel {
        field: "payload",
        message: "payload length must fit in u32",
    })?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(payload)
        .map_err(|_| Error::InvalidModel {
            field: "payload",
            message: "payload compression failed",
        })?;
    let zlib_bytes = encoder.finish().map_err(|_| Error::InvalidModel {
        field: "payload",
        message: "payload compression failed",
    })?;

    let zip_size = u32::try_from(zlib_bytes.len()).map_err(|_| Error::InvalidModel {
        field: "payload",
        message: "compressed payload length must fit in u32",
    })?;

    let info = PayloadCompressionHeader {
        raw_size,
        zip_size,
        compression_format_version: COMPRESSION_FORMAT_VERSION_0,
        reserved: RESERVED_BYTES_0,
    };

    writer.write_u32_be(info.raw_size);
    writer.write_u32_be(info.zip_size);
    writer.write_u16_be(info.compression_format_version);
    writer.write_u16_be(info.reserved);
    writer.write_bytes(&zlib_bytes);

    Ok(info)
}
