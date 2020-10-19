use crate::request::Request;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

pub const SOCKET_PATH: &str = "/tmp/pomodoro.sock";

pub fn write_to_server(stream: &mut UnixStream, command: Request) -> String {
  let reader = stream.try_clone().unwrap();
  let mut reader = std::io::BufReader::new(reader);

  let command: String = command.into();
  writeln!(stream, "{}", command.as_str()).unwrap();
  stream.flush().unwrap();

  let mut response = String::new();
  reader.read_line(&mut response).unwrap();
  response.trim().to_string()
}

pub fn get_time() {
  let mut stream = UnixStream::connect(SOCKET_PATH).unwrap();

  let time = write_to_server(&mut stream, Request::Show);
  let time = time.parse::<i64>().expect("time should be a number");
  let seconds = time % 60;
  let minutes = (time / 60) % 60;
  println!("{:02}:{:02}", minutes, seconds);
}

pub fn start_session() {
  let mut stream = UnixStream::connect(SOCKET_PATH).unwrap();

  write_to_server(&mut stream, Request::Start);
}

pub fn stop_session() {
  let mut stream = UnixStream::connect(SOCKET_PATH).unwrap();

  write_to_server(&mut stream, Request::Stop);
}

pub fn set_time(time_in_seconds: u64) {
  let mut stream = UnixStream::connect(SOCKET_PATH).unwrap();

  write_to_server(&mut stream, Request::Set(time_in_seconds));
}
