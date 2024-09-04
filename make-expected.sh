#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

# cargo run -- databases > $OUTDIR/databases1.txt
# cargo run -- databases --id 12345678-1234-1234-1234-123456789012 > $OUTDIR/databases2.txt
# cargo run -- databases --x POST --file tests/fixture/post/request1.json > $OUTDIR/databases_post1.txt
# cargo run -- query_databases --id 897e5a76ae524b489fdfe71f5945d1af --file tests/fixture/post/request2.json > $OUTDIR/query_databases_post2.txt
# cargo run -- databases -x PATCH -i 668d797c-76fa-4934-9b05-ad288df2d136 -f tests/fixture/patch/request1.json > $OUTDIR/databases_patch1.txt
# cargo run -- pages -x POST -f tests/fixture/post/create_request1.json > $OUTDIR/pages_create1.txt
# cargo run -- pages -i 12345678-1234-1234-1234-123456789012  > $OUTDIR/pages_retrieve1.txt
# cargo run -- property_pages -i 12345678-1234-1234-1234-123456789012 -p aBcd123  > $OUTDIR/property_pages_retrieve1.txt
# cargo run -- pages -x PATCH -i 12345678-1234-1234-1234-123456789012 -f tests/fixture/patch/pages_patch_request1.json > $OUTDIR/pages_patch1.txt
# cargo run -- append_blocks -x PATCH -i 12345678-1234-1234-1234-123456789012 -f tests/fixture/patch/append_blocks_request1.json > $OUTDIR/append_blocks1.txt
# cargo run -- blocks -x PATCH -i 12345678-1234-1234-1234-123456789012 -f tests/fixture/patch/blocks_update_request1.json > $OUTDIR/blocks_update1.txt
# cargo run -- blocks -x DELETE -i 12345678-1234-1234-1234-123456789012 > $OUTDIR/bloks_delete1.txt
# cargo run -- blocks -i 12345678-1234-1234-1234-123456789012 > $OUTDIR/blocks_ret1.txt
# cargo run -- children_blocks -i 12345678-1234-1234-1234-123456789012 > $OUTDIR/child_blocks.txt
# cargo run -- search -f tests/fixture/post/search_by_title_request1.json > $OUTDIR/search_by_title.txt