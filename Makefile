# The default make command.
DEFAULT = help

# Use 'VERBOSE=1' to echo all commands, for example 'make help VERBOSE=1'.
ifdef VERBOSE
  Q :=
else
  Q := @
endif

.PHONY: \
		clean \
		run \
		test \
		bench \
		watch

all: $(DEFAULT)

help:
	$(Q)echo "Dev CLI - v0.0.1"
	$(Q)echo "make run               - Runs main executable"
	$(Q)echo "make exe               - Builds main executable"
	$(Q)echo "make test              - Tests all crates"
	$(Q)echo "make lint              - Runs clippy and formatter"
	$(Q)echo "make fmt               - Runs formatter"
	$(Q)echo "make bench             - Benchmarks all crates"
	$(Q)echo "make watch             - Runs main executable in hot-reloading mode for development"
	$(Q)echo "make clean             - Deletes binaries and documentation."

clean:
	$(Q)cargo clean
	$(Q)echo "--- Deleted binaries and documentation"

run:
	# TODO: consider replacing with env aware script instead
	$(Q)sh infra/scripts/run_test_node.sh
	$(Q)echo "--- Done"

test:
	$(Q)sh infra/scripts/run_tests.sh
	$(Q)echo "--- Executed tests on all crates"

lint:
	$(Q)sh infra/scripts/run_lints.sh
	$(Q)echo "--- Ran all lints and formatters"

fmt:
	$(Q)cargo +nightly fmt --all 
	$(Q)echo "--- Ran formatter"

watch: 
	$(Q)sh infra/scripts/watch.sh
	$(Q)echo "--- Done"
