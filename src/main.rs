use std::{
    fs::{File, OpenOptions},
    os::unix::fs::FileExt,
    process::exit,
};

use core::mem::size_of;

const PNG_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const IEND_CHUNK: [u8; 4] = [73, 69, 78, 68];

struct Chunk<'a> {
    len: i32,
    code: [u8; 4],
    data: &'a [u8],
    crc: i32,
}

impl<'a> Chunk<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut vector = Vec::with_capacity(size_of::<i32>() + 4 + self.data.len() + 4);

        let thing = self.len.to_be_bytes();

        vector.extend_from_slice(&thing);
        vector.extend_from_slice(&self.code);
        vector.extend_from_slice(self.data);
        vector.extend_from_slice(&self.len.to_be_bytes());

        vector
    }
}

fn write_chunk_to_file(file: &File, chunk: &[u8], offset: u64) -> Result<usize, std::io::Error> {
    let res = file.write_at(chunk, offset)?;
    Ok(res)
}

fn main() {
    let (input_file_name, output_file_name) = if let Some(thing) = std::env::args().skip(1).next() {
        let idx = thing.rfind(".").unwrap_or(thing.len());
        let mut output_file_name = String::from(&thing[..idx]);
        output_file_name.push_str("-output.png");

        (thing, output_file_name)
    } else {
        ("./tux.png".into(), "./tux-output.png".into())
    };

    let mut file_write_offset = 0;

    let file_bytes = match std::fs::read(format!("{input_file_name}")) {
        Ok(b) => b,

        Err(err) => {
            eprintln!("Failed to open file {input_file_name}. Err: {err}");
            exit(1);
        }
    };

    if file_bytes[..PNG_HEADER.len()] != PNG_HEADER {
        eprintln!("File {input_file_name} is NOT a valid PNG file. PNG header mismatch.");
        exit(1);
    }

    let mut current_index = PNG_HEADER.len();

    let output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&output_file_name);

    let output_file = match output_file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file {output_file_name}. Err: {e}");
            exit(1);
        }
    };

    let status = write_chunk_to_file(
        &output_file,
        &file_bytes[..current_index],
        file_write_offset,
    );

    match status {
        Ok(total_bytes_written) => file_write_offset += total_bytes_written as u64,
        Err(e) => {
            eprintln!("Failed to write chunk to file. Err: {e}");
            exit(1);
        }
    };

    let mut write = 1;
    let mut wrote = false;

    loop {
        let original_idx = current_index;

        let chunk_len_as_u8: Result<[u8; 4], _> =
            file_bytes[current_index..current_index + 4].try_into();

        let chunk_len_as_u8 = match chunk_len_as_u8 {
            Ok(c) => c,

            Err(err) => {
                eprintln!("Failed to get chunk length. Err: {err}");
                exit(1);
            }
        };

        if write % 2 == 0 && !wrote {
            wrote = true;
            let chunk = Chunk {
                len: 4,
                code: *b"coDe",
                data: b"hide",
                crc: 0,
            };

            let thing = write_chunk_to_file(&output_file, &chunk.to_bytes(), file_write_offset)
                .unwrap_or(0);

            file_write_offset += thing as u64;
        }

        write += 1;

        // This is stored in big endian
        let chunk_len = u32::from_be_bytes(chunk_len_as_u8);
        current_index += 4;

        let chunk_code = &file_bytes[current_index..current_index + 4];
        current_index += 4;

        let _chunk_data = &file_bytes[current_index..current_index + (chunk_len as usize)];
        current_index += chunk_len as usize;

        let _crc = &file_bytes[current_index..current_index + 4];
        current_index += 4;

        let status = write_chunk_to_file(
            &output_file,
            &file_bytes[original_idx..current_index],
            file_write_offset,
        );

        match status {
            Ok(total_bytes_written) => file_write_offset += total_bytes_written as u64,
            Err(e) => {
                eprintln!("Failed to write chunk to file. Err: {e}");
                exit(1);
            }
        };

        println!("chunk_len = {chunk_len:}, chunk_code: {chunk_code:?}. Write status: {status:?}");

        if chunk_code == IEND_CHUNK {
            println!("Encountered IEND");
            break;
        }
    }
}
