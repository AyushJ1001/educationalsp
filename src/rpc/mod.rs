use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub fn encode_message<M>(msg: &M) -> String
where
    M: Serialize,
{
    let content = match serde_json::to_string(msg) {
        Err(e) => panic!("{}", e),
        Ok(v) => v,
    };

    format!("Content-Length: {}\r\n\r\n{}", content.len(), content)
}

#[derive(Serialize, Deserialize)]
struct BaseMessage {
    method: String,
}

pub fn decode_message(msg: String) -> Result<(String, String)> {
    let (header, content) = msg.split_once("\r\n\r\n").ok_or(anyhow!("sep not found"))?;

    let content_length_bytes = &header["Content-Length: ".len()..];
    let content_length = content_length_bytes.parse::<usize>()?;

    let base_message = serde_json::from_str::<BaseMessage>(&content[..content_length])?;

    Ok((base_message.method, content[..content_length].to_string()))
}

pub enum SplitError {
    HeaderEndNotFound,
    InvalidContentLength,
    UnexpectedEof,
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use serde::Serialize;

    #[derive(Serialize)]
    struct EncodingExample {
        testing: bool,
    }

    #[test]
    fn test_encoding() {
        let expected = "Content-Length: 16\r\n\r\n{\"testing\":true}";
        let actual = super::encode_message(&EncodingExample { testing: true });

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_decoding() -> Result<()> {
        let incoming_message = "Content-Length: 15\r\n\r\n{\"method\":\"hi\"}";
        let (method, content) = super::decode_message(incoming_message.to_string())?;
        let content_length = content.len();

        assert_eq!(15, content_length);

        assert_eq!("hi".to_string(), method);

        Ok(())
    }
}
