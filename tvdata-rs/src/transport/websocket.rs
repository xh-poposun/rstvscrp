use std::sync::atomic::{AtomicU64, Ordering};

use futures_util::SinkExt;
use serde_json::{Value, json};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
#[cfg(feature = "tracing")]
use tracing::{debug, warn};

use crate::client::Endpoints;
use crate::error::{Error, Result};

pub type TradingViewWebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub(crate) async fn connect_socket(
    endpoints: &Endpoints,
    user_agent: &str,
    session_id: Option<&str>,
) -> Result<TradingViewWebSocket> {
    #[cfg(feature = "tracing")]
    debug!(
        target: "tvdata_rs::transport",
        url = endpoints.websocket_url().as_str(),
        authenticated = session_id.is_some(),
        "opening TradingView websocket",
    );

    let mut ws_request = endpoints
        .websocket_url()
        .as_str()
        .into_client_request()
        .map_err(Error::from)?;
    ws_request.headers_mut().insert(
        "Origin",
        endpoints
            .data_origin()
            .as_str()
            .parse()
            .map_err(|_| Error::Protocol("failed to encode websocket origin header"))?,
    );
    ws_request.headers_mut().insert(
        "User-Agent",
        user_agent
            .parse()
            .map_err(|_| Error::Protocol("failed to encode websocket user agent header"))?,
    );
    if let Some(session_id) = session_id {
        ws_request.headers_mut().insert(
            "Cookie",
            format!("sessionid={session_id}")
                .parse()
                .map_err(|_| Error::Protocol("failed to encode websocket cookie header"))?,
        );
    }

    match connect_async(ws_request).await {
        Ok((socket, _)) => {
            #[cfg(feature = "tracing")]
            debug!(
                target: "tvdata_rs::transport",
                url = endpoints.websocket_url().as_str(),
                authenticated = session_id.is_some(),
                "TradingView websocket connected",
            );
            Ok(socket)
        }
        Err(error) => {
            #[cfg(feature = "tracing")]
            warn!(
                target: "tvdata_rs::transport",
                url = endpoints.websocket_url().as_str(),
                authenticated = session_id.is_some(),
                error = %error,
                "TradingView websocket connection failed",
            );
            Err(Error::from(error))
        }
    }
}

pub(crate) fn next_session_id(prefix: &str) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}_{id:016x}")
}

pub(crate) async fn send_message(
    socket: &mut TradingViewWebSocket,
    method: &str,
    params: Value,
) -> Result<()> {
    let payload = serde_json::to_string(&json!({ "m": method, "p": params }))?;
    #[cfg(feature = "tracing")]
    debug!(
        target: "tvdata_rs::transport",
        method,
        params = %params,
        "sending message"
    );
    send_raw_frame(socket, payload).await
}

pub(crate) async fn send_raw_frame(
    socket: &mut TradingViewWebSocket,
    payload: String,
) -> Result<()> {
    #[cfg(feature = "wss-debug")]
    if crate::transport::debug_log::is_enabled() {
        crate::transport::debug_log::log_send("", "", &payload);
    }
    let framed = format!("~m~{}~m~{payload}", payload.len());
    socket.send(Message::Text(framed.into())).await?;
    Ok(())
}

