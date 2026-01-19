# Generate placeholder icons for DrawConnect
# This creates simple colored square icons

Add-Type -AssemblyName System.Drawing

$iconDir = Split-Path -Parent $MyInvocation.MyCommand.Path

function Create-Icon {
    param (
        [int]$Size,
        [string]$OutputPath
    )

    $bitmap = New-Object System.Drawing.Bitmap($Size, $Size)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)

    # Fill with gradient-like color (blue to purple)
    $brush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 74, 144, 217))
    $graphics.FillRectangle($brush, 0, 0, $Size, $Size)

    # Add a "D" letter
    $font = New-Object System.Drawing.Font("Arial", [int]($Size * 0.5), [System.Drawing.FontStyle]::Bold)
    $textBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
    $stringFormat = New-Object System.Drawing.StringFormat
    $stringFormat.Alignment = [System.Drawing.StringAlignment]::Center
    $stringFormat.LineAlignment = [System.Drawing.StringAlignment]::Center
    $rect = New-Object System.Drawing.RectangleF(0, 0, $Size, $Size)
    $graphics.DrawString("D", $font, $textBrush, $rect, $stringFormat)

    $graphics.Dispose()
    $bitmap.Save($OutputPath, [System.Drawing.Imaging.ImageFormat]::Png)
    $bitmap.Dispose()

    Write-Host "Created: $OutputPath"
}

# Create PNG icons
Create-Icon -Size 32 -OutputPath "$iconDir\32x32.png"
Create-Icon -Size 128 -OutputPath "$iconDir\128x128.png"
Create-Icon -Size 256 -OutputPath "$iconDir\128x128@2x.png"

# Create ICO file (using 256x256 PNG as base)
$bitmap256 = New-Object System.Drawing.Bitmap(256, 256)
$graphics = [System.Drawing.Graphics]::FromImage($bitmap256)
$brush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 74, 144, 217))
$graphics.FillRectangle($brush, 0, 0, 256, 256)
$font = New-Object System.Drawing.Font("Arial", 128, [System.Drawing.FontStyle]::Bold)
$textBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
$stringFormat = New-Object System.Drawing.StringFormat
$stringFormat.Alignment = [System.Drawing.StringAlignment]::Center
$stringFormat.LineAlignment = [System.Drawing.StringAlignment]::Center
$rect = New-Object System.Drawing.RectangleF(0, 0, 256, 256)
$graphics.DrawString("D", $font, $textBrush, $rect, $stringFormat)
$graphics.Dispose()

# Save as ICO
$icon = [System.Drawing.Icon]::FromHandle($bitmap256.GetHicon())
$stream = [System.IO.File]::Create("$iconDir\icon.ico")
$icon.Save($stream)
$stream.Close()
$bitmap256.Dispose()

Write-Host "Created: $iconDir\icon.ico"

# Create ICNS placeholder (just copy PNG for now - macOS will need proper conversion)
Copy-Item "$iconDir\128x128@2x.png" "$iconDir\icon.icns" -Force
Write-Host "Created: $iconDir\icon.icns (placeholder)"

Write-Host ""
Write-Host "All icons created successfully!" -ForegroundColor Green
