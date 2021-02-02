#!/usr/bin/env bash
set -euo pipefail

function release-gnulinux {
version=$(stoml Cargo.toml package.version)
dirname="release-${version}"
mkdir -p $dirname
if [[ "$1" == "default" ]]; then
  echo "Compiling orion for x86_64-unknown-linux-gnu"
  cargo build 2> /dev/null
  echo "Archiving orion for x86_64-unknown-linux-gnu"
  cp target/release/orion-cli ${dirname}/orion
  tar -czf ${dirname}/orion-x86_64-unknown-linux-gnu.tar.gz ${dirname}/orion
else
  echo "Compiling orion for ${1}"
  cargo build --release --target $1 2> /dev/null
  echo "Archiving orion for ${1}"
  cp target/${1}/release/orion-cli ${dirname}/orion
  tar -czf ${dirname}/orion-${1}.tar.gz ${dirname}/orion
fi
rm ${dirname}/orion
}

function release-windows {
version=$(stoml Cargo.toml package.version)
dirname="release-${version}"
mkdir -p $dirname
echo "Compiling orion for ${1}"
cargo build --release --target $1 2> /dev/null
echo "Archiving orion for ${1}"
cp target/${1}/release/orion-cli.exe ${dirname}/orion.exe
zip -r ${dirname}/orion-${1}.zip ${dirname}/orion.exe
rm ${dirname}/orion.exe
}

if [[ ! -f Cargo.toml ]]; then
  echo "Cannot find \`Cargo.toml\` in the current folder"
  exit 1
fi

version=$(stoml Cargo.toml package.version)
echo "== Preparing orion release version ${version} =="
echo ""
release-windows x86_64-pc-windows-gnu 
release-windows i686-pc-windows-gnu
release-gnulinux i686-unknown-linux-musl
release-gnulinux default
echo ""
echo "== Finished orion release version ${version} =="
