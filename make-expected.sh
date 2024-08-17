#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

cargo run -- databases > $OUTDIR/databases1.txt
cargo run -- databases --id 12345678-1234-1234-1234-123456789012 > $OUTDIR/databases2.txt
cargo run -- databases --x POST --file tests/fixture/post/request1.json > $OUTDIR/databases_post1.txt
cargo run -- qdatabases --id 897e5a76ae524b489fdfe71f5945d1af --file tests/fixture/post/request2.json > $OUTDIR/query_databases_post2.txt