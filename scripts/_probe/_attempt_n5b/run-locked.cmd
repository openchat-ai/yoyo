@echo off
REM scripts/_probe/_attempt_n5b/run-locked.cmd
REM W-START attempt-N5b — run the locked yoyo.ty through the
REM Rust executor and report how far it gets.
setlocal enabledelayedexpansion
pushd "%~dp0\..\..\.." >nul
set REPO=%CD%
popd >nul

set SRC=%REPO%\yoyo\projects\yoyo.ty
set BIN=%REPO%\scripts\_probe\_attempt_n5b\out\yoyo.ty.bin
set STRIPPED=%REPO%\scripts\_probe\_attempt_n5b\out\yoyo.ty.stripped.bin
set REPORT=%REPO%\scripts\_probe\_attempt_n5b\out\yoyo.ty.report.txt

pushd "%REPO%\yoyo-rust" >nul
cargo run -q -p verifier --bin yoyo -- link --target=stub "%SRC%" "%BIN%" > "%REPORT%.tmp" 2>&1
if errorlevel 1 ( type "%REPORT%.tmp" & popd & exit /b 1 )
popd >nul
type "%REPORT%.tmp" > "%REPORT%"
del "%REPORT%.tmp"

powershell -NoProfile -Command "$b=[IO.File]::ReadAllBytes('%BIN%'); [IO.File]::WriteAllBytes('%STRIPPED%', $b[1..($b.Length-1)])" 2>nul
if errorlevel 1 ( echo STRIP-FAILED & exit /b 1 )

pushd "%REPO%\yoyo-rust" >nul
cargo run -q --manifest-path executor/Cargo.toml --bin yoyo-exec-run -- run "%STRIPPED%" > "%REPORT%" 2>&1
set RC=!errorlevel!
popd >nul
echo --- locked yoyo.ty executor exit !RC! ---
type "%REPORT%"
exit /b !RC!
