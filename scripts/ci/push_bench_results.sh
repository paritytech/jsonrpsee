#!/bin/bash
# The script takes output.txt, removes every line that doesn't have "test"
# in it and pushes benchmark result to Victoria Metrics
# Benchmark name should have underscores in the name instead of spaces (e.g. async/http_concurrent_round_trip/8)

RESULT_FILE=$1
CURRENT_DIR=$(pwd)

if [ -z "$RESULT_FILE" ]
then
  RESULT_FILE="output.txt"
fi

grep test "${RESULT_FILE}" > "${CURRENT_DIR}"/output_redacted.txt

INPUT="output_redacted.txt"

while IFS= read -r line
do
  BENCH_NAME=$(awk '{ print $2 }' <<< "${line}")
  BENCH_RESULT=$(awk '{ print $5 }' <<< "${line}")
   
  # send metric with common results
  echo 'parity_benchmark_common_result_ns{project="'${CI_PROJECT_NAME}'",benchmark="'${BENCH_NAME}'"} '${BENCH_RESULT}''
  echo 'parity_benchmark_common_result_ns{project="'${CI_PROJECT_NAME}'",benchmark="'${BENCH_NAME}'"} '${BENCH_RESULT}'' \
    | curl --silent --data-binary @- "https://pushgateway.parity-build.parity.io/metrics/job/${BENCH_NAME}"

  # send metric with detailed results
  echo 'parity_benchmark_specific_result_ns{project="'${CI_PROJECT_NAME}'",benchmark="'${BENCH_NAME}'",commit="'${CI_COMMIT_SHORT_SHA}'",cirunner="'${RUNNER_NAME}'"} '${BENCH_RESULT}''
  echo 'parity_benchmark_specific_result_ns{project="'${CI_PROJECT_NAME}'",benchmark="'${BENCH_NAME}'",commit="'${CI_COMMIT_SHORT_SHA}'",cirunner="'${RUNNER_NAME}'"} '${BENCH_RESULT}'' \
    | curl --silent --data-binary @- "https://pushgateway.parity-build.parity.io/metrics/job/${BENCH_NAME}"


done < "${INPUT}"
