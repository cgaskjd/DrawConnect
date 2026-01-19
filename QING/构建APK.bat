@echo off
chcp 65001 >nul
echo ========================================
echo    轻画 v0.1 APK 构建工具
echo ========================================
echo.

echo [1] 构建 Debug APK (快速测试)
echo [2] 构建 Release APK (正式发布)
echo [3] 清理构建缓存
echo [4] 退出
echo.

set /p choice="请选择操作 (1-4): "

if "%choice%"=="1" goto build_debug
if "%choice%"=="2" goto build_release
if "%choice%"=="3" goto clean
if "%choice%"=="4" goto end

echo 无效的选择!
pause
goto end

:build_debug
echo.
echo 正在构建 Debug APK...
echo.
call gradlew.bat assembleDebug
if %errorlevel% equ 0 (
    echo.
    echo ========================================
    echo ✅ Debug APK 构建成功!
    echo ========================================
    echo.
    echo APK 位置: app\build\outputs\apk\debug\app-debug.apk
    echo.
    echo 正在重命名...
    if exist "app\build\outputs\apk\debug\app-debug.apk" (
        copy "app\build\outputs\apk\debug\app-debug.apk" "轻画-v0.1-debug.apk"
        echo ✅ 已复制到: 轻画-v0.1-debug.apk
    )
) else (
    echo.
    echo ❌ 构建失败! 请检查错误信息
)
echo.
pause
goto end

:build_release
echo.
echo 正在构建 Release APK...
echo ⚠️  注意: Release 版本需要签名密钥
echo.
call gradlew.bat assembleRelease
if %errorlevel% equ 0 (
    echo.
    echo ========================================
    echo ✅ Release APK 构建成功!
    echo ========================================
    echo.
    echo APK 位置: app\build\outputs\apk\release\app-release.apk
    echo.
    echo 正在重命名...
    if exist "app\build\outputs\apk\release\app-release.apk" (
        copy "app\build\outputs\apk\release\app-release.apk" "轻画-v0.1-release.apk"
        echo ✅ 已复制到: 轻画-v0.1-release.apk
    )
) else (
    echo.
    echo ❌ 构建失败! 请检查错误信息
    echo.
    echo 提示: Release 版本需要配置签名密钥
    echo 请参考 APK打包指南.md 中的签名配置说明
)
echo.
pause
goto end

:clean
echo.
echo 正在清理构建缓存...
echo.
call gradlew.bat clean
if %errorlevel% equ 0 (
    echo.
    echo ✅ 清理完成!
) else (
    echo.
    echo ❌ 清理失败!
)
echo.
pause
goto end

:end
echo.
echo 感谢使用轻画构建工具!
echo.