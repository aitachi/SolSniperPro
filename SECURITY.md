# 安全配置指南

## ⚠️ 生产环境部署前必读

### 1. 更改默认密钥和密码

**JWT Secret**:
```bash
# 生成新的 JWT secret (64字符)
openssl rand -base64 64

# 或使用 Node.js
node -e "console.log(require('crypto').randomBytes(64).toString('hex'))"

# 更新 .env 文件中的 JWT_SECRET
```

**数据库密码**:
```bash
# 使用强密码（至少16字符，包含大小写字母、数字、特殊字符）
DATABASE_URL=postgresql://solsniper:YOUR_STRONG_PASSWORD_HERE@localhost:5432/solsniper_db
```

### 2. 启用 HTTPS

生产环境必须使用 HTTPS：

```bash
# 使用 Let's Encrypt 获取免费 SSL 证书
sudo apt-get install certbot
sudo certbot certonly --standalone -d yourdomain.com

# 或使用 Nginx 反向代理 + SSL
```

### 3. 配置防火墙

```bash
# Ubuntu/Debian
sudo ufw enable
sudo ufw allow 22/tcp   # SSH
sudo ufw allow 80/tcp   # HTTP
sudo ufw allow 443/tcp  # HTTPS
sudo ufw deny 3000/tcp  # 不直接暴露 API 端口，使用 Nginx 代理

# 查看状态
sudo ufw status
```

### 4. 数据库安全

```sql
-- 创建受限用户
CREATE USER solsniper_app WITH PASSWORD 'STRONG_PASSWORD';

-- 只授予必要的权限
GRANT SELECT, INSERT, UPDATE ON ALL TABLES IN SCHEMA public TO solsniper_app;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO solsniper_app;

-- 禁止删除权限（根据需要调整）
REVOKE DELETE ON ALL TABLES IN SCHEMA public FROM solsniper_app;
```

### 5. 速率限制

在生产环境中添加速率限制（防止 DDoS 和暴力破解）：

**Nginx 配置示例**:
```nginx
limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s;

server {
    location /api/ {
        limit_req zone=api_limit burst=20 nodelay;
        proxy_pass http://localhost:3000;
    }
}
```

**Express 中间件** (已添加到项目):
```javascript
const rateLimit = require('express-rate-limit');

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 分钟
  max: 100 // 限制每个 IP 100 次请求
});

app.use('/api/', limiter);
```

### 6. 环境变量安全

**不要提交敏感信息到 Git**:
```bash
# 确保 .env 在 .gitignore 中
echo ".env" >> .gitignore

# 只提交示例配置
cp .env .env.example
# 编辑 .env.example，移除所有敏感信息
```

### 7. 日志和监控

```bash
# 使用 PM2 管理进程和日志
npm install -g pm2

# 启动应用
pm2 start npm --name "solsniper-api" -- start

# 查看日志
pm2 logs solsniper-api

# 设置日志轮转
pm2 install pm2-logrotate
```

### 8. 备份策略

```bash
# 自动备份 PostgreSQL
#!/bin/bash
# backup.sh
BACKUP_DIR="/var/backups/postgres"
DATE=$(date +%Y%m%d_%H%M%S)
pg_dump solsniper_db > "$BACKUP_DIR/solsniper_db_$DATE.sql"

# 保留最近7天的备份
find $BACKUP_DIR -name "*.sql" -mtime +7 -delete
```

添加到 crontab:
```bash
# 每天凌晨2点备份
0 2 * * * /path/to/backup.sh
```

### 9. 更新依赖

```bash
# 定期检查安全漏洞
npm audit
cargo audit

# 更新依赖
npm update
cargo update
```

### 10. Solana 钱包安全

```bash
# 使用硬件钱包或加密存储私钥
# 永远不要在代码中硬编码私钥
# 使用环境变量或安全的密钥管理服务

# 示例：加密钱包文件
gpg --symmetric --cipher-algo AES256 wallet.json
# 使用时解密到内存，不写入磁盘
```

---

## 安全检查清单

部署前请确认：

- [ ] 更改了默认 JWT secret
- [ ] 使用了强数据库密码
- [ ] 启用了 HTTPS
- [ ] 配置了防火墙
- [ ] 实施了速率限制
- [ ] .env 文件不在版本控制中
- [ ] 设置了日志和监控
- [ ] 配置了自动备份
- [ ] 钱包私钥安全存储
- [ ] 所有依赖都是最新版本

---

**最后更新**: 2025-12-24
**适用版本**: SolSniper Pro v2.0.0
