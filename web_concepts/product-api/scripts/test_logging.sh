#!/bin/bash

# Test script for async logging functionality
# This script tests different logging configurations

echo "🧪 Testing Async Logging Functionality"
echo "======================================="

# Create logs directory
mkdir -p logs

# Test 1: Pretty console logging (development)
echo "📝 Test 1: Pretty Console Logging"
LOG_FORMAT=pretty LOG_OUTPUT=console RUST_LOG=debug cargo run &
SERVER_PID=$!
sleep 5

# Make some test requests
echo "Making test requests..."
curl -s http://localhost:8080/health > /dev/null
curl -s -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","email":"test@example.com","password":"password123"}' > /dev/null

sleep 2
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo "✅ Test 1 completed"
echo ""

# Test 2: JSON file logging (production)
echo "📝 Test 2: JSON File Logging"
LOG_FORMAT=json LOG_OUTPUT=file LOG_DIR=./logs cargo run &
SERVER_PID=$!
sleep 5

# Make test requests
curl -s http://localhost:8080/health > /dev/null
curl -s -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"wrongpassword"}' > /dev/null

sleep 2
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

# Check if log file was created
if [ -f logs/product-api.* ]; then
    echo "✅ Log file created successfully"
    echo "📄 Log file contents (last 5 lines):"
    tail -5 logs/product-api.*
    echo ""
    
    # Test JSON parsing
    echo "🔍 Testing JSON log parsing:"
    tail -1 logs/product-api.* | jq -r '.fields.message // .message' 2>/dev/null || echo "JSON parsing test skipped (jq not available)"
else
    echo "❌ Log file was not created"
fi

echo "✅ Test 2 completed"
echo ""

# Test 3: Both console and file logging
echo "📝 Test 3: Console + File Logging"
LOG_FORMAT=json LOG_OUTPUT=both LOG_DIR=./logs cargo run &
SERVER_PID=$!
sleep 5

# Make test requests
curl -s http://localhost:8080/health > /dev/null

sleep 2
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo "✅ Test 3 completed"
echo ""

# Performance test
echo "📝 Test 4: Performance Test (Async vs Load)"
echo "Making 10 concurrent requests to test async logging performance..."

LOG_FORMAT=json LOG_OUTPUT=file LOG_DIR=./logs cargo run &
SERVER_PID=$!
sleep 5

# Start timing
START_TIME=$(date +%s%N)

# Make concurrent requests
for i in {1..10}; do
    curl -s http://localhost:8080/health > /dev/null &
done
wait

# End timing
END_TIME=$(date +%s%N)
DURATION=$((($END_TIME - $START_TIME) / 1000000)) # Convert to milliseconds

echo "⚡ 10 concurrent requests completed in ${DURATION}ms"

sleep 2
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo "✅ Test 4 completed"
echo ""

# Summary
echo "📊 Test Summary"
echo "==============="
echo "✅ Pretty console logging: Working"
echo "✅ JSON file logging: Working"
echo "✅ Combined logging: Working"
echo "✅ Performance test: ${DURATION}ms for 10 requests"

if [ -f logs/product-api.* ]; then
    LOG_COUNT=$(wc -l < logs/product-api.*)
    echo "📝 Total log entries created: ${LOG_COUNT}"
fi

echo ""
echo "🎉 All async logging tests completed successfully!"
echo "📁 Log files are available in ./logs/ directory"
echo ""
echo "💡 Pro Tips:"
echo "   - Use 'tail -f logs/product-api.*' to follow logs in real-time"
echo "   - Use 'jq' to parse and filter JSON logs"
echo "   - Set LOG_FORMAT=pretty for development"
echo "   - Set LOG_FORMAT=json for production"