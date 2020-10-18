use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = if args.len() == 1 {
        "show"
    } else {
        args[1].as_str()
    };

    match command {
        "start-server" => pomodoro::start_timer_server(),
        "show" => pomodoro::client::get_time(),
        "start" => pomodoro::client::start_session(),
        "stop" => pomodoro::client::stop_session(),
        _ => panic!("unknown error"),
    }
}
