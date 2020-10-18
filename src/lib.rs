pub mod client;
mod pomodoro;
mod shared;

use std::io::prelude::*;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn handle_client_request(mut stream: UnixStream, pomodoro: Arc<Mutex<pomodoro::Pomodoro>>) {
  println!("starting reading");
  let reader = stream.try_clone().unwrap();
  let reader = std::io::BufReader::new(reader);

  for response in reader.lines() {
    let response = response.unwrap();

    if response == "start" {
      let mut pomodoro = pomodoro.lock().unwrap();
      pomodoro.start_session();
      return;
    }
    if response == "stop" {
      let mut pomodoro = pomodoro.lock().unwrap();
      pomodoro.stop_session();
      return;
    }
    if response == "show" {
      let pomodoro = pomodoro.lock().unwrap();
      let payload = pomodoro.get_time_remaining().as_secs();
      writeln!(stream, "{}", payload).unwrap();
      stream.flush().unwrap();
      return;
    }
    let int = response.parse::<u64>().unwrap();
    let mut pomodoro = pomodoro.lock().unwrap();
    pomodoro.set_time_remaining(Duration::from_secs(int));
    let payload = format!("time left {}", pomodoro.get_time_remaining().as_secs());
    writeln!(stream, "{}", payload).unwrap();
    stream.flush().unwrap();
  }
}

const PATH: &str = "/tmp/rust-uds.sock";

fn start_tick(pomodoro: &Arc<Mutex<pomodoro::Pomodoro>>) {
  let pomodoro_clone = Arc::clone(pomodoro);

  thread::spawn(move || loop {
    thread::sleep(pomodoro::TICK_INTERVAL);
    let mut pomodoro = pomodoro_clone.lock().expect("lock acquired");
    pomodoro.tick();
  });
}

pub fn start_timer_server() {
  std::fs::remove_file(PATH).unwrap_or_else(|e| match e.kind() {
    std::io::ErrorKind::NotFound => (),
    _ => panic!("{}", e),
  });

  let listener = UnixListener::bind(PATH).unwrap();

  let pomodoro = pomodoro::Pomodoro::new(Default::default());

  let pomodoro = Arc::new(Mutex::new(pomodoro));

  start_tick(&pomodoro);

  for stream in listener.incoming() {
    let pomodoro = Arc::clone(&pomodoro);

    match stream {
      Ok(stream) => {
        thread::spawn(|| handle_client_request(stream, pomodoro));
      }
      Err(err) => {
        println!("Error: {}", err);
        break;
      }
    }
  }
}
