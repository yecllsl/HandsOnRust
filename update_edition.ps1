#!/usr/bin/env pwsh
<#
.SYNOPSIS
递归搜索当前项目目录下所有的Cargo.toml文件，并将其中的edition = "2018"字段替换为edition = "2021"。
.DESCRIPTION
脚本能够处理不同操作系统下的路径分隔符，并确保在替换前备份原始文件。
脚本输出被修改的文件路径列表，并验证所有替换操作是否成功完成。
.PARAMETER DryRun
干跑模式，不实际修改文件。
.PARAMETER NoBackup
不创建备份文件（不推荐）。
.PARAMETER Root
指定根目录（默认为脚本所在目录）。
.EXAMPLE
./update_edition.ps1
执行替换操作。
.EXAMPLE
./update_edition.ps1 -DryRun
干跑模式，预览将要修改的文件。
#>

param(
    [switch]$DryRun,
    [switch]$NoBackup,
    [string]$Root
)

$ErrorActionPreference = "Stop"

# 获取根目录
if ($Root) {
    $rootDir = Resolve-Path $Root
} else {
    $rootDir = $PSScriptRoot
}

Write-Host "正在搜索目录: $rootDir" -ForegroundColor Cyan
if ($DryRun) {
    Write-Host "干跑模式：不会实际修改文件" -ForegroundColor Yellow
}
if ($NoBackup) {
    Write-Host "警告：禁用备份功能" -ForegroundColor Red
}

# 递归查找所有Cargo.toml文件，排除target和.git目录
$cargoFiles = Get-ChildItem -Path $rootDir -Filter "Cargo.toml" -File -Recurse | 
    Where-Object { $_.FullName -notmatch "\\target\\" -and $_.FullName -notmatch "\\.git\\" }

Write-Host "找到 $($cargoFiles.Count) 个 Cargo.toml 文件" -ForegroundColor Cyan

$modifiedFiles = @()
$failedFiles = @()

