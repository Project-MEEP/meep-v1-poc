# Reports whether this Windows machine can build the encrypted store (PLAN-14, research.md).
# Needs: Strawberry Perl on PATH (vendored OpenSSL build), the C++ build tools, and a target
# folder path of 121 characters or fewer (research.md counts Cargo's inner folders separately).
$ok = $true

$perl = Get-Command perl -ErrorAction SilentlyContinue
if (-not $perl -or $perl.Source -notmatch 'Strawberry') {
    Write-Output 'FAIL: Strawberry Perl is not first on PATH. The vendored OpenSSL build needs it; install Strawberry Perl and put C:\Strawberry\perl\bin on PATH.'
    $ok = $false
}

# The installer being present is not enough: ask for the C++ x64 build tools component itself.
$cl = Get-Command cl.exe -ErrorAction SilentlyContinue
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
$cppTools = $null
if (Test-Path $vswhere) {
    $cppTools = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
}
if (-not $cl -and -not $cppTools) {
    Write-Output 'FAIL: the Visual Studio C++ build tools were not found. Install "Desktop development with C++".'
    $ok = $false
}

# Measure the target folder itself, not the folders Cargo adds inside it.
if ($env:CARGO_TARGET_DIR) { $target = $env:CARGO_TARGET_DIR } else { $target = Join-Path (Get-Location).Path 'target' }
$length = $target.Length
if ($length -gt 121) {
    Write-Output "FAIL: the target folder path is $length characters; keep it at 121 or fewer (use a shorter repository path or CARGO_TARGET_DIR)."
    $ok = $false
}

if ($ok) { Write-Output 'windows-prereqs: ok' }
exit ([int](-not $ok))
