set windows-powershell := true

run year day part:
    cargo run --release --package "file:///{{invocation_directory()}}/{{year}}/day{{day}}" --bin "{{part}}"

gen year day:
    cargo generate --path ./template --name day{{day}} --define day={{day}} --define year={{year}} --vcs none
    mv day{{day}} ./{{year}}/day{{day}}