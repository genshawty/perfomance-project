#!/usr/bin/env bash
set -euo pipefail

# Пример сравнения бенчмарков (до/после).
cargo bench --bench criterion > artifacts/criterion_after.txt
# После оптимизаций запустите ещё раз и сохраните в baseline_after.txt
