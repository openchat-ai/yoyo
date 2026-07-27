#!/usr/bin/env pwsh
# PROMPT-v3.md comprehensive slim-down script
# Reads file as UTF-8, applies replacements by content search, writes back.

$ErrorActionPreference = 'Stop'
$path = 'f:\yoyo\PROMPT-v3.md'
$bytes = [System.IO.File]::ReadAllBytes($path)
$utf8 = New-Object System.Text.UTF8Encoding($false)
$text = $utf8.GetString($bytes)
$lines = $text -split "`r?`n"
Write-Host ('Start: {0} lines' -f $lines.Length)

function Read-Template {
  param([string]$TplPath)
  return [System.IO.File]::ReadAllText($TplPath, $utf8).TrimEnd("`r","`n")
}

function Find-Line {
  param([string[]]$Lines, [string]$Pattern, [int]$FromIdx = 0)
  for ($i = $FromIdx; $i -lt $Lines.Length; $i++) {
    if ($Lines[$i] -like $Pattern) { return $i }
  }
  return -1
}

function Replace-Line-Range {
  param([string[]]$Lines, [int]$StartIdx, [int]$EndIdx, [string]$Replacement)
  $before = $Lines[0..($StartIdx-1)]
  $after = $Lines[($EndIdx+1)..($Lines.Length-1)]
  return @($before) + @($Replacement) + @($after)
}

# ==================== EDIT 1: Header v3.3.8 → v3.3.9 ====================
$h = Find-Line -Lines $lines -Pattern '# YOYO: Engineering Specification (v*'
$hEnd = Find-Line -Lines $lines -Pattern 'Markers **NORMATIVE** / **NON-NORMATIVE** / **ROADMAP** follow Part N*' -FromIdx $h
$lines = Replace-Line-Range -Lines $lines -StartIdx $h -EndIdx $hEnd -Replacement (Read-Template 'f:\yoyo\tpl-header.txt')
Write-Host ('After header: {0} lines' -f $lines.Length)

# ==================== EDIT 2: Compact "How to Read" section ====================
$hr = Find-Line -Lines $lines -Pattern '## How to Read This Document*'
$hrEnd = Find-Line -Lines $lines -Pattern '## Master Table of Contents*' -FromIdx $hr
$hrEnd = $hrEnd - 2  # the --- before Master TOC
$lines = Replace-Line-Range -Lines $lines -StartIdx $hr -EndIdx $hrEnd -Replacement (Read-Template 'f:\yoyo\tpl-hr.txt')
Write-Host ('After How-to-Read: {0} lines' -f $lines.Length)

# ==================== EDIT 3: Compact Part 1 4-Project Architecture ====================
$p1 = Find-Line -Lines $lines -Pattern '## Part 1: 4-Project Architecture*'
$p1End = Find-Line -Lines $lines -Pattern '## Part 2: Context and Goals*' -FromIdx $p1
$p1End = $p1End - 3  # the --- before Part 2
$lines = Replace-Line-Range -Lines $lines -StartIdx $p1 -EndIdx $p1End -Replacement (Read-Template 'f:\yoyo\tpl-part1.txt')
Write-Host ('After Part 1: {0} lines' -f $lines.Length)

# ==================== EDIT 4: Compact Part 2 ====================
$p2 = Find-Line -Lines $lines -Pattern '## Part 2: Context and Goals*'
$p2End = Find-Line -Lines $lines -Pattern '## Part 3: Thompson Honesty*' -FromIdx $p2
$p2End = $p2End - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $p2 -EndIdx $p2End -Replacement (Read-Template 'f:\yoyo\tpl-part2.txt')
Write-Host ('After Part 2: {0} lines' -f $lines.Length)

# ==================== EDIT 5: Delete Part 4.6 Trit (NON-NORMATIVE CONVENTION) ====================
# Find start of 4.6 and end before Part G
$t46 = Find-Line -Lines $lines -Pattern '### 4.6 Ternary Data Model*'
if ($t46 -lt 0) {
  Write-Host 'Part 4.6 not found (may already be removed)'
} else {
  $t46End = Find-Line -Lines $lines -Pattern '## Part G: Formal `.ty` Grammar*' -FromIdx $t46
  $t46End = $t46End - 3  # the --- before Part G
  $lines = Replace-Line-Range -Lines $lines -StartIdx $t46 -EndIdx $t46End -Replacement (Read-Template 'f:\yoyo\tpl-part46.txt')
  Write-Host ('After Part 4.6 deletion: {0} lines' -f $lines.Length)
}

