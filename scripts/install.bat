REM --------------------------------------------------
REM Created by Nadav Babad, (Github: @BlazingHotCode)
REM Modified (ruined, probably) by Koy
REM --------------------------------------------------

@ECHO OFF
SETLOCAL

SET "TBD_NOTES_HOME=%LOCALAPPDATA%"

REM --------------------------------------------------
REM Clone repository
REM --------------------------------------------------

git clone https://github.com/By-Koy/ToBeDone.git

IF ERRORLEVEL 1 (
    ECHO ERROR: Failed to clone ToBeDone.
    GOTO :FAIL
)

REM --------------------------------------------------
REM Build
REM --------------------------------------------------

CD "ToBeDone"

cargo build --release

IF ERRORLEVEL 1 (
    ECHO ERROR: Cargo build failed.
    CD ..
    GOTO :FAIL
)

REM --------------------------------------------------
REM Copy executable
REM --------------------------------------------------

COPY /Y "target\release\ToBeDone.exe" "%PROGRAMFILES%\ToBeDone.exe" >NUL

IF ERRORLEVEL 1 (
    ECHO ERROR: Failed to copy ToBeDone.exe to:
    ECHO   "%PROGRAMFILES%"
    ECHO.
    ECHO Check that you have permission to write to Program Files.
    CD ..
    GOTO :FAIL
)

REM --------------------------------------------------
REM Create application data directory
REM --------------------------------------------------


MD "%TBD_NOTES_HOME%/TBD"

IF ERRORLEVEL 1 (
    ECHO ERROR: Failed to create:
    ECHO   "%TBD_NOTES_HOME%/TBD"
    CD ..
    GOTO :FAIL
)


REM --------------------------------------------------
REM Cleanup
REM --------------------------------------------------

CD ..
RMDIR /S /Q "ToBeDone"

EXIT /B 0

:FAIL

IF EXIST "ToBeDone" (
    cd ..
    RMDIR /S /Q "ToBeDone"
)

EXIT /B 1