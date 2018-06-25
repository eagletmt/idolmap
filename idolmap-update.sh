#!/bin/bash
set -ex

git clone git@github.com:eagletmt/idolmap-data /idolmap-data
cd /idolmap-data
idolmap aikatsu update
idolmap prichan update
idolmap lovelive update
git add aikatsu lovelive prichan
if git commit -m "Update CSV data $(date --rfc-3339=seconds)"; then
  git push origin master
fi
