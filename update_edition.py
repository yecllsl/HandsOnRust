#!/usr/bin/env python3
"""
递归搜索当前项目目录下所有的Cargo.toml文件，并将其中的edition = "2018"字段替换为edition = "2021"。
脚本能够处理不同操作系统下的路径分隔符，并确保在替换前备份原始文件。
脚本输出被修改的文件路径列表，并验证所有替换操作是否成功完成。
"""

import os
import re
import shutil
import sys
import argparse
from pathlib import Path

def find_cargo_toml_files(root_dir):
    """递归查找所有Cargo.toml文件"""
    cargo_files = []
    for dirpath, _, filenames in os.walk(root_dir):
        # 跳过 target 和 .git 目录
        if "target" in dirpath or ".git" in dirpath:
            continue
        for filename in filenames:
            if filename == "Cargo.toml":
                full_path = os.path.join(dirpath, filename)
                cargo_files.append(full_path)
    return cargo_files

def backup_file(file_path, dry_run=False):
    """创建备份文件，添加.bak后缀"""
    if dry_run:
        print(f"[干跑] 备份: {file_path} -> {file_path}.bak")
        return file_path + ".bak"
    
    backup_path = file_path + ".bak"
    try:
        shutil.copy2(file_path, backup_path)
        print(f"备份: {file_path} -> {backup_path}")
        return backup_path
    except Exception as e:
        print(f"备份文件 {file_path} 失败: {e}")
        return None

def replace_edition(file_path, dry_run=False):
    """替换文件中的edition = "2021"为edition = "2024" """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # 使用正则表达式匹配 edition = "2021"（允许空格变化）
        pattern = r'edition\s*=\s*"2021"'
        replacement = 'edition = "2024"'
        
        # 检查是否匹配
        if not re.search(pattern, content):
            return False, "未找到 edition = \"2021\""
        
        new_content = re.sub(pattern, replacement, content)
        
        # 如果没有变化，则返回
        if new_content == content:
            return False, "内容未改变"
        
        if dry_run:
            print(f"[干跑] 将修改: {file_path}")
            return True, "干跑模式，未实际修改"
        
        # 写入新内容
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        
        return True, "替换成功"
    except Exception as e:
        return False, f"替换过程中出错: {e}"

def verify_replacement(file_path):
    """验证文件中是否已无edition = "2021"，且包含edition = "2024" """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # 检查是否还有edition = "2021"
        if re.search(r'edition\s*=\s*"2021"', content):
            return False, "仍包含 edition = \"2021\""
        
        # 检查是否包含edition = "2024"
        if not re.search(r'edition\s*=\s*"2024"', content):
            return False, "未找到 edition = \"2024\""
        
        return True, "验证通过"
    except Exception as e:
        return False, f"验证过程中出错: {e}"

def main():
    parser = argparse.ArgumentParser(description='递归替换Cargo.toml中的Rust edition字段')
    parser.add_argument('--dry-run', action='store_true', help='干跑模式，不实际修改文件')
    parser.add_argument('--no-backup', action='store_true', help='不创建备份文件（不推荐）')
    parser.add_argument('--root', type=str, default=None, help='指定根目录（默认为脚本所在目录）')
    
    args = parser.parse_args()
    
    # 获取根目录
    if args.root:
        root_dir = os.path.abspath(args.root)
    else:
        script_dir = os.path.dirname(os.path.abspath(__file__))
        root_dir = script_dir
    
    print(f"正在搜索目录: {root_dir}")
    if args.dry_run:
        print("干跑模式：不会实际修改文件")
    if args.no_backup:
        print("警告：禁用备份功能")
    
    cargo_files = find_cargo_toml_files(root_dir)
    print(f"找到 {len(cargo_files)} 个 Cargo.toml 文件")
    
    modified_files = []
    failed_files = []
    
    for file_path in cargo_files:
        rel_path = os.path.relpath(file_path, root_dir)
        print(f"\n处理: {rel_path}")
        
        # 备份文件（除非禁用或干跑模式）
        backup_result = None
        if not args.no_backup:
            backup_result = backup_file(file_path, args.dry_run)
            if backup_result is None and not args.dry_run:
                failed_files.append((file_path, "备份失败"))
                continue
        
        # 替换内容
        success, message = replace_edition(file_path, args.dry_run)
        if not success:
            print(f"替换失败: {message}")
            # 如果已创建备份且不是干跑模式，则恢复
            if backup_result and not args.dry_run:
                shutil.copy2(backup_result, file_path)
                os.remove(backup_result)
            failed_files.append((file_path, message))
            continue
        
        # 如果不是干跑模式，验证替换
        if not args.dry_run:
            verify_success, verify_message = verify_replacement(file_path)
            if not verify_success:
                print(f"验证失败: {verify_message}")
                # 恢复备份
                if backup_result:
                    shutil.copy2(backup_result, file_path)
                    os.remove(backup_result)
                failed_files.append((file_path, f"验证失败: {verify_message}"))
                continue
        
        modified_files.append(file_path)
        print(f"成功: {rel_path}")
    
    # 输出结果
    print("\n" + "="*60)
    print("替换操作完成")
    print("="*60)
    
    if modified_files:
        print(f"\n成功修改的文件 ({len(modified_files)} 个):")
        for file_path in modified_files:
            rel_path = os.path.relpath(file_path, root_dir)
            print(f"  - {rel_path}")
    else:
        print("\n没有文件被修改。")
    
    if failed_files:
        print(f"\n处理失败的文件 ({len(failed_files)} 个):")
        for file_path, reason in failed_files:
            rel_path = os.path.relpath(file_path, root_dir)
            print(f"  - {rel_path}: {reason}")
    
    # 最终验证所有修改的文件（仅当不是干跑模式且没有失败时）
    if not args.dry_run and modified_files:
        print("\n最终验证...")
        all_verified = True
        for file_path in modified_files:
            verify_success, verify_message = verify_replacement(file_path)
            if not verify_success:
                print(f"  ❌ {os.path.relpath(file_path, root_dir)}: {verify_message}")
                all_verified = False
            else:
                print(f"  ✓ {os.path.relpath(file_path, root_dir)}: 验证通过")
        
        if all_verified and not failed_files:
            print("\n✅ 所有替换操作均成功完成！")
            return 0
        else:
            print("\n⚠️  部分操作未完成或验证失败。")
            return 1
    else:
        if args.dry_run:
            print("\n干跑模式完成。请检查以上输出，确认无误后运行不带 --dry-run 的命令进行实际修改。")
        return 0 if not failed_files else 1

if __name__ == "__main__":
    sys.exit(main())