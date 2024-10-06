mod proto;
mod qr;

use std::fs;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
use clap::builder::PossibleValue;
use clap::{Arg, ArgAction, Command};
use image::Luma;
use qrcode::{EcLevel, QrCode};

use crate::proto::wrapper::{self};
use crate::qr::LEVEL_30;

fn main() {
    // --ecl [LMQH]
    // --qr size
    // --input infile
    // --output png

    let matches = Command::new("airgap")
        .about("airgap utility")
        .version("1.0.0")
        .arg_required_else_help(true)
        .arg(
            Arg::new("ecl")
                .short('e')
                .long("ecl")
                .help("Error Correction Level within QR code")
                .action(ArgAction::Set)
                .num_args(1)
                .default_value("L")
                .value_parser([
                    PossibleValue::new("L").help("Approx 7%"),
                    PossibleValue::new("M").help("Approx 15%"),
                    PossibleValue::new("Q").help("Approx 25%"),
                    PossibleValue::new("H").help("Approx 30%")
                ])
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Input File(s)")
                .action(ArgAction::Set)
                .num_args(1..)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output Directory")
                .action(ArgAction::Set)
                .num_args(1..)
        )
        .get_matches();

    let ecl = matches.get_one::<String>("ecl").map(|ecl| {
        match ecl.as_str() {
            "M" => EcLevel::M,
            "Q" => EcLevel::Q,
            "H" => EcLevel::H,
            _ => EcLevel::L,
        }
    }).unwrap();

    let in_file = matches.get_one::<String>("input").map(|f| Path::new(f)).unwrap();
    let out_dir = matches.get_one::<String>("output").map(|f| Path::new(f)).unwrap();

    let data = file_to_data(in_file).unwrap();
    generate_qrs(ecl, &data, out_dir).unwrap();
}

fn file_to_data(file_path: &Path) -> Result<Vec<u8>, Error> {
    let file_data = {
        let mut f = File::open(&file_path)?;
        let file_len = f.metadata()?.len() as usize;
        let mut data = vec![0u8; file_len];
        f.read(&mut data)?;
        data
    };

    // TODO data should be meta data + file data

    let data = file_data;
    Ok(data)
}

fn generate_qrs(ecl: EcLevel, data: &[u8], out_dir: &Path) -> Result<(), Error> {
    let level = LEVEL_30;
    let packet_size = {
        let level_index = match ecl {
            EcLevel::L => 2,
            EcLevel::M => 1,
            EcLevel::Q => 2,
            EcLevel::H => 3,
        };
        level[level_index] as usize
    };

    let mut packets = Vec::new();
    let start_idx = 0;

    loop {
        let mut packet_data = vec![0u8; packet_size];
            packets.push(wrapper::Packet {
                packet_nr: packets.len() as u64,
                data
            });
        } else {
            break;
        }
    }

    let header = wrapper::Header {
        num_packets: packets.len() as u64,
        version: 0,
    };

    // Now the header and packets are going to be written as
    // QR codes

    // Encode some data into bits.
    let code = QrCode::with_error_correction_level(b"01234567", ecl).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    let out_file = out_dir.join("qrcode.png");
    image.save(&out_file).unwrap();

    // You can also render it into a string.
    let string = code.render()
        .light_color(' ')
        .dark_color('#')
        .build();
    println!("{}", string);
}
