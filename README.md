# eol_slack_bot with rust_actix

[![CI](https://github.com/opplol/eof_bot_rs/actions/workflows/ci.yml/badge.svg)](https://github.com/opplol/eof_bot_rs/actions/workflows/ci.yml)

SlackBotを利用して、ライブラリーのEOL（End Of Life）を確認できる。
EOL情報は以下のAPIから取得している。
https://endoflife.date/

# DEMO

WIP

# Features
## Slackでのメッセージ
`@EOL_BOT {Library name}`<br />
ex) `@EOF_BOT {rails}`<br />
Will return EOL info
![image](https://user-images.githubusercontent.com/15142826/221346155-2cdc6aab-30f8-489f-8ada-4a035568b5ba.png)

`@EOL_BOT {Similar name}`<br />
ex) `@EOF_BOT {amazon}`<br />
Will return suggest library names
![image](https://user-images.githubusercontent.com/15142826/221346235-a3fdad40-61b5-4165-99a8-641dc74fa77a.png)

# Requirement

- Production
  * Os：Linux
    実行可能なバイナリーファイルをデプロイする。
- Dev
  * Docker
  * VsCode

# Installation For Dev

git clone git@github.com:opplol/eof_bot_rs.git eof_bot_rs_tut

cd eof_bot_rs_tut

code .


# Usage
## Prepare for Slack

WIP


## Run Web Server
docker-compose up<br />

docker-compose exec -it -e SLACK-TOKEN={slack-oauth-token} rust  cargo run -- -p 8080

# Note

こちらのプロジェクトはRust学習のため作成したプロジェクトです。

# Author
* opplol(Lee Sang Ho)
* colra03@gmail.com

# License
"eol_slack_bot" is under [MIT license](https://en.wikipedia.org/wiki/MIT_License).

# Thanks
Thank [endoflife-date](https://endoflife.date/) for your wonderful API
