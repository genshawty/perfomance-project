.PHONY: valgrind miri flamegraph

reports:
	mkdir -p reports

valgrind: reports
	cargo test --tests --no-run --message-format=json 2>/dev/null \
		| jq -r 'select(.executable != null) | .executable' \
		| while read bin; do \
			echo "=== Running valgrind on $$bin ==="; \
			ulimit -n 1024 && valgrind --leak-check=full --show-leak-kinds=all "$$bin" 2>&1; \
		done > reports/valgrind.txt
	@echo "Valgrind report saved to reports/valgrind.txt"

flamegraph: reports
	cargo flamegraph --package broken-app --bin demo -o reports/flamegraph.svg
	@echo "Flamegraph saved to reports/flamegraph.svg"

miri: reports
	cargo +nightly miri test > reports/miri.txt 2>&1
	@echo "Miri report saved to reports/miri.txt"
