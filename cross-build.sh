#!/bin/sh
cross build --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/micronos.exe . &&
exec ./micronos.exe "$@"