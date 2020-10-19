function prompt_pomodoro() {
  local content session f=208 
  content=$(pomodoro show 2>/dev/null) || return
  session=$(pomodoro session 2>/dev/null) || ""
  if [[ $session != "1" ]]; then
    f=101
  fi;
  p10k segment -f $f -i '🍅' -t "$content"
}
