#!/bin/bash

set -o errexit -o nounset

cd target/doc

git init
git config user.email 'apaseltiner@gmail.com'
git config user.name 'Andrew Paseltiner'
git remote add upstream "https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git"

touch .

git add -A .
git commit -qm "rebuild pages at ${TRAVIS_TAG}"
git push -q upstream HEAD:gh-pages --force
