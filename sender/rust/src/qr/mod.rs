// Corresponding to LMQH

use crate::proto::wrapper;
use image::Luma;
use prost::encoding::{encode_varint, encoded_len_varint};
use qrcode::{EcLevel, QrCode};
use std::io::{Error, Write};
use std::path::Path;

// 57x57
const LEVEL_10: [u16; 4] = [271, 213, 151, 119];

// 97x97
const LEVEL_20: [u16; 4] = [1091, 857, 611, 461];

// 137x137
const LEVEL_30: [u16; 4] = [1732, 1370, 982, 742];

// 177x177
const LEVEL_40: [u16; 4] = [2953, 2331, 1663, 1273];

fn qr_code_payload_size(level: u16, ecl: EcLevel) -> usize {
    let level_index = match ecl {
        EcLevel::L => 2,
        EcLevel::M => 1,
        EcLevel::Q => 2,
        EcLevel::H => 3,
    };
    match level {
        30 => LEVEL_30[level_index] as usize,
        _ => panic!("Level not supported")
    }
}

pub fn encode_as_qr_codes(ecl: EcLevel, data: &[u8], name: &str, out_dir: &Path) -> Result<(), Error> {
    let packet_size = qr_code_payload_size(30, ecl);

    let mut packets = Vec::new();
    let mut start: usize = 0;

    loop {
        let packet_nr = packets.len() as u64;
        let packet_nr_len = encoded_len_varint(packet_nr);
        let chunk_size = packet_size - packet_nr_len;

        // Read through chunks of data
        let end: usize = if start + chunk_size > data.len() {
            data.len()
        } else {
            start + chunk_size
        };

        let mut packet_nr_buf = vec![0u8; 10];
        encode_varint(packet_nr as u64, &mut packet_nr_buf);

        let mut packet_data = vec![0u8; packet_size];

        // Packet consists of packet nr and the bytes
        let mut bytes_written = packet_data.write(&packet_nr_buf[0..packet_nr_len])?;
        bytes_written = bytes_written + packet_data.write(&data[start..end])?;
        assert_eq!(bytes_written, packet_nr_len + (end - start));
        packet_data.resize(bytes_written, 0u8);

        packets.push(wrapper::Packet {
            packet: packets.len() as u64,
            data: packet_data.to_vec(),
        });

        start = end;
        if end == data.len() {
            break;
        }
    }

    //    let header = wrapper::Header {
    //        num_packets: packets.len() as u64,
    //        version: 0,
    //    };
    //    header.encode
    //    let code = QrCode::with_error_correction_level(&p.data, ecl).unwrap();
    //    // Render the bits into an image.
    //    let image = code.render::<Luma<u8>>().build();
    //    let file_name = format!("{}.header.png", name);
    //    let out_file = out_dir.join(file_name);
    //    image.save(&out_file).unwrap();

    // Now the header and packets are going to be written as
    // QR codes

    packets.iter().enumerate().for_each(|(i, p)| {
        let file_name = format!("{}-{}.qr.png", name, i);
        let _ = write_qr_file(ecl, out_dir, &file_name, p);
    });

    Ok(())
}

fn write_qr_file(ecl: EcLevel, out_dir: &Path, file_name: &str, packet: &wrapper::Packet) -> Result<(), Error> {
    // Encode some data into bits.
    let code = QrCode::with_error_correction_level(&packet.data, ecl).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    let out_file = out_dir.join(file_name);
    image.save(&out_file).unwrap();

    // You can also render it into a string.
    //        let string = code.render()
    //            .light_color(' ')
    //            .dark_color('#')
    //            .build();
    //        println!("{}", string);

    Ok(())
}