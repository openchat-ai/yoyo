# stage-truer17.ps1 — AUTO-TRUER17 E2E (parse1 sem batch3: ADDV/ORV/SUBV/IMUL)
# Note: AUTO-TRUER17 deepen knife — NOT Stage 17 / not invent STAGE17 checklist.
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER17 E2E ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-no-sidecar-face.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=sidecar"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-h00-bake-zero.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=h00_clean"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-prefab-bins-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=prefab"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-assemble.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=assemble"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-compact-code.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=compact"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=plant_sot"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-semantic-parse1.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=sem_sot"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer17\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

$slices = @(
  @{ Name = "ret"; Tyb = "yoyo-sem-ret.tyb" },
  @{ Name = "nop"; Tyb = "yoyo-sem-nop.tyb" },
  @{ Name = "sub"; Tyb = "yoyo-sem-sub.tyb" },
  @{ Name = "add"; Tyb = "yoyo-sem-add.tyb" },
  @{ Name = "get"; Tyb = "yoyo-sem-get.tyb" },
  @{ Name = "set"; Tyb = "yoyo-sem-set.tyb" },
  @{ Name = "mov"; Tyb = "yoyo-sem-mov.tyb" },
  @{ Name = "cmp"; Tyb = "yoyo-sem-cmp.tyb" },
  @{ Name = "inc"; Tyb = "yoyo-sem-inc.tyb" },
  @{ Name = "dec"; Tyb = "yoyo-sem-dec.tyb" },
  @{ Name = "addv"; Tyb = "yoyo-sem-addv.tyb" },
  @{ Name = "orv"; Tyb = "yoyo-sem-orv.tyb" },
  @{ Name = "subv"; Tyb = "yoyo-sem-subv.tyb" },
  @{ Name = "imul"; Tyb = "yoyo-sem-imul.tyb" }
)

foreach ($s in $slices) {
  $d = Join-Path $Root ("scripts\_stage-truer17\sem-" + $s.Name)
  if (Test-Path $d) { Remove-Item -Recurse -Force $d }
  New-Item -Force -ItemType Directory $d | Out-Null
  Copy-Item $c1 (Join-Path $d "yoyo_compiler.exe")
  Copy-Item (Join-Path $Root ("yoyo\projects\" + $s.Tyb)) (Join-Path $d "input.tyb")
  $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $d "yoyo_compiler.exe") -WorkingDirectory $d -TimeoutSec 120
  $out = Join-Path $d "output.exe"
  if ($ec -ne 0 -or -not (Test-Path $out)) {
    Write-Host ("TRUER17 status=RED reason=sem_compile name={0} exit={1}" -f $s.Name, $ec)
    exit 1
  }
  $mz = [IO.File]::ReadAllBytes($out)[0..1]
  if ($mz[0] -ne 0x4D -or $mz[1] -ne 0x5A) {
    Write-Host ("TRUER17 status=RED reason=sem_not_mz name={0}" -f $s.Name)
    exit 1
  }
  Write-Host ("TRUER17 sem_{0}=MZ bytes={1}" -f $s.Name, (Get-Item $out).Length)
}

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-semantic-c1-regen.js") "_stage-truer17"
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=c1_sem_regen"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER17 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER17 trust_root=place_stub"
Write-Host "TRUER17 code_sot=yoyo_plant_tyb_halves"
Write-Host "TRUER17 parse1_sem_slice_sot=yoyo_semantic_tyb"
Write-Host "TRUER17 parse1_sem_batch=3"
Write-Host "TRUER17 c1_sem_regen=EXPECT_IN_TEXT"
Write-Host "TRUER17 doc=AUTO-TRUER17.md"
Write-Host "TRUER17 status=GREEN"
Write-Host "TRUER17 note=parse1_sem_batch3_addv_orv_subv_imul;not_Stage17;plant_body_still_a1;not_Thompson"
exit 0
