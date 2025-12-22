#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
批量为文档添加作者信息并移除Claude字样

Author: Aitachi
Email: 44158892@qq.com
Wechat: 18116011230
"""

import os
import re

PROJECT_ROOT = r"C:\Users\ASUS\Desktop\B-partjob\solsinapor\SolSniperPro-main"

AUTHOR_BLOCK_EN = """---

**Author**: Aitachi
**Email**: 44158892@qq.com
**Wechat**: 18116011230

---

"""

AUTHOR_BLOCK_CN = """---

**作者**: Aitachi
**邮箱**: 44158892@qq.com
**微信**: 18116011230

---

"""

DOCS_TO_PROCESS = [
    ("docs/01_SYSTEM_ARCHITECTURE.md", "en"),
    ("docs/02_STRATEGY_GUIDE.md", "en"),
    ("docs/03_API_REFERENCE.md", "en"),
    ("docs/04_DEPLOYMENT_GUIDE.md", "en"),
    ("docs/05_FRONTEND_ARCHITECTURE.md", "en"),
    ("docs/06_FRONTEND_IMPLEMENTATION_SUMMARY.md", "en"),
    ("frontend/README.md", "en"),
    ("QUICK_START.md", "en"),
    ("快速启动指南.md", "cn"),
    ("使用手册.md", "cn"),
]

def remove_claude_references(content):
    """移除Claude相关字样"""
    replacements = [
        (r'Claude Code', 'SolSniper Pro'),
        (r'Claude Sonnet 4\.5', 'AI Assistant'),
        (r'Claude Sonnet', 'AI Assistant'),
        (r'Claude Opus', 'AI Assistant'),
        (r'Claude', 'AI'),
        (r'Anthropic', 'AI Platform'),
        (r'claude\.com/claude-code', 'solsniper.pro'),
        (r'noreply@anthropic\.com', '44158892@qq.com'),
    ]

    for pattern, replacement in replacements:
        content = re.sub(pattern, replacement, content, flags=re.IGNORECASE)

    return content

def add_author_info(file_path, lang):
    """为文档添加作者信息"""
    full_path = os.path.join(PROJECT_ROOT, file_path)

    if not os.path.exists(full_path):
        print(f"[X] File not found: {file_path}")
        return

    print(f"[*] Processing: {file_path}")

    with open(full_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # 检查是否已有作者信息
    if "Author: Aitachi" in content or "作者: Aitachi" in content:
        print(f"   [OK] Already has author info")
    else:
        # 找到第一个标题后插入作者信息
        lines = content.split('\n')
        new_lines = []
        inserted = False

        for i, line in enumerate(lines):
            new_lines.append(line)

            # 在第一个标题和分隔线之后插入
            if not inserted and line.startswith('#'):
                # 查找下一个---或空行
                j = i + 1
                while j < len(lines) and lines[j].strip() and not lines[j].startswith('---'):
                    new_lines.append(lines[j])
                    j += 1

                if j < len(lines) and lines[j].startswith('---'):
                    new_lines.append(lines[j])
                    j += 1

                # 插入作者信息
                author_block = AUTHOR_BLOCK_CN if lang == "cn" else AUTHOR_BLOCK_EN
                new_lines.append(author_block.strip())

                # 继续添加剩余行
                new_lines.extend(lines[j:])
                inserted = True
                break

        if inserted:
            content = '\n'.join(new_lines)
            print(f"   [+] Added author info")

    # 移除Claude字样
    original_content = content
    content = remove_claude_references(content)

    if original_content != content:
        print(f"   [+] Removed references")

    # 写回文件
    with open(full_path, 'w', encoding='utf-8') as f:
        f.write(content)

    print(f"   [DONE]\n")

def main():
    print("=" * 60)
    print("Process Docs - Add Author Info and Remove Claude")
    print("=" * 60)
    print()

    for doc_path, lang in DOCS_TO_PROCESS:
        add_author_info(doc_path, lang)

    print("=" * 60)
    print("[SUCCESS] All documents processed!")
    print("=" * 60)

if __name__ == "__main__":
    main()
