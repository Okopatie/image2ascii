use std::path;

use clap::Parser;
use image::{imageops::FilterType, open, DynamicImage};

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    path: path::PathBuf,

    #[arg(short, long, value_names = ["WIDTH", "HEIGHT"], num_args = 2)]
    size: Option<Vec<u32>>,
}

fn main() {
    let args = Cli::parse();
    let image_file = open(args.path).expect("Unable to open file");
    let image = if let Some(size) = args.size {
        image_file.resize(size[0], size[1], FilterType::Triangle)
    } else {
        image_file
    };
    convert_to_ascii(&image);
}

fn convert_to_ascii(imgbuf: &DynamicImage) {
    let mut output: Vec<String> = Vec::new();
    for row in imgbuf.to_luma8().rows() {
        let mut row_output = Vec::new();
        for pixel in row {
            row_output.push(match pixel.0[0] {
                0..4 => ' ',
                4..8 => '.',
                8..12 => '\'',
                12..16 => '`',
                16..20 => '^',
                20..24 => '"',
                24..28 => ':',
                28..32 => ';',
                32..36 => 'l',
                36..40 => '!',
                40..44 => 'i',
                44..48 => '>',
                48..52 => '<',
                52..56 => '~',
                56..60 => '+',
                60..64 => '_',
                64..68 => '-',
                68..72 => '?',
                72..76 => ']',
                76..80 => '[',
                80..84 => '}',
                84..88 => '{',
                88..92 => '1',
                92..96 => ')',
                96..100 => '(',
                100..104 => '|',
                104..108 => '\\',
                108..112 => '/',
                112..116 => 't',
                116..120 => 'f',
                120..124 => 'j',
                124..128 => 'r',
                128..132 => 'x',
                132..136 => 'n',
                136..140 => 'u',
                140..144 => 'v',
                144..148 => 'c',
                148..152 => 'z',
                152..156 => 'X',
                156..160 => 'X',
                160..164 => 'Y',
                164..168 => 'U',
                168..172 => 'J',
                172..176 => 'C',
                176..180 => 'L',
                180..184 => 'Q',
                184..188 => '0',
                188..192 => 'O',
                192..196 => 'Z',
                196..200 => 'm',
                200..204 => 'd',
                204..208 => 'k',
                208..212 => 'h',
                212..216 => 'a',
                216..220 => 'o',
                220..224 => '*',
                224..228 => '#',
                228..232 => 'M',
                232..236 => 'W',
                236..240 => '&',
                240..244 => '8',
                244..248 => '%',
                248..252 => 'B',
                252.. => '@',
            });
        }
        output.push(row_output.iter().collect());
    }
    let output_string: String = output.join("\n");
    print!("{}", output_string);
}
