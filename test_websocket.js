// WebSocket 测试脚本
const WebSocket = require('ws');

console.log('连接到 WebSocket 服务器...');
const ws = new WebSocket('ws://localhost:3000/ws?token=test_token_123');

ws.on('open', function open() {
  console.log('✅ WebSocket 连接成功');

  // 发送测试消息
  ws.send(JSON.stringify({
    type: 'test',
    data: { message: 'Hello from test client' }
  }));
  console.log('📤 发送测试消息');
});

ws.on('message', function incoming(data) {
  const message = JSON.parse(data.toString());
  console.log('📥 收到消息:', JSON.stringify(message, null, 2));
});

ws.on('error', function error(err) {
  console.error('❌ WebSocket 错误:', err.message);
});

ws.on('close', function close() {
  console.log('🔌 WebSocket 连接已关闭');
});

// 运行 10 秒后关闭
setTimeout(() => {
  console.log('\n⏱️  10秒测试完成，关闭连接...');
  ws.close();
  process.exit(0);
}, 10000);
