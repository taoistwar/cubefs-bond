#!/bin/bash
BASE="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
OLD_DIR=`pwd`



function release() {
  cd $BASE
  VERSION=`cat Cargo.toml|grep '^version ='|cut -d = -f 2|cut -d '"' -f 2`
  cargo build --release
  rm -rf dist/cubefs
  mkdir -p dist/cubefs/bin
  mkdir -p dist/cubefs/conf
  cp build/bin/* dist/cubefs/bin
  cp build/conf/* dist/cubefs/conf
  cp target/release/cubefs-bond dist/cubefs/bin/
  chmod +x dist/cubefs/bin/*

  cd dist/
  mv cubefs cubefs-bond-${VERSION}
  tar -zcf cubefs-bond-${VERSION}.tar.gz cubefs-bond-${VERSION}
  rm -rf cubefs-bond-${VERSION}
  cd ${OLD_DIR}
}


release