pub(crate) fn parse_framed_messages(frame: &str) -> Result<Vec<&str>> {
    #[cfg(feature = "tracing")]
    debug!(
        target: "tvdata_rs::transport",
        frame_len = frame.len(),
        "parsing framed message"
    );
    let mut rest = frame;
    let mut payloads = Vec::new();

    while !rest.is_empty() {
        if let Some(next) = rest.strip_prefix("~m~") {
            let Some((len, tail)) = next.split_once("~m~") else {
                #[cfg(feature = "tracing")]
                warn!(
                    target: "tvdata_rs::transport",
                    "missing frame length"
                );
                rest = "";
                continue;
            };
            let len: usize = match len.parse() {
                Ok(l) => l,
                Err(_) => {
                    #[cfg(feature = "tracing")]
                    warn!(
                        target: "tvdata_rs::transport",
                        "invalid frame length"
                    );
                    rest = "";
                    continue;
                }
            };
            if tail.len() < len {
                #[cfg(feature = "tracing")]
                warn!(
                    target: "tvdata_rs::transport",
                    expected = len,
                    actual = tail.len(),
                    "frame length > payload"
                );
                rest = "";
                continue;
            }
            let (payload, remainder) = tail.split_at(len);
            #[cfg(feature = "tracing")]
            debug!(
                target: "tvdata_rs::transport",
                payload_len = payload.len(),
                "parsed frame payload"
            );
            payloads.push(payload);
            rest = remainder;
            continue;
        }

        if let Some((_, remainder)) = rest.split_once("~m~") {
            rest = remainder;
            continue;
        }

        rest = "";
    }

    Ok(payloads)
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio_tungstenite::tungstenite::handshake::derive_accept_key;

    use crate::client::Endpoints;

    use super::*;

    async fn read_upgrade_request(stream: &mut TcpStream) -> String {
        let mut request = Vec::new();
        let mut buffer = [0_u8; 1024];

        loop {
            let read = stream.read(&mut buffer).await.unwrap();
            assert_ne!(read, 0, "client closed connection before websocket upgrade");
            request.extend_from_slice(&buffer[..read]);
            if request.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }

        String::from_utf8(request).unwrap()
    }

    fn header_value<'a>(request: &'a str, name: &str) -> Option<&'a str> {
        request.lines().find_map(|line| {
            let (header, value) = line.split_once(':')?;
            header
                .trim()
                .eq_ignore_ascii_case(name)
                .then_some(value.trim())
        })
    }

    #[test]
    fn parses_concatenated_websocket_frames() {
        let frames = parse_framed_messages("~m~9~m~{\"m\":\"a\"}~m~9~m~{\"m\":\"b\"}").unwrap();

        assert_eq!(frames, vec![r#"{"m":"a"}"#, r#"{"m":"b"}"#]);
    }

    #[tokio::test]
    async fn connect_socket_includes_session_cookie_when_configured() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let cookie = Arc::new(Mutex::new(None::<String>));
        let cookie_clone = Arc::clone(&cookie);

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let request = read_upgrade_request(&mut stream).await;
            *cookie_clone.lock().unwrap() = header_value(&request, "cookie").map(str::to_owned);

            let key = header_value(&request, "sec-websocket-key")
                .expect("websocket upgrade request must contain Sec-WebSocket-Key");
            let response = format!(
                "HTTP/1.1 101 Switching Protocols\r\n\
                 Connection: Upgrade\r\n\
                 Upgrade: websocket\r\n\
                 Sec-WebSocket-Accept: {}\r\n\
                 \r\n",
                derive_accept_key(key.as_bytes())
            );

            stream.write_all(response.as_bytes()).await.unwrap();
        });

        let endpoints = Endpoints::default()
            .with_websocket_url(format!("ws://{address}"))
            .unwrap();

        let _socket = connect_socket(&endpoints, "tvdata-rs/test", Some("abc123"))
            .await
            .unwrap();

        assert_eq!(cookie.lock().unwrap().as_deref(), Some("sessionid=abc123"));
    }

    #[tokio::test]
    async fn send_raw_frame_sends_correct_framed_message() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let received_frame = Arc::new(Mutex::new(None::<String>));
        let received_frame_clone = Arc::clone(&received_frame);

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let request = read_upgrade_request(&mut stream).await;

            let key = header_value(&request, "sec-websocket-key")
                .expect("websocket upgrade request must contain Sec-WebSocket-Key");
            let response = format!(
                "HTTP/1.1 101 Switching Protocols\r\n\
                 Connection: Upgrade\r\n\
                 Upgrade: websocket\r\n\
                 Sec-WebSocket-Accept: {}\r\n\
                 \r\n",
                derive_accept_key(key.as_bytes())
            );

            stream.write_all(response.as_bytes()).await.unwrap();

            let mut buffer = vec![0u8; 4096];
            let n = stream.read(&mut buffer).await.unwrap();
            if n >= 6 {
                let payload_len = (buffer[1] & 0x7F) as usize;
                let (mask_key, payload_start) = match payload_len {
                    126 => {
                        if n >= 8 {
                            ([buffer[4], buffer[5], buffer[6], buffer[7]], 8)
                        } else {
                            return;
                        }
                    }
                    127 => {
                        if n >= 14 {
                            ([buffer[10], buffer[11], buffer[12], buffer[13]], 14)
                        } else {
                            return;
                        }
                    }
                    _ => ([buffer[2], buffer[3], buffer[4], buffer[5]], 6),
                };

                let actual_len = if payload_len == 126 {
                    u16::from_be_bytes([buffer[2], buffer[3]]) as usize
                } else if payload_len == 127 {
                    u64::from_be_bytes([
                        buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7],
                        buffer[8], buffer[9],
                    ]) as usize
                } else {
                    payload_len
                };

                if n >= payload_start + actual_len {
                    let mut payload = buffer[payload_start..payload_start + actual_len].to_vec();
                    for (i, byte) in payload.iter_mut().enumerate() {
                        *byte ^= mask_key[i % 4];
                    }
                    if let Ok(s) = String::from_utf8(payload) {
                        *received_frame_clone.lock().unwrap() = Some(s);
                    }
                }
            }
        });

        let endpoints = Endpoints::default()
            .with_websocket_url(format!("ws://{address}"))
            .unwrap();

        let mut socket = connect_socket(&endpoints, "tvdata-rs/test", None)
            .await
            .unwrap();

        let test_payload = r#"{"m":"quote_add_symbols","p":["NASDAQ:AAPL"]}"#;
        send_raw_frame(&mut socket, test_payload.to_string())
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let received = received_frame.lock().unwrap();
        assert!(received.is_some(), "server should have received a frame");
        let frame = received.as_ref().unwrap();
        let expected = format!("~m~{}~m~{}", test_payload.len(), test_payload);
        assert_eq!(frame, &expected);
    }

    #[test]
    fn parse_framed_messages_with_single_frame() {
        let frame = "~m~19~m~{\"m\":\"test\",\"p\":[]}";
        let parsed = parse_framed_messages(frame).unwrap();
        assert_eq!(parsed, vec![r#"{"m":"test","p":[]}"#]);
    }

    #[test]
    fn parse_framed_messages_with_multiple_frames() {
        let frame = "~m~9~m~{\"m\":\"a\"}~m~9~m~{\"m\":\"b\"}~m~9~m~{\"m\":\"c\"}";
        let parsed = parse_framed_messages(frame).unwrap();
        assert_eq!(parsed, vec![r#"{"m":"a"}"#, r#"{"m":"b"}"#, r#"{"m":"c"}"#]);
    }

    #[test]
    fn parse_framed_messages_with_empty_payload() {
        let frame = "~m~0~m~";
        let parsed = parse_framed_messages(frame).unwrap();
        assert_eq!(parsed, vec![""]);
    }

    #[test]
    fn parse_framed_messages_ignores_invalid_length() {
        let frame = "~m~abc~m~{\"m\":\"test\"}";
        let parsed = parse_framed_messages(frame).unwrap();
        assert!(parsed.is_empty());
    }

    #[test]
    fn parse_framed_messages_handles_truncated_payload() {
        let frame = "~m~100~m~short";
        let parsed = parse_framed_messages(frame).unwrap();
        assert!(parsed.is_empty());
    }

    #[cfg(feature = "wss-debug")]
    #[test]
    fn debug_logging_does_not_affect_parse_framed_messages() {
        let frame = "~m~9~m~{\"m\":\"a\"}~m~9~m~{\"m\":\"b\"}";
        let parsed = parse_framed_messages(frame).unwrap();
        assert_eq!(parsed, vec![r#"{"m":"a"}"#, r#"{"m":"b"}"#]);
    }

    #[cfg(feature = "wss-debug")]
    #[tokio::test]
    async fn send_raw_frame_with_debug_logging_enabled() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let received_frame = Arc::new(Mutex::new(None::<String>));
        let received_frame_clone = Arc::clone(&received_frame);

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let request = read_upgrade_request(&mut stream).await;

            let key = header_value(&request, "sec-websocket-key")
                .expect("websocket upgrade request must contain Sec-WebSocket-Key");
            let response = format!(
                "HTTP/1.1 101 Switching Protocols\r\n\
                 Connection: Upgrade\r\n\
                 Upgrade: websocket\r\n\
                 Sec-WebSocket-Accept: {}\r\n\
                 \r\n",
                derive_accept_key(key.as_bytes())
            );

            stream.write_all(response.as_bytes()).await.unwrap();

            let mut buffer = vec![0u8; 4096];
            let n = stream.read(&mut buffer).await.unwrap();
            if n >= 6 {
                let payload_len = (buffer[1] & 0x7F) as usize;
                let (mask_key, payload_start) = match payload_len {
                    126 => {
                        if n >= 8 {
                            ([buffer[4], buffer[5], buffer[6], buffer[7]], 8)
                        } else {
                            return;
                        }
                    }
                    127 => {
                        if n >= 14 {
                            ([buffer[10], buffer[11], buffer[12], buffer[13]], 14)
                        } else {
                            return;
                        }
                    }
                    _ => ([buffer[2], buffer[3], buffer[4], buffer[5]], 6),
                };

                let actual_len = if payload_len == 126 {
                    u16::from_be_bytes([buffer[2], buffer[3]]) as usize
                } else if payload_len == 127 {
                    u64::from_be_bytes([
                        buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7],
                        buffer[8], buffer[9],
                    ]) as usize
                } else {
                    payload_len
                };

                if n >= payload_start + actual_len {
                    let mut payload = buffer[payload_start..payload_start + actual_len].to_vec();
                    for (i, byte) in payload.iter_mut().enumerate() {
                        *byte ^= mask_key[i % 4];
                    }
                    if let Ok(s) = String::from_utf8(payload) {
                        *received_frame_clone.lock().unwrap() = Some(s);
                    }
                }
            }
        });

        let endpoints = Endpoints::default()
            .with_websocket_url(format!("ws://{address}"))
            .unwrap();

        let mut socket = connect_socket(&endpoints, "tvdata-rs/test", None)
            .await
            .unwrap();

        let test_payload = r#"{"m":"quote_add_symbols","p":["NASDAQ:AAPL"]}"#;
        send_raw_frame(&mut socket, test_payload.to_string())
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let received = received_frame.lock().unwrap();
        assert!(received.is_some(), "server should have received a frame");
        let frame = received.as_ref().unwrap();
        let expected = format!("~m~{}~m~{}", test_payload.len(), test_payload);
        assert_eq!(frame, &expected);
    }
}
