# build and bundle the nerum binary and license files into a zip file
# usage: 
# Set-ExecutionPolicy -ExecutionPolicy Unrestricted -Scope Process (Only needed if not already set)
# .\scripts\bundle_win_x86_64.ps1

$binName = "nerum.exe"
$version = "1.2.0"
$osArch = "x86_64-pc-windows-msvc"
$distDir = ".\dist"

$zipFilename = "nerum-$version-$osArch.zip"

Write-Host "Building nerum binary for $osArch"
cargo build --release

# if distDir does not exist, create it
if (-not (Test-Path -Path $distDir -PathType Container)) {
    New-Item -Path $distDir -ItemType Directory
}

Copy-Item -Path ".\target\release\$binName" -Destination "$distDir\$binName" -Force
Copy-Item -Path ".\LICENSE" -Destination "$distDir\LICENSE" -Force

Set-Location -Path $distDir
Write-Host "Creating zip file $zipFilename"
Compress-Archive -Path "$binName", "LICENSE" -DestinationPath $zipFilename
Write-Host "Done"
