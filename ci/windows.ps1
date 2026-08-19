# CI for Windows: build, test and enforce the plugin standard.
$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")

cargo test --workspace

# Standard: every plugin must wrap its work in with_timeout (CONTRIBUTING.md).
foreach ($f in Get-ChildItem "plugins\*\src\main.rs") {
    if (-not (Select-String -Path $f.FullName -Pattern "with_timeout" -Quiet)) {
        Write-Error "$($f.FullName) must use xfetch_plugin_api::with_timeout"
        exit 1
    }
}
Write-Host "All plugins use with_timeout."
