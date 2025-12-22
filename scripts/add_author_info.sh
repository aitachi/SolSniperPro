#!/bin/bash

###############################################################################
# 批量为文档添加作者信息
# Author: Aitachi
# Email: 44158892@qq.com
# Wechat: 18116011230
###############################################################################

AUTHOR_INFO="---

**Author**: Aitachi
**Email**: 44158892@qq.com
**Wechat**: 18116011230

---"

PROJECT_ROOT="C:/Users/ASUS/Desktop/B-partjob/solsinapor/SolSniperPro-main"

# 需要处理的文档列表
DOCS=(
    "docs/01_SYSTEM_ARCHITECTURE.md"
    "docs/02_STRATEGY_GUIDE.md"
    "docs/03_API_REFERENCE.md"
    "docs/04_DEPLOYMENT_GUIDE.md"
    "docs/05_FRONTEND_ARCHITECTURE.md"
    "docs/06_FRONTEND_IMPLEMENTATION_SUMMARY.md"
    "frontend/README.md"
    "QUICK_START.md"
    "快速启动指南.md"
    "使用手册.md"
)

cd "$PROJECT_ROOT"

for doc in "${DOCS[@]}"; do
    if [ -f "$doc" ]; then
        echo "Processing: $doc"

        # 检查是否已有作者信息
        if grep -q "Author: Aitachi" "$doc"; then
            echo "  ✓ Already has author info, skipping..."
        else
            # 在文件开头添加作者信息
            echo "$AUTHOR_INFO" | cat - "$doc" > temp && mv temp "$doc"
            echo "  ✓ Added author info"
        fi

        # 检查是否有Claude字样
        if grep -qi "claude" "$doc"; then
            echo "  ⚠ Warning: Found 'Claude' in file!"
            # 替换Claude相关字样
            sed -i 's/Claude Code/SolSniper Pro/g' "$doc"
            sed -i 's/Claude Sonnet/AI Assistant/g' "$doc"
            sed -i 's/Claude/AI/g' "$doc"
            echo "  ✓ Removed Claude references"
        fi
    else
        echo "  ✗ File not found: $doc"
    fi
done

echo ""
echo "✨ Done!"
