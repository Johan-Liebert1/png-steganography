use std::process::exit;

const PNG_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

const IEND_CHUNK: [u8; 4] = [73, 69, 78, 68];

struct Chunk<'a> {
    length: u32,
    chunk_type: &'a [u8],
    chunk_data: &'a [u8],
    crc: u32,
}

fn main() {
    let file_name = if let Some(thing) = std::env::args().skip(1).next() {
        println!("{thing:?}");
        thing
    } else {
        "./tux.png".into()
    };

    let file_bytes = match std::fs::read(format!("{file_name}")) {
        Ok(b) => b,

        Err(err) => {
            eprintln!("Failed to open file {file_name}. Err: {err:?}");
            exit(1);
        }
    };

    if file_bytes[..PNG_HEADER.len()] != PNG_HEADER {
        eprintln!("File {file_name} is NOT a valid PNG file. PNG header mismatch.");
        exit(1);
    }

    println!("File {file_name} is a valid PNG file");

    let mut current_index = PNG_HEADER.len();

    loop {
        let chunk_len_as_u8: Result<[u8; 4], _> =
            file_bytes[current_index..current_index + 4].try_into();

        let chunk_len_as_u8 = match chunk_len_as_u8 {
            Ok(c) => c,

            Err(err) => {
                eprintln!("Failed to get chunk length. Err: {err:?}");
                exit(1);
            }
        };

        // This is stored in big endian
        let chunk_len = u32::from_be_bytes(chunk_len_as_u8);

        current_index += 4;

        let chunk_code = &file_bytes[current_index..current_index + 4];

        current_index += 4;

        let chunk_data = &file_bytes[current_index..current_index + (chunk_len as usize)];

        current_index += chunk_len as usize;

        let crc = &file_bytes[current_index..current_index + 4];

        current_index += 4;

        println!("chunk_len = {chunk_len:}, chunk_code: {chunk_code:?}");

        if chunk_code == IEND_CHUNK {
            println!("Encountered IEND");
            break;
        }
    }
}
