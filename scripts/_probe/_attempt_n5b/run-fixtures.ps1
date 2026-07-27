# scripts/_probe/_attempt_n5b/run-fixtures.ps1
# W-START attempt-N5b — run a list of golden fixtures through the
# executor. Prints one block per fixture.
$ErrorActionPreference = "Continue"
$repo = (Resolve-Path "$PSScriptRoot\..\..\..").Path
$list = @(
  '00_nop_ret',
  '01_set_get',
  '02_addv_orv',
  '03_cmp_je',
  '04_call_ret',
  'selfhost_min_inc',
  'selfhost_min_dec',
  'selfhost_min_jmp',
  'selfhost_min_call',
  'selfhost_min_je',
  'selfhost_min_jcc_all',
  'selfhost_min_io',
  'selfhost_min_ldb',
  'selfhost_min_ldb_off8',
  'selfhost_min_ldb_off127',
  'selfhost_min_ldb_off128',
  'selfhost_min_ldb_off256',
  'selfhost_min_ldb_offm128',
  'selfhost_min_ldb_offm129'
)
foreach ($f in $list) {
  Write-Host "=== $f ==="
  $out = cmd /c "scripts\_probe\_attempt_n5b\run-fixture.cmd $f" 2>&1
  $out | ForEach-Object { Write-Host $_ }
  Write-Host "----"
}
