$ErrorActionPreference = "Stop"
$root = "F:\yoyo"
$yoyoJs = "node $root\yoyo-js\src\yoyo.js"
$yoyoRust = "$root\yoyo-rust\target\release\yoyo.exe"
$tmpDir = "$root\yoyo-js\build"

# Build a function to test a handler subset
function Test-Subset {
    param($label, $handlers)
    $ty = "40 00`n  30 50 00`n  FF`n$handlers"
    $path = "$root\yoyo\tests\golden\_parity_temp.ty"
    Set-Content -LiteralPath $path -Value $ty -Encoding ascii
    $jsOut = Invoke-Expression "& node $root\yoyo-js\src\yoyo.js $path $tmpDir\_ptemp_js.exe 2>&1"
    $rustOut = & $yoyoRust link --target=win32 $path $tmpDir\_ptemp_rust.exe 2>&1
    $jsCode = ($jsOut -split 'code=')[1] -split ',' | Select-Object -First 1
    $rustCode = ($rustOut -match 'code bytes' | Select-String '\d+').Matches.Value
    if (-not $rustCode) {
        $rustCode = ($rustOut -split '→ ')[1] -split ' code bytes' | Select-Object -First 1
    }
    Write-Host "$label`: JS code=$jsCode Rust code=$rustCode diff=$([int]$rustCode - [int]$jsCode)"
    $diffOut = & $yoyoRust diff $tmpDir\_ptemp_js.exe $tmpDir\_ptemp_rust.exe 2>&1
    $diffLine = ($diffOut | Select-String "EQUAL|DIFFER|DIFF").Line
    Write-Host "  DDC: $diffLine"
    Remove-Item $path -ErrorAction SilentlyContinue
}

# H_17 INC, H_18 DEC
Test-Subset "INC+DEC" @"
40 11
  66 50
  FF
40 12
  67 50
  FF
"@

# Add JMP H_00
Test-Subset "+JMP" @"
40 11
  66 50
  FF
40 12
  67 50
  FF
40 13
  70 00
  FF
"@

# Add CALL H_00
Test-Subset "+CALL" @"
40 11
  66 50
  FF
40 12
  67 50
  FF
40 13
  70 00
  FF
40 14
  41 00
  FF
"@

# Add JE
Test-Subset "+JE" @"
40 11
  66 50
  FF
40 12
  67 50
  FF
40 13
  70 00
  FF
40 14
  41 00
  FF
40 15
  30 50 00
  30 51 00
  65 50 51
  71 00
  FF
"@

# Add I/O handlers
Test-Subset "+I/O" @"
40 11
  66 50
  FF
40 12
  67 50
  FF
40 13
  70 00
  FF
40 14
  41 00
  FF
40 15
  30 50 00
  30 51 00
  65 50 51
  71 00
  FF
40 1F
  20 50 1000
  FF
40 20
  50 50 00
  FF
40 21
  51 50 00 51
  FF
"@
