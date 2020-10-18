use crate::shared;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

const ADDRESS: &str = "/tmp/rust-uds.sock";

pub fn write_to_server(stream: &mut UnixStream, command: &str) -> String {
  let reader = stream.try_clone().unwrap();
  let mut reader = std::io::BufReader::new(reader);

  writeln!(stream, "{}", command).unwrap();
  stream.flush().unwrap();

  let mut response = String::new();
  reader.read_line(&mut response).unwrap();
  response.trim().to_string()
}

pub fn get_time() {
  let mut stream = UnixStream::connect(ADDRESS).unwrap();

  let time = write_to_server(&mut stream, shared::SHOW);
  let time = time.parse::<i64>().expect("time should be a number");
  let seconds = time % 60;
  let minutes = (time / 60) % 60;
  println!("{:02}:{:02}", minutes, seconds);
}

pub fn start_session() {
  let mut stream = UnixStream::connect(ADDRESS).unwrap();

  write_to_server(&mut stream, shared::START);
}

pub fn stop_session() {
  let mut stream = UnixStream::connect(ADDRESS).unwrap();

  write_to_server(&mut stream, shared::START_SERVER);
}

pub fn set_time() {
  let mut stream = UnixStream::connect(ADDRESS).unwrap();

  write_to_server(&mut stream, "set,10");
}
