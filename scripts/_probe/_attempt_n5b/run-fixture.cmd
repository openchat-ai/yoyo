@echo off
REM scripts/_probe/_attempt_n5b/run-fixture.cmd
REM W-START attempt-N5b — run a golden fixture from yoyo/tests/golden
REM through the executor to test wider opcode coverage.
setlocal enabledelayedexpansion
pushd "%~dp0\..\..\.." >nul
set REPO=%CD%
popd >nul

set NAME=%1
if "%NAME%"=="" set NAME=selfhost_min_io
set SRC=%REPO%\yoyo\tests\golden\%NAME%.ty
set BIN=%REPO%\scripts\_probe\_attempt_n5b\out\%NAME%.bin
set STRIPPED=%REPO%\scripts\_probe\_attempt_n5b\out\%NAME%.stripped.bin
set REPORT=%REPO%\scripts\_probe\_attempt_n5b\out\%NAME%.report.txt

if not exist "%SRC%" ( echo missing %SRC% & exit /b 2 )

pushd "%REPO%\yoyo-rust" >nul
cargo run -q -p verifier --bin yoyo -- link --target=stub "%SRC%" "%BIN%" > nul 2>&1
if errorlevel 1 ( popd & echo %NAME% VERIFIER-FAILED & exit /b 1 )
popd >nul

powershell -NoProfile -Command "$b=[IO.File]::ReadAllBytes('%BIN%'); [IO.File]::WriteAllBytes('%STRIPPED%', $b[1..($b.Length-1)])" 2>nul
if errorlevel 1 ( echo %NAME% STRIP-FAILED & exit /b 1 )

pushd "%REPO%\yoyo-rust" >nul
cargo run -q --manifest-path executor/Cargo.toml --bin yoyo-exec-run -- run "%STRIPPED%" > "%REPORT%" 2>&1
set RC=!errorlevel!
popd >nul
echo === %NAME% ===
type "%REPORT%"
echo --- %NAME% executor exit !RC! ---
exit /b !RC!
