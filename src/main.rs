use std::io::{self, BufRead, Read};

use anyhow::{anyhow, Result};
use simple_log::{info, LogConfigBuilder};

pub mod lsp;
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

        header_buf.append(&mut content_buf);

        let msg = std::str::from_utf8(&header_buf)?;

        let (method, contents) = match rpc::decode_message(msg.to_string()) {
            Err(e) => {
                info!("Got an error: {}", e);
                continue;
            }
            Ok(v) => v,
        };

        handle_message(method, contents);
    }
}

fn handle_message(method: String, contents: String) {
    info!("Received msg with method: {}", method);

    match method.as_str() {
        "initialize" => {
            let request =
                match serde_json::from_str::<lsp::initialize::InitializeRequest>(contents.as_str())
                {
                    Err(e) => {
                        info!("Hey, we couldn't parse this: {}", e);
                        return;
                    }
                    Ok(v) => v,
                };

            if let Some(client_info) = request.params.client_info {
                info!("Connected to: {} {}", client_info.name, client_info.version);
            }
        }
        _ => (),
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
