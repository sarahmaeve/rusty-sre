.PHONY: help bootstrap test guides fmt lint check ex hard status solution unsolution \
	verify-solution verify-failures verify-patches verify clean

SHELL := /bin/sh

help:
	@echo "Rusty SRE"
	@echo "  make bootstrap            download locked dependencies for offline use"
	@echo "  make test                 run known-good workspace tests"
	@echo "  make guides               run all twenty concept guides"
	@echo "  make ex N=01              reproduce one exercise (01..48; failure expected)"
	@echo "  make hard N=01            show the symptom-only card"
	@echo "  make status               report all expected exercise failures"
	@echo "  make solution N=01        apply the reference patch"
	@echo "  make unsolution N=01      reverse the reference patch"
	@echo "  make verify-solution N=01 apply, test, and reverse one patch"
	@echo "  make check                format, lint, test, and run guides"
	@echo "  make verify               validate failures and every solution"

bootstrap:
	cargo fetch --locked
	cargo fetch --locked --manifest-path compile-fail/Cargo.toml

test:
	cargo test --workspace --all-targets --offline

guides:
	@set -e; for guide in guides/examples/*.rs; do \
		name=$$(basename "$$guide" .rs); \
		echo "==> guide $$name"; \
		if [ "$$name" = "08_crates" ]; then \
			cargo run --quiet --offline -p rusty-sre-guides --example "$$name" --features telemetry; \
		else \
			cargo run --quiet --offline -p rusty-sre-guides --example "$$name"; \
		fi; \
	done

fmt:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets --all-features --offline -- -D warnings

check: fmt lint test guides

ex:
	@test -n "$(N)" || (echo "set N=01..48"; exit 2)
	@num=$$(printf '%02d' $$(echo "$(N)" | sed 's/^0*//')); \
	case $$num in \
		0[1-8]) package=fleet-core ;; \
		09|1[0-8]) package=ops-core ;; \
		1[9]|2[0-6]) package=async-ops ;; \
		2[7-9]|3[0-9]|40) exec cargo check --offline --manifest-path compile-fail/Cargo.toml -p compile-$$num ;; \
		4[1-4]|4[6-8]) package=advanced-core ;; \
		45) exec cargo test --offline -p advanced-core --features audit --test exercises "exercise_45_" -- --ignored --nocapture ;; \
		*) echo "exercise must be 01..48"; exit 2 ;; \
	esac; \
	cargo test --offline -p $$package --test exercises "exercise_$${num}_" -- --ignored --nocapture

hard:
	@test -n "$(N)" || (echo "set N=01..48"; exit 2)
	@num=$$(printf '%02d' $$(echo "$(N)" | sed 's/^0*//')); \
	awk -F '|' -v num="$$num" '{ \
		key=$$2; gsub(/[[:space:]]/, "", key); \
		if (key == num) { \
			report=$$3; \
			sub(/^[[:space:]]*/, "", report); \
			sub(/[[:space:]]*$$/, "", report); \
			print num " — " report; found=1 \
		} \
	} END {if (!found) exit 1}' exercises/HARD_MODE.md

status:
	@set +e; passed=0; expected=0; wrong=0; \
	for num in $$(seq -w 1 48); do \
		output=$$($(MAKE) --no-print-directory ex N=$$num 2>&1); result=$$?; \
		if [ $$result -eq 0 ]; then \
			echo "$$num  UNEXPECTED PASS"; passed=$$((passed + 1)); \
		else \
			case $$num in \
				27) marker='error[E0382]' ;; \
				28) marker='error[E0502]' ;; \
				29) marker='error[E0106]' ;; \
				30) marker='future cannot be sent between threads safely' ;; \
				31) marker='error[E0277]' ;; \
				32) marker='error[E0004]' ;; \
				33) marker='error[E0621]' ;; \
				34) marker='error[E0310]' ;; \
				35) marker='error[E0038]' ;; \
				36) marker='error[E0191]' ;; \
				37) marker='error[E0005]' ;; \
				38) marker='no rules expected' ;; \
				39) marker='error[E0277]' ;; \
				40) marker='error[E0790]' ;; \
				*) marker="test exercise_$${num}_" ;; \
			esac; \
			if printf '%s\n' "$$output" | grep -Fq "$$marker"; then \
				echo "$$num  expected failure"; expected=$$((expected + 1)); \
			else \
				echo "$$num  WRONG FAILURE"; printf '%s\n' "$$output"; \
				wrong=$$((wrong + 1)); \
			fi; \
		fi; \
	done; \
	echo "$$expected expected failures; $$passed unexpected passes; $$wrong wrong failures"; \
	test $$passed -eq 0 -a $$wrong -eq 0

solution:
	@test -n "$(N)" || (echo "set N=01..48"; exit 2)
	@num=$$(printf '%02d' $$(echo "$(N)" | sed 's/^0*//')); \
	set -- solutions/$$num-*.patch; \
	test -f "$$1" || (echo "no solution patch for $$num"; exit 2); \
	git apply "$$1"; echo "applied $$1"

unsolution:
	@test -n "$(N)" || (echo "set N=01..48"; exit 2)
	@num=$$(printf '%02d' $$(echo "$(N)" | sed 's/^0*//')); \
	set -- solutions/$$num-*.patch; \
	test -f "$$1" || (echo "no solution patch for $$num"; exit 2); \
	git apply -R "$$1"; echo "reversed $$1"

verify-solution:
	@test -n "$(N)" || (echo "set N=01..48"; exit 2)
	@set -e; num=$$(printf '%02d' $$(echo "$(N)" | sed 's/^0*//')); \
	set -- solutions/$$num-*.patch; patch=$$1; \
	test -f "$$patch" || (echo "no solution patch for $$num"; exit 2); \
	git apply --check "$$patch"; \
	git apply "$$patch"; \
	trap 'git apply -R "'"$$patch"'"' EXIT HUP INT TERM; \
	$(MAKE) --no-print-directory ex N=$$num; \
	git apply -R "$$patch"; trap - EXIT HUP INT TERM; \
	echo "$$num  patch round-trip passed"

verify-failures: status

verify-patches:
	@set -e; for num in $$(seq -w 1 48); do \
		$(MAKE) --no-print-directory verify-solution N=$$num; \
	done

verify: check verify-failures verify-patches

clean:
	cargo clean
	cargo clean --manifest-path compile-fail/Cargo.toml
