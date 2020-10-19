use mac_notification_sys::send_notification;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub enum Session {
  Focus = 1,
  ShortBreak = 2,
  LongBreak = 3,
}

pub struct Pomodoro {
  rounds: u32,
  current_session: Session,
  time_remaining: Duration,
  is_running: bool,
  long_break_duration: Duration,
  short_break_duration: Duration,
  focus_duration: Duration,
  should_notify: bool,
}

pub struct PomodoroConfig {
  pub is_running: bool,
  pub long_break_duration: Duration,
  pub short_break_duration: Duration,
  pub focus_duration: Duration,
  pub initial_session: Session,
  pub should_notify: bool,
}

impl Default for PomodoroConfig {
  fn default() -> Self {
    Self {
      is_running: false,
      long_break_duration: Duration::from_secs(25 * 60),
      short_break_duration: Duration::from_secs(5 * 60),
      focus_duration: Duration::from_secs(25 * 60),
      initial_session: Session::Focus,
      should_notify: false,
    }
  }
}

pub const TICK_INTERVAL: std::time::Duration = Duration::from_secs(1);

impl Pomodoro {
  pub fn new(config: PomodoroConfig) -> Pomodoro {
    let mut pomodoro = Pomodoro {
      rounds: 0,
      time_remaining: Duration::from_secs(0),
      current_session: config.initial_session,
      is_running: config.is_running,
      long_break_duration: config.long_break_duration,
      short_break_duration: config.short_break_duration,
      focus_duration: config.focus_duration,
      should_notify: config.should_notify,
    };

    pomodoro.set_time_remaining(pomodoro.get_initial_time_for_session(&pomodoro.current_session));
    pomodoro
  }

  fn get_initial_time_for_session(&self, session: &Session) -> Duration {
    match session {
      Session::Focus => self.focus_duration,
      Session::LongBreak => self.long_break_duration,
      Session::ShortBreak => self.short_break_duration,
    }
  }

  pub fn tick(&mut self) {
    if !self.get_is_running() {
      return;
    }

    if self.time_remaining.as_secs() == 0 {
      self.stop_session();
      let old_session = &self.current_session.clone();
      match &self.current_session {
        Session::Focus => {
          self.rounds += 1;
          if self.rounds % 4 == 0 {
            self.set_session(Session::LongBreak);
          } else {
            self.set_session(Session::ShortBreak);
          }
        }
        _ => {
          self.set_session(Session::Focus);
        }
      }

      self.set_time_remaining(self.get_initial_time_for_session(&self.current_session));

      if self.should_notify {
        if cfg!(target_os = "macos") {
          notify(create_notification_config_for_session(old_session));
        }
      }
    } else {
      self.set_time_remaining(self.time_remaining - TICK_INTERVAL);
    }
  }

  pub fn get_is_running(&self) -> bool {
    self.is_running
  }

  fn set_is_running(&mut self, is_running: bool) {
    self.is_running = is_running;
  }

  fn set_session(&mut self, session: Session) {
    self.current_session = session;
  }

  pub fn get_session(&mut self) -> &Session {
    &self.current_session
  }

  pub fn start_session(&mut self) {
    self.set_is_running(true)
  }

  pub fn stop_session(&mut self) {
    self.set_is_running(false)
  }

  pub fn get_time_remaining(&self) -> Duration {
    self.time_remaining
  }

  pub fn set_time_remaining(&mut self, time_remaining: Duration) {
    self.time_remaining = time_remaining;
  }
}

#[cfg(test)]
mod pomodoro_struct {
  use super::*;
  #[test]
  fn can_set_time_manually() {
    let mut pomodoro = Pomodoro::new(PomodoroConfig {
      is_running: true,
      ..Default::default()
    });

    pomodoro.set_time_remaining(Duration::from_secs(4));
    pomodoro.tick();
    assert_eq!(pomodoro.get_time_remaining().as_secs(), 3);
  }

  #[test]
  fn stops_session_on_zero() {
    let mut pomodoro = Pomodoro::new(PomodoroConfig {
      is_running: true,
      focus_duration: TICK_INTERVAL,
      ..Default::default()
    });
    pomodoro.tick();
    assert_eq!(pomodoro.get_is_running(), false);
  }
  #[test]
  fn increments_round_on_zero() {
    let mut pomodoro = Pomodoro::new(PomodoroConfig {
      is_running: true,
      focus_duration: TICK_INTERVAL,
      ..Default::default()
    });
    pomodoro.tick();
    assert_eq!(pomodoro.rounds, 1);
  }
  #[test]
  fn switches_to_short_break_after_focus() {
    let mut pomodoro = Pomodoro::new(PomodoroConfig {
      is_running: true,
      focus_duration: TICK_INTERVAL,
      ..Default::default()
    });
    pomodoro.tick();
    assert_eq!(pomodoro.current_session, Session::ShortBreak);
  }

  #[test]
  fn switches_to_focus_after_short_break() {
    let mut pomodoro = Pomodoro::new(PomodoroConfig {
      is_running: true,
      focus_duration: TICK_INTERVAL,
      short_break_duration: TICK_INTERVAL,
      ..Default::default()
    });
    pomodoro.tick();
    assert_eq!(pomodoro.current_session, Session::ShortBreak);
    pomodoro.start_session();
    pomodoro.tick();
    assert_eq!(pomodoro.current_session, Session::Focus);
  }

  #[test]
  fn switches_to_long_break_after_4_rounds() {
    let mut pomodoro = Pomodoro::new(PomodoroConfig {
      is_running: true,
      short_break_duration: TICK_INTERVAL,
      focus_duration: TICK_INTERVAL,
      ..Default::default()
    });
    pomodoro.tick();
    assert_eq!(pomodoro.current_session, Session::ShortBreak);
    assert_eq!(pomodoro.rounds, 1);
    pomodoro.start_session();
    pomodoro.tick();
    assert_eq!(pomodoro.current_session, Session::Focus);
    pomodoro.start_session();
    pomodoro.tick();
    assert_eq!(pomodoro.current_session, Session::ShortBreak);
    assert_eq!(pomodoro.rounds, 2);
    pomodoro.start_session();
    pomodoro.tick();
    assert_eq!(pomodoro.current_session, Session::Focus);
    pomodoro.start_session();
    pomodoro.tick();
    assert_eq!(pomodoro.current_session, Session::ShortBreak);
    assert_eq!(pomodoro.rounds, 3);
    pomodoro.start_session();
    pomodoro.tick();
    assert_eq!(pomodoro.current_session, Session::Focus);
    pomodoro.start_session();
    pomodoro.tick();
    assert_eq!(pomodoro.current_session, Session::LongBreak);
    assert_eq!(pomodoro.rounds, 4);
  }
}

fn create_notification_config_for_session(session: &Session) -> NotificationConfig {
  match session {
    Session::Focus => NotificationConfig {
      title: "Focus Session Ended",
      body: "Take a break",
    },
    Session::LongBreak => NotificationConfig {
      title: "Long Break Ended",
      body: "Get back To work",
    },
    Session::ShortBreak => NotificationConfig {
      title: "Short Break Ended",
      body: "Get back To work",
    },
  }
}

struct NotificationConfig<'a> {
  title: &'a str,
  body: &'a str,
}

#[cfg(target_os = "macos")]
fn notify(config: NotificationConfig) {
  send_notification(config.title, &None, config.body, &None).ok();
}
