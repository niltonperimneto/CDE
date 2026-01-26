#!/bin/sh
# $1 is the install directory (bin)

BINDIR="$DESTDIR/$1"
cd "$BINDIR" || exit 1

for tool in dtopen_image dtopen_pdf dtopen_ps dtopen_video; do
  rm -f "$tool"
  ln -s dtopen "$tool"
done
