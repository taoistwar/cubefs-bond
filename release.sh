#!/bin/bash
BASE="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
OLD_DIR=`pwd`


function release() {
  cd $BASE
  cargo build --release
  rm -rf dist/cubefs
  mkdir -p dist/cubefs/bin
  mkdir -p dist/cubefs/conf
  cp build/bin/* dist/cubefs/bin
  cp build/conf/* dist/cubefs/conf
  cp target/release/cubefs-bond dist/cubefs/bin/
  chmod +x dist/cubefs/bin/*
  cd dist/ && tar -zcf release.tar.gz  cubefs
  cd ${OLD_DIR}
  rm -rf dist/cubefs target
}
release