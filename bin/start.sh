#!/usr/bin/env bash

main() {
  local data_dir="$1"
  local data_file="$2"
  local prog="$3"
  uname -s
  uname -m
  ldd --version

  mkdir -p "$data_dir"
  unzip "$data_file" -d "$data_dir"
  DATA_DIR="$data_dir" "$prog"
}

main "$@"
