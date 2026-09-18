SET TBD_NOTES_HOME=%LOCALAPPDATA%

git clone https://github.com/By-Koy/ToBeDone.git
CD ToBeDone
cargo build --release && XCOPY target\release\ToBeDone.exe %PROGRAMFILES%\TBD\ && MD %LOCALAPPDATA%\TBD

CD ..
RMDIR ToBeDone
