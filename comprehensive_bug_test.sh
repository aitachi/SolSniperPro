#!/bin/bash
# Comprehensive Bug Testing Script for SolSniper Pro API
# Tests all endpoints for bugs, edge cases, and errors

API_BASE="http://localhost:3000/api/v1"
HEALTH_URL="http://localhost:3000/api/v1/health"
WS_URL="ws://localhost:3000/ws"

echo "==================================================================="
echo "SolSniper Pro - Comprehensive Bug Testing"
echo "==================================================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS_COUNT=0
FAIL_COUNT=0
WARN_COUNT=0

# Test function
test_endpoint() {
    local name="$1"
    local url="$2"
    local method="${3:-GET}"
    local data="$4"
    local expected_status="${5:-200}"

    echo -n "Testing: $name ... "

    if [ "$method" = "GET" ]; then
        response=$(curl -s -w "\n%{http_code}" "$url")
    else
        response=$(curl -s -w "\n%{http_code}" -X "$method" -H "Content-Type: application/json" -d "$data" "$url")
    fi

    status_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | head -n-1)

    if [ "$status_code" = "$expected_status" ]; then
        echo -e "${GREEN}✓ PASS${NC} (Status: $status_code)"
        ((PASS_COUNT++))

        # Check if response is valid JSON
        if echo "$body" | python -m json.tool > /dev/null 2>&1; then
            : # JSON is valid
        else
            echo -e "${YELLOW}  ⚠ WARNING: Response is not valid JSON${NC}"
            ((WARN_COUNT++))
        fi
    else
        echo -e "${RED}✗ FAIL${NC} (Expected: $expected_status, Got: $status_code)"
        echo "  Response: $body"
        ((FAIL_COUNT++))
    fi
}

# Test for data consistency
test_data_consistency() {
    local name="$1"
    local url="$2"
    local field="$3"

    echo -n "Data Consistency: $name ... "

    response1=$(curl -s "$url")
    sleep 0.5
    response2=$(curl -s "$url")

    value1=$(echo "$response1" | python -c "import sys,json; data=json.load(sys.stdin); print(data['data']['$field'])" 2>/dev/null)
    value2=$(echo "$response2" | python -c "import sys,json; data=json.load(sys.stdin); print(data['data']['$field'])" 2>/dev/null)

    if [ "$value1" = "$value2" ]; then
        echo -e "${GREEN}✓ CONSISTENT${NC}"
        ((PASS_COUNT++))
    else
        echo -e "${YELLOW}⚠ VALUES CHANGE${NC} (Mock data may be randomized)"
        ((WARN_COUNT++))
    fi
}

echo "==================================================================="
echo "1. HEALTH CHECK ENDPOINT"
echo "==================================================================="
test_endpoint "Health Check" "$HEALTH_URL" "GET" "" "200"
echo ""

echo "==================================================================="
echo "2. AUTHENTICATION ENDPOINTS"
echo "==================================================================="
test_endpoint "Login with valid credentials" "$API_BASE/auth/login" "POST" '{"username":"admin","password":"admin123"}' "200"
test_endpoint "Login with invalid credentials" "$API_BASE/auth/login" "POST" '{"username":"admin","password":"wrong"}' "401"
test_endpoint "Login with missing fields" "$API_BASE/auth/login" "POST" '{"username":"admin"}' "400"
test_endpoint "Login with empty data" "$API_BASE/auth/login" "POST" '{}' "400"
test_endpoint "Logout" "$API_BASE/auth/logout" "POST" '{}' "200"
echo ""

echo "==================================================================="
echo "3. TOKEN ENDPOINTS"
echo "==================================================================="
test_endpoint "Get all tokens" "$API_BASE/tokens" "GET" "" "200"
test_endpoint "Get specific token (BONK)" "$API_BASE/tokens/5t1dC...abc123" "GET" "" "200"
test_endpoint "Get non-existent token" "$API_BASE/tokens/invalid_mint_address" "GET" "" "404"
test_data_consistency "Token list consistency" "$API_BASE/tokens" "total"
echo ""

