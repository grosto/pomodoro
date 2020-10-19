pub mod client;
mod pomodoro;
mod request;

use request::Request;
use std::io::prelude::*;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn handle_client_request(mut stream: UnixStream, pomodoro: Arc<Mutex<pomodoro::Pomodoro>>) {
  let reader = stream.try_clone().expect("failed to copy unix stream");
  let reader = std::io::BufReader::new(reader);

  for request in reader.lines() {
    let request = request.expect("failed to read request");

    let request: Request = Request::from(request.as_str());
    let paylod = match request {
      Request::Start => {
        let mut pomodoro = pomodoro.lock().expect("failed to acquire lock");
        pomodoro.start_session();
        String::from("")
      }
      Request::Stop => {
        let mut pomodoro = pomodoro.lock().expect("failed to acquire lock");
        pomodoro.stop_session();
        String::from("")
      }
      Request::Show => {
        let pomodoro = pomodoro.lock().expect("failed to acquire lock");
        let payload = pomodoro.get_time_remaining().as_secs();
        format!("{}", payload)
      }
      Request::Set(n) => {
        let mut pomodoro = pomodoro.lock().expect("failed to acquire lock");
        pomodoro.set_time_remaining(Duration::from_secs(n));
        format!("{}", pomodoro.get_time_remaining().as_secs())
      }
    };

    writeln!(stream, "{}", paylod).expect("failed to write to stream");
    stream.flush().expect("failed to flush stream");
  }
}

pub const SOCKET_PATH: &str = "/tmp/pomodoro.sock";

fn start_tick(pomodoro: &Arc<Mutex<pomodoro::Pomodoro>>) {
  let pomodoro_clone = Arc::clone(pomodoro);

  thread::spawn(move || loop {
    thread::sleep(pomodoro::TICK_INTERVAL);
    let mut pomodoro = pomodoro_clone.lock().expect("lock acquired");
    pomodoro.tick();
  });
}

pub fn start_pomodoro_server() {
  std::fs::remove_file(SOCKET_PATH).unwrap_or_else(|e| match e.kind() {
    std::io::ErrorKind::NotFound => (),
    _ => panic!("{}", e),
  });

  let listener = UnixListener::bind(SOCKET_PATH).expect("failed to bind to socket");
  println!("Server starting listening");

  let pomodoro = pomodoro::Pomodoro::new(pomodoro::PomodoroConfig {
    should_notify: true,
    ..Default::default()
  });

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
