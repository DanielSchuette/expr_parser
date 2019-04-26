#!/bin/sh
cargo build || exit 1

if [ "$1" = '' ]; then
    ./target/debug/expr_parser -d -g -e '(18+29) / 50*611+ 41^12' || exit 1

# call binary with erroneous expressions to inspect error messages
elif [ "$1" = 'err1' ]; then
    ./target/debug/expr_parser -e '2123^sdkfj(141+22-(5998)-142'
elif [ "$1" = 'err2' ]; then
    ./target/debug/expr_parser -e '2123^(141+22-(5998)-142sdkfj'
elif [ "$1" = 'err3' ]; then
    ./target/debug/expr_parser -e '223^(11+2429-(542)-11'
fi
