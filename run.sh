#!/bin/bash

# Function to parse options and extract the file name
function parse_options() {
  local file_name=""
  while getopts ":f:" opt; do
    case "$opt" in
      f) file_name="$OPTARG" ;;
      \?) echo "Invalid option: -$OPTARG"
         exit 1 ;;
    esac
  done
  shift $((OPTIND - 1))

  if [[ -z "$file_name" ]]; then
    echo "Missing file name. Use -f option to specify the file."
    exit 1
  fi
  return 0
}

# Get the file name from the user using options
# $1 is file name
file_name=$1

# Determine the file extension
file_ext="${file_name##*.}"
# Check if the file extension is .ts or .rs
if [[ ! "$file_ext" =~ ^(ts|rs)$ ]]; then
  echo "Invalid file extension. Please enter a file with .ts or .rs extension."
  exit 1
fi

# Check if the file exists
if [[ ! -f "./solutions/$file_name" ]]; then
  echo "File '$file_name' not found."
  exit 1
fi

# Determine the language based on the extension
if [[ "$file_ext" == "ts" ]]; then
  language="typescript"
else
  language="rust"
fi

# Build and run the file based on the language
if [[ "$language" == "typescript" ]]; then
  tsc "./solutions/$file_name" --outDir ./build
  node "./build/${file_name%.*}.js"
elif [[ "$language" == "rust" ]]; then
  rustc --out-dir ./build "./solutions/$file_name"
  ./"build/${file_name%.*}"
fi