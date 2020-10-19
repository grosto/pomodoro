use std::convert::From;
pub const START: &str = "start";
pub const STOP: &str = "stop";
pub const SHOW: &str = "show";
pub const SET: &str = "set";
pub const SESSION: &str = "session";

#[derive(Debug, PartialEq)]
pub enum Request {
  Show,
  Set(u64), // seconds
  Start,
  Stop,
  Session,
}

impl From<&str> for Request {
  fn from(string: &str) -> Self {
    match string {
      SHOW => Request::Show,
      START => Request::Start,
      STOP => Request::Stop,
      SESSION => Request::Session,
      a => {
        let vec: Vec<&str> = a.split(',').collect();
        if vec[0] != "set" {
          panic!("unknown request")
        };
        let int = vec[1].parse::<u64>().expect("invalid argument to set-time");
        Request::Set(int)
      }
    }
  }
}

impl Into<String> for Request {
  fn into(self) -> String {
    match self {
      Request::Show => String::from(SHOW),
      Request::Set(n) => format!("{},{}", SET, n),
      Request::Start => String::from(START),
      Request::Stop => String::from(STOP),
      Request::Session => String::from(SESSION),
    }
  }
}

#[test]
fn conversion() {
  let show_str: String = Request::Show.into();
  assert_eq!(Request::Show, Request::from(show_str.as_str()));
  let set_str: String = Request::Set(30).into();
  assert_eq!(Request::Set(30), Request::from(set_str.as_str()));
  let start_str: String = Request::Start.into();
  assert_eq!(Request::Start, Request::from(start_str.as_str()));
  let stop_str: String = Request::Stop.into();
  assert_eq!(Request::Stop, Request::from(stop_str.as_str()));
}
