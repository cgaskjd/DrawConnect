# 轻画 v0.1 APK 打包指南

## 📱 应用信息

- **应用名称**: 轻画
- **版本号**: v0.1
- **包名**: com.qinghua
- **最低Android版本**: Android 8.0 (API 26)
- **目标Android版本**: Android 14 (API 34)

## 🔧 打包步骤

### 方法一: 使用 Android Studio (推荐)

#### 1. 打开项目
在 Android Studio 中打开 `DrawConnect` 项目

#### 2. 同步 Gradle
- 点击菜单: `File` → `Sync Project with Gradle Files`
- 等待同步完成

#### 3. 构建 Debug APK (快速测试)

**步骤:**
1. 点击菜单: `Build` → `Build Bundle(s) / APK(s)` → `Build APK(s)`
2. 等待构建完成
3. 点击通知中的 `locate` 链接,或在以下路径找到APK:
   ```
   DrawConnect/app/build/outputs/apk/debug/app-debug.apk
   ```

**特点:**
- ✅ 快速构建
- ✅ 适合测试
- ⚠️ 文件较大(未优化)
- ⚠️ 不能发布到应用商店

#### 4. 构建 Release APK (正式发布)

**步骤 A: 生成签名密钥(首次)**

1. 点击菜单: `Build` → `Generate Signed Bundle / APK`
2. 选择 `APK` → 点击 `Next`
3. 点击 `Create new...` 创建新密钥库
4. 填写信息:
   ```
   Key store path: 选择保存位置,如 C:\qinghua-keystore.jks
   Password: 输入密码(请记住!)
   Alias: qinghua-key
   Password: 输入密钥密码
   Validity: 25 (年)

   Certificate:
   First and Last Name: 你的名字
   Organizational Unit: 可选
   Organization: 可选
   City or Locality: 城市
   State or Province: 省份
   Country Code: CN
   ```
5. 点击 `OK` 保存

**步骤 B: 构建签名APK**

1. 点击菜单: `Build` → `Generate Signed Bundle / APK`
2. 选择 `APK` → 点击 `Next`
3. 选择刚才创建的密钥库文件
4. 输入密码
5. 选择 `release` 构建类型
6. 勾选 `V1 (Jar Signature)` 和 `V2 (Full APK Signature)`
7. 点击 `Finish`
8. 等待构建完成,APK位于:
   ```
   DrawConnect/app/build/outputs/apk/release/app-release.apk
   ```

**特点:**
- ✅ 文件较小(已优化)
- ✅ 可以发布
- ✅ 已签名
- ⚠️ 构建时间较长

### 方法二: 使用命令行

#### 1. 构建 Debug APK

```bash
cd DrawConnect
./gradlew assembleDebug
```

APK位置: `app/build/outputs/apk/debug/app-debug.apk`

#### 2. 构建 Release APK (需要先配置签名)

```bash
cd DrawConnect
./gradlew assembleRelease
```

APK位置: `app/build/outputs/apk/release/app-release.apk`

## 📦 APK 文件说明

### Debug APK
- **文件名**: `app-debug.apk`
- **大小**: 约 15-20 MB
- **用途**: 开发测试
- **签名**: 使用 debug 签名
- **可安装**: ✅ 是

### Release APK
- **文件名**: `app-release.apk` 或 `qinghua-v0.1.apk`(重命名后)
- **大小**: 约 10-15 MB
- **用途**: 正式发布
- **签名**: 使用自定义签名
- **可安装**: ✅ 是

## 📲 安装 APK

### 在 Android 设备上安装

1. **传输 APK 到手机**
   - 通过 USB 数据线复制
   - 通过微信/QQ发送
   - 通过云盘下载

2. **启用未知来源安装**
   - 打开 `设置` → `安全` → `未知来源`
   - 或在安装时允许

3. **安装应用**
   - 点击 APK 文件
   - 点击 `安装`
   - 等待安装完成

### 使用 ADB 安装

```bash
adb install app-debug.apk
# 或
adb install app-release.apk
```

## 🔍 验证 APK

### 查看 APK 信息

```bash
# 查看包名和版本
aapt dump badging app-release.apk | grep package

# 应该显示:
# package: name='com.qinghua' versionCode='1' versionName='0.1'
```

### 查看签名信息

```bash
jarsigner -verify -verbose -certs app-release.apk
```

## 📝 重命名 APK

建议将生成的 APK 重命名为更有意义的名称:

```bash
# Debug 版本
mv app-debug.apk 轻画-v0.1-debug.apk

# Release 版本
mv app-release.apk 轻画-v0.1-release.apk
```

## ⚠️ 常见问题

### Q1: 构建失败 - Gradle 同步错误

**解决方案:**
1. 检查网络连接
2. 在 Android Studio 中: `File` → `Invalidate Caches / Restart`
3. 删除 `.gradle` 文件夹后重新构建

### Q2: 安装失败 - 解析包时出现问题

**解决方案:**
1. 确保 Android 版本 ≥ 8.0
2. 重新下载 APK(可能下载不完整)
3. 清除旧版本后重新安装

### Q3: 应用闪退

**解决方案:**
1. 检查 Android 版本是否 ≥ 8.0
2. 查看 logcat 日志
3. 确保所有依赖都正确编译

### Q4: 签名密钥丢失

**重要提示:**
- ⚠️ 请务必备份签名密钥文件 (.jks)
- ⚠️ 记住密钥密码
- ⚠️ 如果丢失,将无法更新应用

## 📊 APK 大小优化建议

如果需要进一步减小 APK 大小,可以在 `app/build.gradle.kts` 中启用以下选项:

```kotlin
buildTypes {
    release {
        isMinifyEnabled = true  // 启用代码混淆
        isShrinkResources = true  // 移除未使用的资源
        proguardFiles(
            getDefaultProguardFile("proguard-android-optimize.txt"),
            "proguard-rules.pro"
        )
    }
}
```

## 🚀 发布清单

在发布前,请确认:

- [ ] 应用名称已改为"轻画"
- [ ] 版本号设置为 0.1
- [ ] 包名设置为 com.qinghua
- [ ] 已使用 Release 构建
- [ ] APK 已签名
- [ ] 在真机上测试通过
- [ ] 所有核心功能正常
- [ ] 备份了签名密钥

## 📱 测试设备要求

- **最低要求**: Android 8.0 (API 26)
- **推荐配置**:
  - Android 10+
  - 2GB+ RAM
  - 100MB+ 可用存储空间

## 📄 版本信息

- **版本**: v0.1
- **构建日期**: 2026-01-14
- **包名**: com.qinghua
- **应用名**: 轻画

---

**祝打包顺利! 🎉**