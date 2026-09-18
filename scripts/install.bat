set TBD_NOTES_HOME=%LOCALAPPDATA%

git clone https://github.com/By-Koy/ToBeDone.git
cd ToBeDone
cargo build --release

xcopy target\release\ToBeDone.exe %PROGRAMFILES%\TBD\
MD %LOCALAPPDATA%\TBD
cd ..
RMDIR ToBeDone