# ==================== EDIT 6: Compact Part 5 historical sections (5.3-5.8) ====================
# Keep Part 5 intro and 5.1-5.2; replace 5.3-5.8 with aux pointer
$p5 = Find-Line -Lines $lines -Pattern '### 5.3 Pre-Phase-2 State*'
if ($p5 -ge 0) {
  $p5End = Find-Line -Lines $lines -Pattern '## Part 5B: Cold-Start*' -FromIdx $p5
  $p5End = $p5End - 3
  $lines = Replace-Line-Range -Lines $lines -StartIdx $p5 -EndIdx $p5End -Replacement (Read-Template 'f:\yoyo\tpl-part5-hist.txt')
  Write-Host ('After Part 5 history: {0} lines' -f $lines.Length)
}

# ==================== EDIT 7: Compact Part 6 sub-sections (6.4.1, 6.5.1, 6.6, 6.7) ====================
$p6 = Find-Line -Lines $lines -Pattern '### 6.4.1 DDC vs Reproducible Builds*'
if ($p6 -ge 0) {
  $p6End = Find-Line -Lines $lines -Pattern '### 6.5 What 3-Chain DDC Does NOT Catch*' -FromIdx $p6
  $p6End = $p6End - 3
  $lines = Replace-Line-Range -Lines $lines -StartIdx $p6 -EndIdx $p6End -Replacement ''
  Write-Host ('After 6.4.1: {0} lines' -f $lines.Length)
}

# 6.6 Trust Root → compact
$p66 = Find-Line -Lines $lines -Pattern '### 6.6 Trust Root*'
if ($p66 -ge 0) {
  $p66End = Find-Line -Lines $lines -Pattern '### 6.7 Chain-of-Compilation Logs*' -FromIdx $p66
  $p66End = $p66End - 3
  $lines = Replace-Line-Range -Lines $lines -StartIdx $p66 -EndIdx $p66End -Replacement (Read-Template 'f:\yoyo\tpl-part66.txt')
  Write-Host ('After 6.6: {0} lines' -f $lines.Length)
}

# 6.7 Chain-of-Compilation Logs → compact
$p67 = Find-Line -Lines $lines -Pattern '### 6.7 Chain-of-Compilation Logs*'
if ($p67 -ge 0) {
  $p67End = Find-Line -Lines $lines -Pattern '### 6.9 Normative Compare Algorithm*' -FromIdx $p67
  $p67End = $p67End - 3
  $lines = Replace-Line-Range -Lines $lines -StartIdx $p67 -EndIdx $p67End -Replacement (Read-Template 'f:\yoyo\tpl-part67.txt')
  Write-Host ('After 6.7: {0} lines' -f $lines.Length)
}

# ==================== EDIT 8: Compact Part 7 (Platform Abstraction) ====================
# Replace from 7.1 Problem through 7.7 Bare-Metal with compact pointer + aux reference
$p7 = Find-Line -Lines $lines -Pattern '### 7.1 Problem*'
if ($p7 -ge 0) {
  $p7End = Find-Line -Lines $lines -Pattern '### 7.8 Why Split at Syscalls*' -FromIdx $p7
  $p7End = $p7End - 3
  # We keep 7.1-7.5 intro compact; remove 7.6 libyoyo API (huge table) and 7.7 Bare-Metal (huge x64 code)
  $lines = Replace-Line-Range -Lines $lines -StartIdx $p7 -EndIdx $p7End -Replacement (Read-Template 'f:\yoyo\tpl-part7.txt')
  Write-Host ('After Part 7: {0} lines' -f $lines.Length)
}

# ==================== EDIT 9: Compact Part 8 ====================
$p8 = Find-Line -Lines $lines -Pattern '## Part 8: Variable / Name Layer*'
$p8End = Find-Line -Lines $lines -Pattern '## Part 9: Safety Architecture*' -FromIdx $p8
$p8End = $p8End - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $p8 -EndIdx $p8End -Replacement (Read-Template 'f:\yoyo\tpl-part8.txt')
Write-Host ('After Part 8: {0} lines' -f $lines.Length)

