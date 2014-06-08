use {get,request};
use super::server;

#[test]
pub fn test_simple_get() {
  let srv = server!(
    send!("FOO"), // Receives
    recv!("BAR")); // Sends

  let res = get("http://localhost:8482");

  // assert!(srv.recv());
  // assert!(res.is_success());

  fail!("nope");
}
