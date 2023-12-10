set dotenv-load := true
export RUST_MIN_STACK := "8388608"

run day='':
    cargo run --bin "day$(just _day {{ day }})"

bench day='':
    cargo run --release --bin "day$(just _day {{ day }})"

test day='':
    RUST_BACKTRACE=1 cargo test --bin "day$(just _day {{ day }})"

watch day='':
    cargo watch -s "cargo test --bin \"day$(just _day {{ day }})\""

prepare day='':
    #! /bin/sh

    set -eu

    day="$(just _day {{ day }})"
    filename="day${day}"

    [ -d "input" ] || mkdir input

    curl --fail --cookie "session=${SESSION_COOKIE:?Session cookie unavailable}" "https://adventofcode.com/2023/day/$((10#${day}))/input" > "input/${filename}.txt"
    git add "input/${filename}.txt"
    git ci -m "Day ${day}"

@_day day='':
    if [ -n "{{ day }}" ]; then \
        printf "%02d" "{{ day }}"; \
    else \
        date "+%d"; \
    fi
