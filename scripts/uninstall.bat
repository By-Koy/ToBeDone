REM --------------------------------------------------
REM This is a modified copy of install.bat, credits:
REM Created by Nadav Babad, (Github: @BlazingHotCode)
REM Modified (ruined, probably) by Koy
REM --------------------------------------------------

@ECHO OFF

REM --------------------------------------------------
REM Remove executable
REM --------------------------------------------------

DEL /Q "%PROGRAMFILES%\ToBeDone.exe"

IF ERRORLEVEL 1 (
    ECHO ERROR: Failed to Remove ToBeDone.exe from:
    ECHO   "%PROGRAMFILES%"
    ECHO.
    ECHO Check that you have permission to write to Program Files.
    EXIT /B 1
)

REM --------------------------------------------------
REM Remove application data directory
REM --------------------------------------------------


    RMDIR /S /Q "%TBD_NOTES_HOME%/TBD"

IF ERRORLEVEL 1 (
    ECHO ERROR: Failed to remove:
    ECHO   "%TBD_NOTES_HOME%/TBD"
    EXIT /B 1
)