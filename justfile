run YEAR DAY:
    cargo run --bin aoc-{{YEAR}}-{{DAY}}

@run-all:
    cargo build --workspace -q
    echo "Running all..."
    echo "-------------------------------------"
    awk '/members = \[/,/\]/ {if ($0 !~ /\[|\]/) print $0}' Cargo.toml | tr -d '", ' | while IFS=/ read -r year day; do \
        echo "Running $year Day $day"; \
        cargo run -q --bin aoc-$year-$day; \
        echo "-------------------------------------"; \
    done
    echo "All done!"
