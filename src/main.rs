use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = if args.len() == 1 {
        "show"
    } else {
        args[1].as_str()
    };

    match command {
        "start-server" => pomodoro::start_pomodoro_server(),
        "show" => pomodoro::client::get_time(),
        "start" => pomodoro::client::start_session(),
        "stop" => pomodoro::client::stop_session(),
        "set" => pomodoro::client::set_time(
            args[2]
                .parse::<u64>()
                .expect("second argument of set must be a number")
                * 60,
        ),
        _ => panic!("unknown error"),
    }
}
