echo "KILL PROCESS"
sudo kill `cat ~/eof-bot/release/pid/server.pid`
echo "RUN SERVER"
sudo SLACK_TOKEN=${SLACK_TOKEN} ~/eof-bot/release/eof_bot_rs -d
