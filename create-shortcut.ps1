$sh = New-Object -COM WScript.Shell
$sc = $sh.CreateShortcut([Environment]::GetFolderPath('Desktop') + '\Apex Setup.lnk')
$sc.TargetPath = 'C:\Users\pista\Documents\FocusApp\src-tauri\target\release\bundle\nsis\Apex_1.0.0_x64-setup.exe'
$sc.Save()
Write-Host "Zastupce vytvoren na Plose!"
