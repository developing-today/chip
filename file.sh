#!/bin/bash
commitHash=$(curl https://raw.githubusercontent.com/rust-lang-nursery/rust-toolstate/master/history/linux.tsv 2>/dev/null |
	sed 1d | grep '"rls":"test-pass"' | grep '"miri":"test-pass"' |
	grep '"rust-by-example":"test-pass"' | grep '"reference":"test-pass"' |
	head -n1 | cut -d$'\t' -f1)

commitDate=$(curl -X GET "https://api.github.com/repos/rust-lang/rust/commits/$commitHash" 2>/dev/null |
	python -c "import sys,json; print json.load(sys.stdin)['commit']['author']['date']")

date --date="$commitDate" --iso-8601=date
