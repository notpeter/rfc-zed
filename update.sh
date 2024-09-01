#!/bin/sh

set -e
cd data
# curl -s -O https://www.ietf.org/rfc/rfc-index.txt

rsync -avz --delete rsync.rfc-editor.org::rfcs-json-only rfcs-json-only
rm -f rfcs-json-only/rfcThis.json rfcs-json-only/8078.json
jq -s '
  flatten |
  map(select(.doc_id != null)) |
  map(select(.pub_date != null)) |
  sort_by(.doc_id) |
  map({
    id: ( .doc_id | (sub("RFC"; "") | tonumber) ),
    date: .pub_date,
    title: ( .title | gsub("^\\s+|\\s+$"; "") )
  })
  ' rfcs-json-only/*.json \
  > ../assets/rfc.json
