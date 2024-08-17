#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

cargo run -- --type databases > $OUTDIR/databases1.txt
cargo run -- --type databases --id 12345678-1234-1234-1234-123456789012 > $OUTDIR/databases2.txt