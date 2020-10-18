function prompt_pomodoro() {
  local content 
  content=$(~/projects/pomodoro/target/debug/pomodoro show 2>/dev/null) || return
  p10k segment -f 208 -i '🍅' -t "$content"
}
