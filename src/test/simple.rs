use std::io::stdio::stdout;
use {get,request};
use super::server;

#[test]
pub fn test_simple_get() {
  let srv = server!(
    send!("FOO"), // Receives
    recv!("BAR")); // Sends

  stdout().write_str("Making request\n");
  let res = get("http://localhost:8482");
  stdout().write_str("Finishing request\n");

  srv.assert();
  stdout().write_str("Finished test\n");
  // assert!(srv.recv());
  // assert!(res.is_success());

  // fail!("nope");
}
