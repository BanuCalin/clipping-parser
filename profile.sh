#!/bin/bash

OUTPUT="profile"
TEST=0
TOOL="grcov"
FORMAT="html"
RUN_OPTIONS=""
CLEAN=0

while [[ $# -gt 0 ]]; do
  key="$1"
  case $key in
    -o|--output)
      OUTPUT="$2"
      shift # past argument
      shift # past value
      ;;
    -t|--tool)
      TOOL="$2"
      shift # past argument
      shift # past value
      ;;
    -f|--format)
      FORMAT="$2"
      shift # past argument
      shift # past value
      ;;
    -c|--clean)
      CLEAN=1;
      shift
      ;;
    -e|--test)
      CLEAN=1;
      TEST=1;
      shift
      ;;
    --)    # unknown option
      shift # past argument
      while [[ $# -gt 0 ]]; do
        RUN_OPTIONS+="$1 " # save it in an array for later
        shift # past value
      done
      ;;
    *)
      echo "Invalid option: $1"
      exit 1
      ;;
  esac
done

PROFRAW_FILE=$OUTPUT/data.profraw
PROFDATA_FILE=$OUTPUT/data.profdata

export LLVM_PROFILE_FILE=$PROFRAW_FILE
export RUSTFLAGS="-Z instrument-coverage"

if [ $CLEAN -ne 0 ]; then
  cargo +nightly clean
fi

BINARY_NAME=$(cargo +nightly metadata --no-deps --format-version 1 | sed 's/.*,"name":"\([a-zA-Z0-9_-]*\)","src_path":.*/\1/')

if [[ "$TOOL" == "cov" ]]; then
  if [ $TEST -eq 1 ]; then
    cargo +nightly test
  else
    cargo +nightly test -- $RUN_OPTIONS
  fi

  cargo +nightly profdata -- merge -sparse $PROFRAW_FILE -o $PROFDATA_FILE
  cargo +nightly cov -- show \
    -Xdemangler=rustfilt \
    -instr-profile=$PROFDATA_FILE \
    -output-dir=$OUTPUT \
    -format=$FORMAT \
    -ignore-filename-regex='.*\.cargo.*' \
    -show-instantiations \
    -show-expansions \
    -show-line-counts-or-regions \
    target/debug/$BINARY_NAME
  rm -f $PROFRAW_FILE
  rm -f $PROFDATA_FILE
elif [[ "$TOOL" == "grcov" ]]; then
  if [ $TEST -eq 1 ]; then
    cargo +nightly test
  else
    cargo +nightly test -- $RUN_OPTIONS
  fi

  if [[ "$FORMAT" == "lcov" ]]; then
    OUTPUT=$OUTPUT/lcov.info
  fi

  if [[ "$FORMAT" == "cobertura" ]]; then
    OUTPUT=$OUTPUT/cobertura.info
  fi

  grcov $PROFRAW_FILE --binary-path target/debug/ -s . -t $FORMAT --branch --ignore-not-existing -o $OUTPUT
  rm -f $PROFRAW_FILE
else
  echo "Invalid type: $TOOL"
  exit 1
fi
