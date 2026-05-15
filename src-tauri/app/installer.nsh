!macro NSIS_HOOK_POSTINSTALL
  EnVar::AddValue "PATH" "$INSTDIR"
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  EnVar::DeleteValue "PATH" "$INSTDIR"
!macroend
