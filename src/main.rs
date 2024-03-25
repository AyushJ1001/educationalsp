use std::io::{self, BufRead, Read};

use anyhow::{anyhow, Result};
use simple_log::{info, LogConfigBuilder};

pub mod rpc;

fn main() -> Result<()> {
    get_logger().unwrap();

    info!("Hey, I started!");

    let mut header_buf = Vec::new();

    loop {
        //'Content-Length: 15\r\n\r\n{"method":"hi"}'

        // 1. read until '\r'
        // -> 'Content-Length: 15'

        // 2. parse number from last
        // -> 15

        // 3. read exactly (15)+4 bytes = 19 bytes
        // -> '\r\n\r\n{"method:"hi"}'

        // 4. take out first 4 bytes.

        // 5. step 1 -> header, step 2 -> content

        let header_length = io::stdin().lock().read_until(b'\r', &mut header_buf)?;

        if header_length == 0 {
            return Err(anyhow!("EOF"));
        }

        let content_length_bytes = &header_buf["Content-Length: ".len()..header_buf.len() - 1];
        println!(
            "Content-Length: {:?}",
            std::str::from_utf8(content_length_bytes)?
        );
        let content_length = match std::str::from_utf8(content_length_bytes)?.parse::<usize>() {
            Err(_) => return Err(anyhow!("Invalid content length")),
            Ok(v) => v,
        };
        info!("Content-Length: {}", content_length);

        let mut content_buf = vec![0; content_length + 3];

        io::stdin().lock().read_exact(&mut content_buf)?;

        header_buf.pop();
        content_buf.drain(..3);
        info!("Header: {:?}", std::str::from_utf8(&header_buf)?);
        info!("Content: {:?}", std::str::from_utf8(&content_buf)?);
        info!("Content-Length: {}", content_buf.len());
    }
}

fn get_logger() -> Result<(), String> {
    let config = LogConfigBuilder::builder()
        .path("./log.txt")
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f")
        .level("debug")
        .output_file()
        .output_console()
        .build();

    simple_log::new(config)?;

    Ok(())
}
