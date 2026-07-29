Write-Host "🚀 Installing Flux for Windows..." -ForegroundColor Cyan

$OS = "windows"
$ARCH = "x64" # Assuming x64 for Windows
$VERSION = "latest"
$URL = "https://github.com/NotZenith/flux/releases/download/$VERSION/flux-$OS-$ARCH.exe"

$InstallDir = "$env:USERPROFILE\.flux\bin"
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

$Dest = "$InstallDir\flux.exe"

Write-Host "Downloading Flux from $URL..."
Invoke-WebRequest -Uri $URL -OutFile $Dest

# Add to PATH if not present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-Host "✅ Flux added to PATH. Please restart your terminal." -ForegroundColor Green
}

Write-Host "✅ Flux successfully installed! Run 'flux --help' to get started." -ForegroundColor Green
