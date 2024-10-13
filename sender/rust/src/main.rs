mod proto;
mod qr;

use crate::proto::wrapper::{self};
use clap::builder::PossibleValue;
use clap::{Arg, ArgAction, Command};
use qrcode::EcLevel;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

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

    let file = file_to_data(in_file).unwrap();
    let name = in_file.file_name().unwrap().to_str().unwrap();
    qr::encode_as_qr_codes(ecl, &file.data, name, out_dir).unwrap();
}

fn file_to_data(file_path: &Path) -> Result<wrapper::File, Error> {
    let data = {
        let mut f = File::open(&file_path)?;
        let file_len = f.metadata()?.len() as usize;
        let mut data = vec![0u8; file_len];
        f.read(&mut data)?;
        data
    };

    // TODO data should be meta data + file data
    Ok(wrapper::File {
        crc32: 0,
        filename: file_path.file_name().unwrap().to_str().unwrap().as_bytes().to_vec(),
        size: data.len() as u64,
        data,
    })
}

