#!/bin/bash

BUILD_DIR=${1:-build}

echo "Removing old mcl lib"
rm -rf lib/*
mkdir -p lib

print_msg "Cloning mcl"
git clone https://github.com/herumi/mcl.git
cd mcl || exit 1

echo "Building mcl"
make -f Makefile MCL_USE_GMP=0

echo "Preparing mcl crate"
cd ..
cp mcl/lib/libmcl.a lib/
cp mcl/lib/libmclbn384_256.a lib/

echo "Cleaning up"
rm -rf mcl/