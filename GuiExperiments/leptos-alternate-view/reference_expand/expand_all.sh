#!/bin/bash

set -e

cd $(dirname $0)

shopt -s extglob

for file in ./src/examples/!(*.expanded).rs ; do
  echo -e "\nExpanding: $file"

  file_basename=$(basename $file)
  module_name=${file_basename%.rs}

  outfile=${file%.rs}.expanded.rs

  cargo expand examples::${module_name} 1> $outfile
done
