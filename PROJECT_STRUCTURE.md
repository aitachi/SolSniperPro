# SolSniper Pro - Project Structure

**Author**: Aitachi  
**Email**: 44158892@qq.com  
**Last Updated**: 2025-11-10

---

## Directory Structure

```
SolSniperPro/
├── README.md                    # English documentation
├── README_CN.md                 # Chinese documentation
├── Cargo.toml                   # Workspace configuration
├── Cargo.lock                   # Dependency lock file
├── config.toml                  # Runtime configuration
├── .env.example                 # Environment variables template
├── product.md                   # Product requirements
│
├── crates/                      # Source code (9 crates)
│   ├── core/                    # Core types & config
│   ├── ml-model/                # ML risk prediction
│   ├── smart-money-tracker/     # Smart wallet following
│   ├── behavior-pattern/        # Scam pattern detection
│   ├── strategy-engine/         # 6 basic strategies
│   ├── advanced-strategies/     # 6 MEV strategies
│   ├── trading-engine/          # Transaction execution
│   ├── risk-analyzer/           # Contract analysis
│   └── data-collector/          # Data pipeline
│
├── docs/                        # Documentation center
│   ├── reports/                 # All reports
│   │   ├── STRATEGIES_REPORT.md
│   │   ├── SECURITY_AUDIT.md
│   │   ├── COMPREHENSIVE_TEST_REPORT.md
│   │   ├── TEST_REPORT.md
│   │   ├── TEST_SUMMARY.md
│   │   ├── COVERAGE_REPORT.md
│   │   └── TEST_DELIVERY.md
│   │
│   ├── guides/                  # User guides
│   │   ├── FEATURES.md
│   │   ├── ARCHITECTURE.md
│   │   ├── IMPLEMENTATION_SUMMARY.md
│   │   ├── QUICKSTART.md
│   │   ├── ADVANCED_STRATEGIES.md
│   │   └── SNIPING_STRATEGIES.md
│   │
│   └── tests/                   # Test documentation
│       ├── TESTING.md
│       └── test_results.json
│
├── scripts/                     # Build & test scripts
│   ├── run_tests.sh
│   ├── build.sh
│   └── build.bat
│
└── data/                        # Data files
    └── DATA_README.md
```

---

## Quick Links

### 📖 Getting Started
- [README (English)](README.md)
- [README (中文)](README_CN.md)
- [Quick Start Guide](docs/guides/QUICKSTART.md)

### 📊 Reports
- [Strategies Report](docs/reports/STRATEGIES_REPORT.md) - 12 sniping strategies analysis
- [Security Audit](docs/reports/SECURITY_AUDIT.md) - Full security assessment
- [Test Report](docs/reports/COMPREHENSIVE_TEST_REPORT.md) - Test results

### 🔧 Guides
- [Features Guide](docs/guides/FEATURES.md) - Feature descriptions
- [Architecture](docs/guides/ARCHITECTURE.md) - System design
- [Advanced Strategies](docs/guides/ADVANCED_STRATEGIES.md) - MEV strategies

### 🧪 Testing
- [Testing Guide](docs/tests/TESTING.md)
- [Test Results](docs/tests/test_results.json)

---

## File Count

```
Source Code:    41 Rust files (12,847 lines)
Documentation:  21 files
Tests:          45 tests (100% passed)
Scripts:        3 files
```

---

**GitHub**: https://github.com/aitachi/SolSniperPro  
**Author**: Aitachi (44158892@qq.com)
