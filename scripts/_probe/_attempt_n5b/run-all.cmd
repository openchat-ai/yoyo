@echo off
REM scripts/_probe/_attempt_n5b/run-all.cmd
REM W-START attempt-N5b — run all three N5 canaries through the
REM Rust executor. Outputs one block per canary to stdout.
setlocal enabledelayedexpansion
pushd "%~dp0\..\..\.." >nul
set REPO=%CD%
popd >nul

for %%C in (canary-A canary-B canary-C) do (
  echo === %%C ===
  pushd "%REPO%\yoyo-rust" >nul
  cargo run -q -p verifier --bin yoyo -- link --target=stub "%REPO%\scripts\_probe\_attempt_n5b\canaries\%%C.ty" "%REPO%\scripts\_probe\_attempt_n5b\out\%%C.bin" 2>nul
  if errorlevel 1 ( echo %%C VERIFIER-FAILED & popd & exit /b 1 )
  popd >nul
  REM Strip the leading 0xC3 startup byte.
  powershell -NoProfile -Command "$b=[IO.File]::ReadAllBytes('%REPO%\scripts\_probe\_attempt_n5b\out\%%C.bin'); [IO.File]::WriteAllBytes('%REPO%\scripts\_probe\_attempt_n5b\out\%%C.stripped.bin', $b[1..($b.Length-1)])" 2>nul
  if errorlevel 1 ( echo %%C STRIP-FAILED & exit /b 1 )
  pushd "%REPO%\yoyo-rust" >nul
  cargo run -q --manifest-path executor/Cargo.toml --bin yoyo-exec-run -- run "%REPO%\scripts\_probe\_attempt_n5b\out\%%C.stripped.bin" 2>nul
  set RC=!errorlevel!
  popd >nul
  echo --- %%C executor exit !RC! ---
)
exit /b 0
