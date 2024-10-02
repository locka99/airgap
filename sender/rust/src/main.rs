use qrcode::QrCode;
use image::Luma;
use clap::{command, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use clap::builder::PossibleValue;

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
                .help("Output File")
                .action(ArgAction::Set)
                .num_args(1..)
        )
        .get_matches();

    // Encode some data into bits.
    let code = QrCode::new(b"01234567").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("qrcode.png").unwrap();

    // You can also render it into a string.
    let string = code.render()
        .light_color(' ')
        .dark_color('#')
        .build();
    println!("{}", string);
}
