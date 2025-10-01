#!/bin/bash

# Array of test case numbers
tests=(1 2 3 4 5)

# Loop through each test case
for i in "${tests[@]}"; do
  input="test${i}.input"
  expected="test${i}.output"

  echo -n "Test $i: "

  # Compare program output to expected output
  if cargo run "$input" | diff -u - "$expected" > /dev/null; then
    echo "✅ Passed"
  else
    echo "❌ Failed"
  fi
done
