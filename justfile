set windows-powershell := true

run year day part:
    cargo run --release --package "file:///C:/Users/Ilias/Code/advent-of-code/{{year}}/{{day}}" --bin "{{part}}"

gen year day:
    cargo generate --path ./template --name {{day}} --vcs none
    mv {{day}} ./{{year}}/{{day}}