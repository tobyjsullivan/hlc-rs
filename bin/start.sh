#!/bin/bash

main() {
  local data_dir="$1"
  local data_file="$2"
  local prog="$3"

  echo "making data dir $data_dir"
  mkdir -p "$data_dir"
  echo "unzipping"
  unzip "$data_file" -d "$data_dir"
  echo "launching"
  DATA_DIR="$data_dir" "$prog"
}

main "$@"