# ==================== EDIT 10: Compact Part 9 Safety ====================
# Keep core (9.1-9.4); replace 9.5-9.6 with compact references
$p95 = Find-Line -Lines $lines -Pattern '### 9.5 How the 13 Decisions Layer*'
if ($p95 -ge 0) {
  $p95End = Find-Line -Lines $lines -Pattern '## Part E: Morphological Adaptation*' -FromIdx $p95
  $p95End = $p95End - 3
  $lines = Replace-Line-Range -Lines $lines -StartIdx $p95 -EndIdx $p95End -Replacement (Read-Template 'f:\yoyo\tpl-part95.txt')
  Write-Host ('After Part 9.5-9.6: {0} lines' -f $lines.Length)
}

# ==================== EDIT 11: Compact Part E sub-sections ====================
# Move E.11-E.18 detailed text to compact pointers; keep E.0-E.10, E.19
$pe11 = Find-Line -Lines $lines -Pattern '### E.11 Language / ISA Surface for Evolution*'
if ($pe11 -ge 0) {
  $pe11End = Find-Line -Lines $lines -Pattern '### E.19 Energy↔Performance Posture Continuum*' -FromIdx $pe11
  $pe11End = $pe11End - 3
  $lines = Replace-Line-Range -Lines $lines -StartIdx $pe11 -EndIdx $pe11End -Replacement (Read-Template 'f:\yoyo\tpl-partE11-18.txt')
  Write-Host ('After E.11-18: {0} lines' -f $lines.Length)
}

# ==================== EDIT 12: Compact Part 10 (Phase Plan) ====================
# Move 4a (tyo format spec), Phase 1b, Phase 5 detail to aux; keep phase gates
$p10 = Find-Line -Lines $lines -Pattern '### 10.1 Phase 0: Foundation*'
$p10End = Find-Line -Lines $lines -Pattern '## Part 11: Cross-Project Comparison*' -FromIdx $p10
$p10End = $p10End - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $p10 -EndIdx $p10End -Replacement (Read-Template 'f:\yoyo\tpl-part10.txt')
Write-Host ('After Part 10: {0} lines' -f $lines.Length)

# ==================== EDIT 13: Delete Part 11 (Cross-Project Comparison) → aux ====================
$p11 = Find-Line -Lines $lines -Pattern '## Part 11: Cross-Project Comparison*'
$p11End = Find-Line -Lines $lines -Pattern '## Part 12: SIMD Extensions*' -FromIdx $p11
$p11End = $p11End - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $p11 -EndIdx $p11End -Replacement (Read-Template 'f:\yoyo\tpl-part11.txt')
Write-Host ('After Part 11: {0} lines' -f $lines.Length)

# ==================== EDIT 14: Delete Part 12 SIMD → aux ====================
$p12 = Find-Line -Lines $lines -Pattern '## Part 12: SIMD Extensions*'
$p12End = Find-Line -Lines $lines -Pattern '## Part 13: Decision History*' -FromIdx $p12
$p12End = $p12End - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $p12 -EndIdx $p12End -Replacement (Read-Template 'f:\yoyo\tpl-part12.txt')
Write-Host ('After Part 12: {0} lines' -f $lines.Length)

# ==================== EDIT 15: Compact Part 13 ====================
# Keep 13.1 (decisions list), 13.5 Anti-Patterns; condense 13.2-13.4, 13.6
$p13 = Find-Line -Lines $lines -Pattern '### 13.2 The 4 User Patterns*'
if ($p13 -ge 0) {
  $p13End = Find-Line -Lines $lines -Pattern '## Part 14: Maintainer Role*' -FromIdx $p13
  $p13End = $p13End - 3
  $lines = Replace-Line-Range -Lines $lines -StartIdx $p13 -EndIdx $p13End -Replacement (Read-Template 'f:\yoyo\tpl-part13.txt')
  Write-Host ('After Part 13: {0} lines' -f $lines.Length)
}

# ==================== EDIT 16: Delete Part 14 (Maintainer) → aux ====================
$p14 = Find-Line -Lines $lines -Pattern '## Part 14: Maintainer Role*'
$p14End = Find-Line -Lines $lines -Pattern '## Part 15: Demos & Use Cases*' -FromIdx $p14
$p14End = $p14End - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $p14 -EndIdx $p14End -Replacement (Read-Template 'f:\yoyo\tpl-part14.txt')
Write-Host ('After Part 14: {0} lines' -f $lines.Length)

