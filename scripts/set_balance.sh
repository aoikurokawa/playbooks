#!/bin/bash
# Simple script to set balance using Surfpool cheatcodes
# This WORKS and doesn't require runbooks

set -e

ADDRESS="BgKUXdS29YcHCFrPm5M8oLHiTzZaMDjsebggjoaQ6KFL"
LAMPORTS=1
SURFPOOL_URL="http://127.0.0.1:8899"

echo "Setting balance for $ADDRESS to $LAMPORTS lamports"

# First, get the current account info (or use defaults if it doesn't exist)
ACCOUNT_INFO=$(curl -s -X POST "$SURFPOOL_URL" \
  -H "Content-Type: application/json" \
  -d "{
    \"jsonrpc\": \"2.0\",
    \"id\": 1,
    \"method\": \"getAccountInfo\",
    \"params\": [\"$ADDRESS\", {\"encoding\": \"base64\"}]
  }")

# Extract account details (or use defaults)
ACCOUNT_DATA=$(echo "$ACCOUNT_INFO" | jq -r '.result.value.data[0] // ""')
ACCOUNT_OWNER=$(echo "$ACCOUNT_INFO" | jq -r '.result.value.owner // "11111111111111111111111111111111"')
ACCOUNT_EXECUTABLE=$(echo "$ACCOUNT_INFO" | jq -r '.result.value.executable // false')
ACCOUNT_RENT_EPOCH=$(echo "$ACCOUNT_INFO" | jq -r '.result.value.rentEpoch // 0')

# Use surfnet_setAccount cheatcode
echo "Calling surfnet_setAccount..."

# Handle data field - if empty, use empty array, otherwise use the base64 data
if [ "$ACCOUNT_DATA" = "" ] || [ "$ACCOUNT_DATA" = "null" ]; then
    DATA_FIELD="[]"
else
    DATA_FIELD="[\"$ACCOUNT_DATA\", \"base64\"]"
fi

RESULT=$(curl -s -X POST "$SURFPOOL_URL" \
  -H "Content-Type: application/json" \
  -d "{
    \"jsonrpc\": \"2.0\",
    \"id\": 1,
    \"method\": \"surfnet_setAccount\",
    \"params\": [
      \"$ADDRESS\",
      {
        \"lamports\": $LAMPORTS,
        \"data\": $DATA_FIELD,
        \"owner\": \"$ACCOUNT_OWNER\",
        \"executable\": $ACCOUNT_EXECUTABLE,
        \"rentEpoch\": $ACCOUNT_RENT_EPOCH
      }
    ]
  }")

# Check for errors
ERROR=$(echo "$RESULT" | jq -r '.error // empty')
if [ -n "$ERROR" ]; then
    echo "❌ Error: $ERROR"
    echo "Full response: $RESULT"
    exit 1
fi

echo "✅ Balance set successfully!"

# Verify
sleep 0.5
NEW_BALANCE=$(curl -s -X POST "$SURFPOOL_URL" \
  -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getBalance\",\"params\":[\"$ADDRESS\"]}" \
  | jq -r '.result.value')

NEW_SOL=$(echo "scale=9; $NEW_BALANCE / 1000000000" | bc)
echo "Current balance: $NEW_SOL SOL ($NEW_BALANCE lamports)"
