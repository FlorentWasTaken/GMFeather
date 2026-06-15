!macro NSIS_HOOK_POSTINSTALL
  CopyFiles "$INSTDIR\_up_\target\release\gmfeather.exe" "$INSTDIR"
  nsExec::Exec `powershell -NoProfile -ExecutionPolicy Bypass -Command "$$p = [Environment]::GetEnvironmentVariable('PATH', 'User'); $$d = '$INSTDIR'; $$paths = $$p -split ';' | Where-Object { $$_ -ne '' }; if ($$paths -notcontains $$d) { $$paths += $$d; [Environment]::SetEnvironmentVariable('PATH', ($$paths -join ';'), 'User') }"`
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  Delete "$INSTDIR\gmfeather.exe"
  nsExec::Exec `powershell -NoProfile -ExecutionPolicy Bypass -Command "$$p = [Environment]::GetEnvironmentVariable('PATH', 'User'); $$d = '$INSTDIR'; $$paths = $$p -split ';' | Where-Object { $$_ -ne '' -and $$_ -ne $$d }; [Environment]::SetEnvironmentVariable('PATH', ($$paths -join ';'), 'User') "`
!macroend