foreach ($file in $cargoFiles) {
    $relPath = $file.FullName.Substring($rootDir.Length + 1)
    Write-Host "`n处理: $relPath" -ForegroundColor White
    
    # 备份文件（除非禁用或干跑模式）
    $backupPath = $null
    if (-not $NoBackup) {
        $backupPath = $file.FullName + ".bak"
        if (-not $DryRun) {
            try {
                Copy-Item -Path $file.FullName -Destination $backupPath -Force
                Write-Host "备份: $($file.FullName) -> $backupPath" -ForegroundColor Gray
            } catch {
                Write-Host "备份文件 $($file.FullName) 失败: $_" -ForegroundColor Red
                $failedFiles += [pscustomobject]@{File=$file.FullName; Reason="备份失败"}
                continue
            }
        } else {
            Write-Host "[干跑] 备份: $($file.FullName) -> $backupPath" -ForegroundColor Gray
        }
    }
    
    # 读取内容
    try {
        $content = Get-Content -Path $file.FullName -Raw -Encoding UTF8
    } catch {
        Write-Host "读取文件 $($file.FullName) 失败: $_" -ForegroundColor Red
        $failedFiles += [pscustomobject]@{File=$file.FullName; Reason="读取失败"}
        continue
    }
    
    # 检查是否包含 edition = "2018"
    if ($content -notmatch 'edition\s*=\s*"2018"') {
        Write-Host "未找到 edition = `"2018`"，跳过" -ForegroundColor Gray
        continue
    }
    
    # 替换
    $newContent = $content -replace 'edition\s*=\s*"2018"', 'edition = "2021"'
    
    # 检查是否有变化
    if ($newContent -eq $content) {
        Write-Host "内容未改变，跳过" -ForegroundColor Gray
        continue
    }
    
    if ($DryRun) {
        Write-Host "[干跑] 将修改: $($file.FullName)" -ForegroundColor Green
        $modifiedFiles += $file.FullName
        continue
    }
    
    # 写入新内容
    try {
        Set-Content -Path $file.FullName -Value $newContent -Encoding UTF8 -NoNewline
        Write-Host "替换成功" -ForegroundColor Green
    } catch {
        Write-Host "写入文件 $($file.FullName) 失败: $_" -ForegroundColor Red
        # 恢复备份
        if ($backupPath -and (Test-Path $backupPath)) {
            Copy-Item -Path $backupPath -Destination $file.FullName -Force
            Remove-Item -Path $backupPath
        }
        $failedFiles += [pscustomobject]@{File=$file.FullName; Reason="写入失败"}
        continue
    }
    
    # 验证替换
    $verifyContent = Get-Content -Path $file.FullName -Raw -Encoding UTF8
    if ($verifyContent -match 'edition\s*=\s*"2018"') {
        Write-Host "验证失败：仍包含 edition = `"2018`"" -ForegroundColor Red
        # 恢复备份
        if ($backupPath -and (Test-Path $backupPath)) {
            Copy-Item -Path $backupPath -Destination $file.FullName -Force
            Remove-Item -Path $backupPath
        }
        $failedFiles += [pscustomobject]@{File=$file.FullName; Reason="验证失败：仍包含 edition = `"2018`""}
        continue
    }
    if ($verifyContent -notmatch 'edition\s*=\s*"2021"') {
        Write-Host "验证失败：未找到 edition = `"2021`"" -ForegroundColor Red
        # 恢复备份
        if ($backupPath -and (Test-Path $backupPath)) {
            Copy-Item -Path $backupPath -Destination $file.FullName -Force
            Remove-Item -Path $backupPath
        }
        $failedFiles += [pscustomobject]@{File=$file.FullName; Reason="验证失败：未找到 edition = `"2021`""}
        continue
    }
    
    $modifiedFiles += $file.FullName
    Write-Host "成功: $relPath" -ForegroundColor Green
}

# 输出结果
Write-Host "`n" + ("="*60) -ForegroundColor Cyan
Write-Host "替换操作完成" -ForegroundColor Cyan
Write-Host ("="*60) -ForegroundColor Cyan

if ($modifiedFiles.Count -gt 0) {
    Write-Host "`n成功修改的文件 ($($modifiedFiles.Count) 个):" -ForegroundColor Green
    foreach ($file in $modifiedFiles) {
        $rel = $file.Substring($rootDir.Length + 1)
        Write-Host "  - $rel" -ForegroundColor Gray
    }
} else {
    Write-Host "`n没有文件被修改。" -ForegroundColor Yellow
}

if ($failedFiles.Count -gt 0) {
    Write-Host "`n处理失败的文件 ($($failedFiles.Count) 个):" -ForegroundColor Red
    foreach ($fail in $failedFiles) {
        $rel = $fail.File.Substring($rootDir.Length + 1)
        Write-Host "  - $rel : $($fail.Reason)" -ForegroundColor Gray
    }
}

# 最终验证所有修改的文件（仅当不是干跑模式且没有失败时）
if (-not $DryRun -and $modifiedFiles.Count -gt 0) {
    Write-Host "`n最终验证..." -ForegroundColor Cyan
    $allVerified = $true
    foreach ($file in $modifiedFiles) {
        $content = Get-Content -Path $file -Raw -Encoding UTF8
        if ($content -match 'edition\s*=\s*"2018"') {
            Write-Host "  ❌ $($file.Substring($rootDir.Length + 1)): 仍包含 edition = `"2018`"" -ForegroundColor Red
            $allVerified = $false
        } elseif ($content -notmatch 'edition\s*=\s*"2021"') {
            Write-Host "  ❌ $($file.Substring($rootDir.Length + 1)): 未找到 edition = `"2021`"" -ForegroundColor Red
            $allVerified = $false
        } else {
            Write-Host "  ✓ $($file.Substring($rootDir.Length + 1)): 验证通过" -ForegroundColor Green
        }
    }
    
    if ($allVerified -and $failedFiles.Count -eq 0) {
        Write-Host "`n✅ 所有替换操作均成功完成！" -ForegroundColor Green
        exit 0
    } else {
        Write-Host "`n⚠️  部分操作未完成或验证失败。" -ForegroundColor Yellow
        exit 1
    }
} else {
    if ($DryRun) {
        Write-Host "`n干跑模式完成。请检查以上输出，确认无误后运行不带 -DryRun 的参数进行实际修改。" -ForegroundColor Cyan
    }
    exit ($failedFiles.Count -eq 0 ? 0 : 1)
}