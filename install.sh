#!/usr/bin/env bash
set -eu

cLibVersion=0.18.0
os=$(uname)

# if there's no tty this is probably part of a docker build - therefore we install the c-api explicitly
cLibArgs=
if [[ "$os" != MINGW* ]] && [[ "$os" != CYGWIN* ]]; then
  tty -s || cLibArgs="${cLibArgs} --install"
fi

bash <(curl -s https://raw.githubusercontent.com/objectbox/objectbox-c/main/download.sh) ${cLibArgs} ${cLibVersion}