echo "==================================================================="
echo "4. STRATEGY ENDPOINTS"
echo "==================================================================="
test_endpoint "Get all strategies" "$API_BASE/strategies" "GET" "" "200"
test_endpoint "Create new strategy" "$API_BASE/strategies" "POST" '{"name":"Test Strategy","type":"EarlyBird","config":{"min_liquidity":50}}' "200"
test_endpoint "Create strategy with invalid data" "$API_BASE/strategies" "POST" '{"name":""}' "400"
test_endpoint "Start strategy" "$API_BASE/strategies/strategy1/start" "POST" '{}' "200"
test_endpoint "Pause strategy" "$API_BASE/strategies/strategy1/pause" "POST" '{}' "200"
test_endpoint "Start non-existent strategy" "$API_BASE/strategies/invalid_id/start" "POST" '{}' "404"
echo ""

echo "==================================================================="
echo "5. POSITION ENDPOINTS"
echo "==================================================================="
test_endpoint "Get all positions" "$API_BASE/positions" "GET" "" "200"
test_endpoint "Get specific position" "$API_BASE/positions/pos1" "GET" "" "200"
test_endpoint "Get non-existent position" "$API_BASE/positions/invalid_id" "GET" "" "404"
test_endpoint "Close position" "$API_BASE/positions/pos1/close" "POST" '{}' "200"
test_endpoint "Close already closed position" "$API_BASE/positions/pos1/close" "POST" '{}' "400"
echo ""

echo "==================================================================="
echo "6. TRADE ENDPOINTS"
echo "==================================================================="
test_endpoint "Get all trades (default page)" "$API_BASE/trades" "GET" "" "200"
test_endpoint "Get trades with pagination" "$API_BASE/trades?page=1&limit=5" "GET" "" "200"
test_endpoint "Get trades with invalid page" "$API_BASE/trades?page=-1&limit=5" "GET" "" "400"
test_endpoint "Get specific trade" "$API_BASE/trades/trade1" "GET" "" "200"
test_endpoint "Get non-existent trade" "$API_BASE/trades/invalid_id" "GET" "" "404"
echo ""

echo "==================================================================="
echo "7. METRICS ENDPOINTS"
echo "==================================================================="
test_endpoint "Get metrics summary" "$API_BASE/metrics/summary" "GET" "" "200"
test_endpoint "Get system metrics" "$API_BASE/metrics/system" "GET" "" "200"
test_endpoint "Get strategy metrics" "$API_BASE/metrics/strategy/strategy1" "GET" "" "200"
test_endpoint "Get metrics for non-existent strategy" "$API_BASE/metrics/strategy/invalid_id" "GET" "" "404"

echo ""
echo "Testing system metrics variation (should have some randomness)..."
cpu1=$(curl -s "$API_BASE/metrics/system" | python -c "import sys,json; print(json.load(sys.stdin)['data']['cpu_usage'])" 2>/dev/null)
sleep 0.5
cpu2=$(curl -s "$API_BASE/metrics/system" | python -c "import sys,json; print(json.load(sys.stdin)['data']['cpu_usage'])" 2>/dev/null)

if [ "$cpu1" != "$cpu2" ]; then
    echo -e "${GREEN}✓ System metrics show variation (expected for mock data)${NC}"
    ((PASS_COUNT++))
else
    echo -e "${YELLOW}⚠ System metrics are static${NC}"
    ((WARN_COUNT++))
fi
echo ""

echo "==================================================================="
echo "8. RISK MANAGEMENT ENDPOINTS"
echo "==================================================================="
test_endpoint "Get risk limits" "$API_BASE/risk/limits" "GET" "" "200"
test_endpoint "Update risk limits" "$API_BASE/risk/limits" "PUT" '{"max_position_size_sol":15.0,"max_daily_loss_sol":20.0}' "200"
test_endpoint "Update risk limits with invalid data" "$API_BASE/risk/limits" "PUT" '{"max_position_size_sol":-10}' "400"
test_endpoint "Get risk status" "$API_BASE/risk/status" "GET" "" "200"
echo ""

echo "==================================================================="
echo "9. EDGE CASE TESTS"
echo "==================================================================="
echo -n "Testing: Malformed JSON ... "
response=$(curl -s -w "\n%{http_code}" -X POST -H "Content-Type: application/json" -d 'invalid json' "$API_BASE/auth/login")
status=$(echo "$response" | tail -n1)
if [ "$status" = "400" ] || [ "$status" = "500" ]; then
    echo -e "${GREEN}✓ PASS${NC} (Server handles malformed JSON)"
    ((PASS_COUNT++))
