@echo off
setlocal
call :find_bin
if "%1" == "-cc1" goto :L

set "_BIN_DIR=" && "%_BIN_DIR%clang140++.exe" -target aarch64-linux-android26 %*
if ERRORLEVEL 1 exit /b 1
goto :done

:L
rem target/triple already spelled out.
set "_BIN_DIR=" && "%_BIN_DIR%clang140++.exe" %*
if ERRORLEVEL 1 exit /b 1
goto :done

:find_bin
rem Accommodate a quoted arg0, e.g.: "clang"
rem https://github.com/android-ndk/ndk/issues/616
set _BIN_DIR=%~dp0
exit /b

:done
