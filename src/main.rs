use anyhow::{Context, Result};
use clap::{App, Arg, SubCommand};
use pomodoro::request::Request;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

const SHOW_COMMAND: &str = "show";
const START_COMMNAND: &str = "start";
const STOP_COMMAND: &str = "stop";
const SET_COMMNAND: &str = "set";
const RUN_SERVER_COMMAND: &str = "run-server";
const GET_SESSION: &str = "get-session";
const NEXT_SESSION: &str = "next";
const RESET_ROUNDS: &str = "reset";

const SOCKET_PATH: &str = "/tmp/pomodoro.sock";

fn send_request(stream: &mut UnixStream, command: Request) -> String {
  let reader = stream.try_clone().unwrap();
  let mut reader = std::io::BufReader::new(reader);

  let command: String = command.into();
  writeln!(stream, "{}", command.as_str()).unwrap();
  stream.flush().unwrap();

  let mut response = String::new();
  reader.read_line(&mut response).unwrap();
  response.trim().to_string()
}

fn print_time(mut stream: &mut UnixStream) {
  let response = send_request(&mut stream, Request::Get);
  let mut iter = response.split(",");

  let time = iter
    .next()
    .expect("time missing from server resposne")
    .parse::<u64>()
    .expect("time should be a number");

  let round = iter.next().expect("rounds missing from server response");

  let seconds = time % 60;
  let minutes = (time / 60) % 60;
  println!("{:02}:{:02} | {}", minutes, seconds, round);
}

fn main() -> Result<()> {
  let matches = App::new("Over-engineered Pomodoro")
    .version("0.4.2")
    .subcommand(SubCommand::with_name(RUN_SERVER_COMMAND).about("Run pomodoro server"))
    .subcommand(SubCommand::with_name(SHOW_COMMAND).about("Prints remaining time and round"))
    .subcommand(SubCommand::with_name(START_COMMNAND).about("Starts pomodoro session"))
    .subcommand(SubCommand::with_name(STOP_COMMAND).about("Stops pomodoro session"))
    .subcommand(
      SubCommand::with_name(SET_COMMNAND)
        .about("Sets remaning time in pomodoro session")
        .arg(Arg::with_name("time").takes_value(true).required(true)),
    )
    .subcommand(SubCommand::with_name(GET_SESSION).about("Prints pomodoro session"))
    .subcommand(SubCommand::with_name(RESET_ROUNDS).about("Reset pomodoro rounds"))
    .subcommand(
      SubCommand::with_name(NEXT_SESSION)
        .about("Start next pomodoro session")
        .arg(
          Arg::with_name("no-start")
            .long("no-start")
            .short("n")
            .takes_value(false),
        ),
    )
    .get_matches();

  if let Some(_) = matches.subcommand_matches(RUN_SERVER_COMMAND) {
    pomodoro::start_pomodoro_server();
    return Ok(());
  }

  let mut stream = UnixStream::connect(SOCKET_PATH).context("connection refused")?;

  if let Some(_) = matches.subcommand_matches(SHOW_COMMAND) {
    print_time(&mut stream)
  }

  if let Some(_) = matches.subcommand_matches(STOP_COMMAND) {
    send_request(&mut stream, Request::Stop);
  }

  if let Some(_) = matches.subcommand_matches(START_COMMNAND) {
    send_request(&mut stream, Request::Start);
  }

  if let Some(args) = matches.subcommand_matches(SET_COMMNAND) {
    let val = args.value_of("time").unwrap();
    let val = val
      .parse::<u64>()
      .context(format!("was expecing a number for time got {}", val))?;

    send_request(&mut stream, Request::Set(val * 60));
  }

  if let Some(_) = matches.subcommand_matches(GET_SESSION) {
    let session = send_request(&mut stream, Request::Session);
    println!("{}", session);
  }

  if let Some(_) = matches.subcommand_matches(RESET_ROUNDS) {
    send_request(&mut stream, Request::ResetRounds);
  }

  if let Some(args) = matches.subcommand_matches(NEXT_SESSION) {
    let no_start = args.is_present("no-start");
    send_request(&mut stream, Request::NextSession(no_start));
  }

  Ok(())
}
