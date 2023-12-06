build DAY:
    cd day-{{DAY}} && \
    cargo build

release DAY:
    cd day-{{DAY}} && \
    cargo build --release

run DAY PART:
    cd day-{{DAY}} && \
    cargo run --bin part{{PART}}

test DAY:
    cd day-{{DAY}} && \
    cargo test

keeptesting DAY:
    cd day-{{DAY}} && \
    cargo watch -x test

create DAY:
    cargo new day-{{DAY}} --vcs none
    cd day-{{DAY}}/src && \
    mkdir bin data && \
    mkdir bin/shared && \
    touch bin/shared/mod.rs && \
    touch data/sample_input.txt && \
    touch data/puzzle_input.txt && \
    touch bin/part1.rs && \
    cargo add test-case --dev
    cp templates/main.rs day-{{DAY}}/src/bin/part1.rs
    cp templates/main.rs day-{{DAY}}/src/bin/part2.rs

delete DAY:
    rm -rf day-{{DAY}}
