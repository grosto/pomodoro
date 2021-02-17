#!/bin/sh

echo "building release"
cargo build --release
echo "installing binary in path"
cp ./target/release/pomodoro /usr/local/bin/
echo "installing daemon"
cp ./com.grosto.pomodoro.plist ~/Library/LaunchAgents
echo "launching pomodoro server daemon"
launchctl unload ~/Library/LaunchAgents/com.grosto.pomodoro.plist
launchctl load ~/Library/LaunchAgents/com.grosto.pomodoro.plist

