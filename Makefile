# Usage:
#   make run year=2024 day=05 part=part1
#   make gen year=2024 day=05

SHELL := /bin/sh

# Safe clear: use tput if available; otherwise do nothing
clear_cmd = if command -v tput >/dev/null 2>&1; then tput reset || tput clear || true; else true; fi

# Validate required vars
req = if [ -z "$($(1))" ]; then echo "$(1)= is required"; exit 1; fi

# Normalize zero-padded day (optional)
pad2 = $(shell printf "%02d" $(1))

# Run a specific binary "part" inside the crate at ./<year>/day<day>
# Uses --manifest-path to point cargo at the right Cargo.toml.
run:
	@$(call req,year); $(call req,day); $(call req,part); \
	$(clear_cmd); \
	dayp=$$(printf "%02d" "$(day)"); \
	mp="./$${year}/day$${dayp}/Cargo.toml"; \
	if [ ! -f "$$mp" ]; then echo "No Cargo.toml at $$mp"; exit 1; fi; \
	exec cargo run --release --manifest-path "$$mp" --bin "$(part)"

# Generate a new sub-crate from ./template into ./<year>/day<day>
gen:
	@$(call req,year); $(call req,day); \
	dayp=$$(printf "%02d" "$(day)"); \
	name="day$${dayp}"; \
	cargo generate --path ./template --name "$$name" --define day="$$dayp" --define year="$(year)" --vcs none && \
	mkdir -p "./$(year)" && \
	mv "$$name" "./$(year)/$$name"