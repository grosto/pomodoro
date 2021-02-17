use anyhow::bail;
use std::convert::{From, TryFrom};

pub const START: &str = "start";
pub const STOP: &str = "stop";
pub const GET: &str = "get";
pub const SET: &str = "set";
pub const SESSION: &str = "session";
pub const NEXT_SESSION: &str = "next-session";
pub const RESET_ROUNDS: &str = "reset-rounds";

#[derive(Debug, PartialEq)]
pub enum Request {
  Get,
  Set(u64), // seconds
  Start,
  Stop,
  Session,
  NextSession(bool),
  ResetRounds,
}

impl TryFrom<&str> for Request {
  type Error = anyhow::Error;

  fn try_from(string: &str) -> Result<Self, Self::Error> {
    Ok(match string {
      GET => Request::Get,
      START => Request::Start,
      STOP => Request::Stop,
      SESSION => Request::Session,
      RESET_ROUNDS => Request::ResetRounds,
      a => {
        let vec: Vec<&str> = a.split(',').collect();
        if vec[0] == SET {
          let int = vec[1].parse::<u64>().expect("invalid argument to set-time");
          Request::Set(int)
        } else if vec[0] == NEXT_SESSION {
          let no_start = vec[1]
            .parse::<bool>()
            .expect("invalid argument to next-session");
          Request::NextSession(no_start)
        } else {
          bail!("Wrong string")
        }
      }
    })
  }
}

impl From<Request> for String {
  fn from(t: Request) -> Self {
    match t {
      Request::Get => String::from(GET),
      Request::Set(n) => format!("{},{}", SET, n),
      Request::Start => String::from(START),
      Request::Stop => String::from(STOP),
      Request::Session => String::from(SESSION),
      Request::NextSession(no_start) => format!("{},{}", NEXT_SESSION, no_start),
      Request::ResetRounds => RESET_ROUNDS.to_string(),
    }
  }
}

#[test]
fn conversion() {
  let show_str: String = Request::Get.into();
  assert_eq!(Request::Get, Request::try_from(show_str.as_str()).unwrap());
  let set_str: String = Request::Set(30).into();
  assert_eq!(
    Request::Set(30),
    Request::try_from(set_str.as_str()).unwrap()
  );
  let start_str: String = Request::Start.into();
  assert_eq!(
    Request::Start,
    Request::try_from(start_str.as_str()).unwrap()
  );
  let stop_str: String = Request::Stop.into();
  assert_eq!(Request::Stop, Request::try_from(stop_str.as_str()).unwrap());
}
