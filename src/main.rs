use clap::Parser;

use std::{
    fs, fs::File,
    io::Write, 
    path, 
    process::exit
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: path::PathBuf,

    #[arg(short, long)]
    output_file: path::PathBuf,

    #[arg(short, long)]
    decode: bool,

    #[arg(short, long)]
    encode: bool,

    #[arg(short, long)]
    save_image: bool,

    #[arg(short, long)]
    width: Option<u16>
}

fn main() {
    let args = Args::parse();

    if args.encode == args.decode {
        eprintln!("Error: You're decoding and encoding options are incorrect");
        exit(0);
    }

    let input_file = match fs::read_to_string(args.input_file) {
        Err(_) => { 
            eprintln!("Error: Couldn't read input file"); 
            exit(0); 
        },
        Ok(file) => { 
            let f = file.replace("\n", "");
            
            f.as_bytes().to_vec()
        },
    };

    let mut output_file = match File::create(args.output_file) {
        Err(_) => { 
            eprintln!("Error: Couldn't create output file"); 
            exit(0); 
        },
        Ok(file) => { file },
    };

    let mut data = match (args.encode, args.decode) {
        (true, false) => {
            let mut data = input_file.encode();
            data.remove(0);

            let mut data = data
                .iter().map(|x| format!("{x}, "))
                .collect::<String>();

            data.insert(0, '[');
            data.pop();
            data.pop();
            data.push(']');

            data
        },
        (false, true) => {
            let data = serde_json::from_slice::<Vec<i32>>(&input_file).unwrap()
                .decode()
                .iter().map(|x| x.to_string())
                .collect::<String>();

            data
        },
        (_, _) => { 
            eprintln!("Error: Decoding and encoding options are wrong"); 
            exit(0);
        }
    };

    if args.decode && args.save_image {
        data.insert_str(0, &format!("P1\n#Bitmap Image Generate by Rusty_RLE\n{} {}\n", args.width.unwrap(), data.len() as u16 / args.width.unwrap()))
    }

    output_file.write_all(data.as_bytes()).unwrap();
}

pub trait Encode {
    fn encode(&self) -> Vec<i32>;
}

pub trait Decode {
    fn decode(&self) -> Vec<u8>;
}

impl Encode for [u8] {
    fn encode(&self) -> Vec<i32> {
        let mut bits = vec![];
        let mut last_bit = 1;
        let mut count = 0;

        for bit in self {
            if bit == &last_bit {
                count += 1;
            } else {
                bits.push(count);
                count = 0;
            }

            last_bit = *bit;
        }

        bits.push(count);

        return bits
    }
}

impl Decode for [i32] {
    fn decode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];

        let mut byte = 1;
        for num in self {
            (0..=*num).for_each(|_| buf.push(byte) );

            if byte == 1 {
                byte = 0;
            } else {
                byte = 1;
            }
        }

        buf
    }
}