# Reports whether this Windows machine can build the encrypted store (PLAN-14, research.md).
# Needs: Strawberry Perl on PATH (vendored OpenSSL build), the C++ build tools, and a repository
# path short enough that target\ paths stay at 121 characters or fewer.
$ok = $true
$perl = Get-Command perl -ErrorAction SilentlyContinue
if (-not $perl -or $perl.Source -notmatch 'Strawberry') {
    Write-Output 'FAIL: Strawberry Perl is not first on PATH. The vendored OpenSSL build needs it; install Strawberry Perl and put C:\Strawberry\perl\bin on PATH.'
    $ok = $false
}
$cl = Get-Command cl.exe -ErrorAction SilentlyContinue
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not $cl -and -not (Test-Path $vswhere)) {
    Write-Output 'FAIL: the Visual Studio C++ build tools were not found. Install "Desktop development with C++".'
    $ok = $false
}
$repo = (Get-Location).Path
$longest = ($repo + '\target\debug\build').Length
if ($longest -gt 121) {
    Write-Output "FAIL: the build folder path is $longest characters; keep the repository path short so it stays at 121 or fewer."
    $ok = $false
}
if ($ok) { Write-Output 'windows-prereqs: ok' }
exit ([int](-not $ok))