else
    echo -e "${RED}✗ FAIL${NC} (Unexpected status: $status)"
    ((FAIL_COUNT++))
fi

echo -n "Testing: Very large page number ... "
response=$(curl -s -w "\n%{http_code}" "$API_BASE/trades?page=999999&limit=10")
status=$(echo "$response" | tail -n1)
if [ "$status" = "200" ]; then
    echo -e "${GREEN}✓ PASS${NC} (Server handles large page numbers)"
    ((PASS_COUNT++))
else
    echo -e "${YELLOW}⚠ WARNING${NC} (Status: $status)"
    ((WARN_COUNT++))
fi

echo -n "Testing: Zero limit pagination ... "
response=$(curl -s -w "\n%{http_code}" "$API_BASE/trades?page=1&limit=0")
status=$(echo "$response" | tail -n1)
if [ "$status" = "400" ]; then
    echo -e "${GREEN}✓ PASS${NC} (Server validates limit parameter)"
    ((PASS_COUNT++))
else
    echo -e "${YELLOW}⚠ WARNING${NC} (Status: $status, expected 400)"
    ((WARN_COUNT++))
fi

echo -n "Testing: SQL injection attempt ... "
response=$(curl -s -w "\n%{http_code}" "$API_BASE/tokens/'; DROP TABLE tokens; --")
status=$(echo "$response" | tail -n1)
if [ "$status" = "404" ] || [ "$status" = "400" ]; then
    echo -e "${GREEN}✓ PASS${NC} (Server handles SQL injection safely)"
    ((PASS_COUNT++))
else
    echo -e "${RED}✗ FAIL${NC} (Unexpected behavior: $status)"
    ((FAIL_COUNT++))
fi

echo ""

echo "==================================================================="
echo "10. WEBSOCKET CONNECTION TEST"
echo "==================================================================="
echo "Testing WebSocket connection (checking if server accepts WS connections)..."

# Try to connect to WebSocket using Node.js test script
if [ -f "/c/Users/ASUS/Desktop/B-partjob/solsinapor/SolSniperPro-main/test_websocket.js" ]; then
    cd /c/Users/ASUS/Desktop/B-partjob/solsinapor/SolSniperPro-main
    timeout 5 node test_websocket.js > /tmp/ws_test.log 2>&1 &
    WS_PID=$!
    sleep 2

    if ps -p $WS_PID > /dev/null 2>&1; then
        echo -e "${GREEN}✓ WebSocket connection successful${NC}"
        kill $WS_PID 2>/dev/null
        ((PASS_COUNT++))
    else
        if grep -q "connected" /tmp/ws_test.log 2>/dev/null; then
            echo -e "${GREEN}✓ WebSocket connection successful${NC}"
            ((PASS_COUNT++))
        else
            echo -e "${RED}✗ WebSocket connection failed${NC}"
            cat /tmp/ws_test.log 2>/dev/null
            ((FAIL_COUNT++))
        fi
    fi
else
    echo -e "${YELLOW}⚠ WebSocket test script not found${NC}"
    ((WARN_COUNT++))
fi

echo ""
echo "==================================================================="
echo "SUMMARY"
echo "==================================================================="
echo -e "${GREEN}Passed: $PASS_COUNT${NC}"
echo -e "${RED}Failed: $FAIL_COUNT${NC}"
echo -e "${YELLOW}Warnings: $WARN_COUNT${NC}"
echo ""

TOTAL=$((PASS_COUNT + FAIL_COUNT + WARN_COUNT))
if [ $TOTAL -gt 0 ]; then
    PASS_RATE=$((PASS_COUNT * 100 / TOTAL))
    echo "Pass Rate: $PASS_RATE%"
fi

echo ""
if [ $FAIL_COUNT -eq 0 ]; then
    echo -e "${GREEN}✓ All critical tests passed!${NC}"
    if [ $WARN_COUNT -gt 0 ]; then
        echo -e "${YELLOW}⚠ There are $WARN_COUNT warnings that may need attention${NC}"
    fi
    exit 0
else
    echo -e "${RED}✗ Found $FAIL_COUNT failing tests${NC}"
    exit 1
fi
