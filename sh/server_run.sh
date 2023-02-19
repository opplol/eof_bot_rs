echo "KILL PROCESS"
sudo kill `cat ~/eol-bot/release/pid/server.pid`
echo "RUN SERVER"
sudo SLACK_TOKEN=${SLACK_TOKEN} ~/eol-bot/release/eol_bot_rs -d
