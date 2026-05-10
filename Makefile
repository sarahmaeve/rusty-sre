# Rusty SRE — single-command builds for every challenge.
#
# Each challenge is a set of standalone .rs files compiled with `rustc --test`.
# This Makefile discovers challenges under challenges/ and builds them into
# .build/<challenge>/ so the source directories stay clean.
#
# Usage:
#   make                # alias for `make test`
#   make test           # run all known-passing tests (concepts + solutions)
#   make test-concepts  # run all concept.rs test suites
#   make test-solutions # run all solution/debug_solution.rs test suites
#   make test-debug     # run all debug.rs (expected to FAIL until fixed)
#   make test-skeletons # run all skeleton.rs (expected to FAIL until filled in)
#   make CH=11_result_and_question_mark concept   # run one file in one challenge
#   make clean          # remove .build/

EDITION    := 2024
RUSTC      := rustc --edition $(EDITION) --test
BUILD      := .build
CHALLENGES := $(notdir $(wildcard challenges/[0-9]*))

.PHONY: all help test test-concepts test-solutions test-debug test-skeletons clean \
        concept skeleton debug solution

all: test

help:
	@echo "Rusty SRE — targets:"
	@echo "  test            run concepts + solutions (everything expected to pass)"
	@echo "  test-concepts   run every concept.rs"
	@echo "  test-solutions  run every solution/debug_solution.rs"
	@echo "  test-debug      run every debug.rs (expected to fail until bugs are fixed)"
	@echo "  test-skeletons  run every skeleton.rs (expected to fail until TODOs are done)"
	@echo "  clean           remove $(BUILD)/"
	@echo ""
	@echo "Run a single file (set CH= to the directory name under challenges/):"
	@echo "  make CH=01_vectors concept"
	@echo "  make CH=11_result_and_question_mark skeleton"
	@echo "  make CH=11_result_and_question_mark debug"
	@echo "  make CH=11_result_and_question_mark solution"
	@echo ""
	@echo "Discovered challenges:"
	@for ch in $(CHALLENGES); do echo "  - $$ch"; done

test: test-concepts test-solutions

# ---------------------------------------------------------------------------
# Bulk targets
# ---------------------------------------------------------------------------

test-concepts:
	@set -e; for ch in $(CHALLENGES); do \
		src=challenges/$$ch/concept.rs; \
		[ -f $$src ] || continue; \
		echo "==> $$ch / concept"; \
		mkdir -p $(BUILD)/$$ch; \
		$(RUSTC) $$src -o $(BUILD)/$$ch/concept; \
		$(BUILD)/$$ch/concept; \
	done

test-solutions:
	@set -e; for ch in $(CHALLENGES); do \
		src=challenges/$$ch/solution/debug_solution.rs; \
		[ -f $$src ] || continue; \
		echo "==> $$ch / solution"; \
		mkdir -p $(BUILD)/$$ch; \
		$(RUSTC) $$src -o $(BUILD)/$$ch/debug_solution; \
		$(BUILD)/$$ch/debug_solution; \
	done

test-debug:
	@for ch in $(CHALLENGES); do \
		src=challenges/$$ch/debug.rs; \
		[ -f $$src ] || continue; \
		echo "==> $$ch / debug (expected to fail until bugs are fixed)"; \
		mkdir -p $(BUILD)/$$ch; \
		$(RUSTC) $$src -o $(BUILD)/$$ch/debug 2>&1 || true; \
		[ -x $(BUILD)/$$ch/debug ] && $(BUILD)/$$ch/debug || true; \
	done

test-skeletons:
	@for ch in $(CHALLENGES); do \
		src=challenges/$$ch/skeleton.rs; \
		[ -f $$src ] || continue; \
		echo "==> $$ch / skeleton (expected to fail until TODOs are done)"; \
		mkdir -p $(BUILD)/$$ch; \
		$(RUSTC) $$src -o $(BUILD)/$$ch/skeleton 2>&1 || true; \
		[ -x $(BUILD)/$$ch/skeleton ] && $(BUILD)/$$ch/skeleton || true; \
	done

# ---------------------------------------------------------------------------
# Single-challenge targets — set CH=<dirname>
# ---------------------------------------------------------------------------

concept:
	@test -n "$(CH)" || (echo "set CH=<challenge-dir>, e.g. CH=01_vectors" && exit 2)
	@mkdir -p $(BUILD)/$(CH)
	$(RUSTC) challenges/$(CH)/concept.rs -o $(BUILD)/$(CH)/concept
	$(BUILD)/$(CH)/concept

skeleton:
	@test -n "$(CH)" || (echo "set CH=<challenge-dir>" && exit 2)
	@mkdir -p $(BUILD)/$(CH)
	$(RUSTC) challenges/$(CH)/skeleton.rs -o $(BUILD)/$(CH)/skeleton
	$(BUILD)/$(CH)/skeleton

debug:
	@test -n "$(CH)" || (echo "set CH=<challenge-dir>" && exit 2)
	@mkdir -p $(BUILD)/$(CH)
	$(RUSTC) challenges/$(CH)/debug.rs -o $(BUILD)/$(CH)/debug
	$(BUILD)/$(CH)/debug

solution:
	@test -n "$(CH)" || (echo "set CH=<challenge-dir>" && exit 2)
	@mkdir -p $(BUILD)/$(CH)
	$(RUSTC) challenges/$(CH)/solution/debug_solution.rs -o $(BUILD)/$(CH)/debug_solution
	$(BUILD)/$(CH)/debug_solution

clean:
	rm -rf $(BUILD)
