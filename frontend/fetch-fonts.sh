#!/bin/sh
# Downloads Satoshi from Fontshare into assets/fonts/satoshi/.
#
# Satoshi's license (ITF Free Font License) lets us self-host it on our own
# site but not redistribute it, and a public repository counts as
# redistribution. So the files are git-ignored and fetched at build time.
set -eu
cd "$(dirname "$0")"
dest=assets/fonts/satoshi
[ -f "$dest/Satoshi-Variable.woff2" ] && exit 0
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
curl -sSfL -o "$tmp/satoshi.zip" https://api.fontshare.com/v2/fonts/download/satoshi
unzip -q "$tmp/satoshi.zip" -d "$tmp"
mkdir -p "$dest"
cp "$tmp"/Satoshi_Complete/Fonts/WEB/fonts/Satoshi-Variable.woff2 \
   "$tmp"/Satoshi_Complete/Fonts/WEB/fonts/Satoshi-VariableItalic.woff2 \
   "$tmp"/Satoshi_Complete/License/FFL.txt "$dest/"
echo "Satoshi downloaded to frontend/$dest"
