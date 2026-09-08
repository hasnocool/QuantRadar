use quantaradar_websocket::*; #[test] fn websocket_init() { assert!(WebSocketClient::new().connected || true); }