# ==================== EDIT 17: Delete Part 15 (Demos) → aux ====================
$p15 = Find-Line -Lines $lines -Pattern '## Part 15: Demos & Use Cases*'
$p15End = Find-Line -Lines $lines -Pattern '## Part 16: Master Roadmap*' -FromIdx $p15
$p15End = $p15End - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $p15 -EndIdx $p15End -Replacement (Read-Template 'f:\yoyo\tpl-part15.txt')
Write-Host ('After Part 15: {0} lines' -f $lines.Length)

# ==================== EDIT 18: Delete Part 16 (Roadmap) → aux ====================
$p16 = Find-Line -Lines $lines -Pattern '## Part 16: Master Roadmap*'
$p16End = Find-Line -Lines $lines -Pattern '## Appendix A: libyoyo API*' -FromIdx $p16
$p16End = $p16End - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $p16 -EndIdx $p16End -Replacement (Read-Template 'f:\yoyo\tpl-part16.txt')
Write-Host ('After Part 16: {0} lines' -f $lines.Length)

# ==================== EDIT 19: Appendix B (yoyo-asm) → aux ====================
$paB = Find-Line -Lines $lines -Pattern '## Appendix B: yoyo-asm Third Implementation*'
$paBEnd = Find-Line -Lines $lines -Pattern '## Appendix C: Cross-Platform Story*' -FromIdx $paB
$paBEnd = $paBEnd - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $paB -EndIdx $paBEnd -Replacement (Read-Template 'f:\yoyo\tpl-appB.txt')
Write-Host ('After Appendix B: {0} lines' -f $lines.Length)

# ==================== EDIT 20: Appendix C (Cross-Platform) → aux ====================
$paC = Find-Line -Lines $lines -Pattern '## Appendix C: Cross-Platform Story*'
$paCEnd = Find-Line -Lines $lines -Pattern '## Appendix D: Anti-Patterns Catalog*' -FromIdx $paC
$paCEnd = $paCEnd - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $paC -EndIdx $paCEnd -Replacement (Read-Template 'f:\yoyo\tpl-appC.txt')
Write-Host ('After Appendix C: {0} lines' -f $lines.Length)

# ==================== EDIT 21: Appendix E (Build) → aux ====================
$paE = Find-Line -Lines $lines -Pattern '## Appendix E: Build & Test*'
$paEEnd = Find-Line -Lines $lines -Pattern '## Appendix F: Conformance Suite*' -FromIdx $paE
$paEEnd = $paEEnd - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $paE -EndIdx $paEEnd -Replacement (Read-Template 'f:\yoyo\tpl-appE.txt')
Write-Host ('After Appendix E: {0} lines' -f $lines.Length)

# ==================== EDIT 22: Appendix T (Thompson) → aux ====================
$paT = Find-Line -Lines $lines -Pattern '## Appendix T: Thompson 1984 Background*'
$paTEnd = Find-Line -Lines $lines -Pattern '## Appendix Bib: FACT Bibliography*' -FromIdx $paT
$paTEnd = $paTEnd - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $paT -EndIdx $paTEnd -Replacement (Read-Template 'f:\yoyo\tpl-appT.txt')
Write-Host ('After Appendix T: {0} lines' -f $lines.Length)

# ==================== EDIT 23: Compact Appendix A, D ====================
$paA = Find-Line -Lines $lines -Pattern '## Appendix A: libyoyo API*'
$paAEnd = Find-Line -Lines $lines -Pattern '## Appendix B: yoyo-asm*' -FromIdx $paA
$paAEnd = $paAEnd - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $paA -EndIdx $paAEnd -Replacement (Read-Template 'f:\yoyo\tpl-appA.txt')
Write-Host ('After Appendix A: {0} lines' -f $lines.Length)

$paD = Find-Line -Lines $lines -Pattern '## Appendix D: Anti-Patterns Catalog*'
$paDEnd = Find-Line -Lines $lines -Pattern '## Appendix E: Build & Test*' -FromIdx $paD
$paDEnd = $paDEnd - 3
$lines = Replace-Line-Range -Lines $lines -StartIdx $paD -EndIdx $paDEnd -Replacement (Read-Template 'f:\yoyo\tpl-appD.txt')
Write-Host ('After Appendix D: {0} lines' -f $lines.Length)

# Save
$joined = $lines -join "`n"
[System.IO.File]::WriteAllText($path, $joined, $utf8)
Write-Host 'Saved.'