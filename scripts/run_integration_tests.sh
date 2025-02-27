#!/usr/bin/env bash

echo "The integration test output will be logged on this console"
echo "and the Frequency node output will be logged to the file frequency.log."
echo "You can 'tail -f frequency.log' in another terminal to see both side-by-side."
echo ""
echo -e "Checking to see if Frequency is running..."

PID=$(lsof -i tcp:9933 | grep frequency | grep -v grep | xargs | awk '{print $2}')

SHOULD_KILL=false

if [ -z "$PID" ]
then
    make build-local
    echo -e "Starting a Frequency Node..."
    make start >& frequency.log &
    SHOULD_KILL=true
fi

while [ -z "$PID" ]
do
    PID=$(ps aux | grep target/release/frequency | grep -v grep | xargs | awk '{print $2}')
done

echo "---------------------------------------------"
echo "Frequency running here:"
echo "PID: ${PID}"
echo "---------------------------------------------"

cd integration-tests
npm i
WS_PROVIDER_URL="ws://127.0.0.1:9944" npm test

if $SHOULD_KILL
then
   pwd
   ../scripts/kill_freq.sh
fi
