#!/bin/bash
# Final working version:
# - Uses raw bytes for data (not base64 string)
# - Takes lamports as input (not SOL)

set -e

ADDRESS="${1:-BgKUXdS29YcHCFrPm5M8oLHiTzZaMDjsebggjoaQ6KFL}"
LAMPORTS="${2:-1000000000000}"  # Default: 1000 SOL
SURFPOOL_URL="http://127.0.0.1:8899"

SOL_AMOUNT=$(echo "scale=9; $LAMPORTS / 1000000000" | bc)
echo "🎯 Setting EXACT balance for $ADDRESS"
echo "   Target: $LAMPORTS lamports ($SOL_AMOUNT SOL)"
echo ""

# Get current balance
CURRENT=$(curl -s -X POST "$SURFPOOL_URL" \
  -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getBalance\",\"params\":[\"$ADDRESS\"]}" \
  | jq -r '.result.value')
CURRENT_SOL=$(echo "scale=9; $CURRENT / 1000000000" | bc)
echo "Current balance: $CURRENT lamports ($CURRENT_SOL SOL)"

# Get account info with jsonParsed encoding to get raw data
echo "Fetching account info..."
ACCOUNT_INFO=$(curl -s -X POST "$SURFPOOL_URL" \
  -H "Content-Type: application/json" \
  -d "{
    \"jsonrpc\": \"2.0\",
    \"id\": 1,
    \"method\": \"getAccountInfo\",
    \"params\": [\"$ADDRESS\", {\"encoding\": \"base64\"}]
  }")

ACCOUNT_VALUE=$(echo "$ACCOUNT_INFO" | jq -r '.result.value')
if [ "$ACCOUNT_VALUE" = "null" ]; then
    echo "❌ Account not found"
    exit 1
fi

# Extract account details
ACCOUNT_DATA_BASE64=$(echo "$ACCOUNT_INFO" | jq -r '.result.value.data[0]')
ACCOUNT_OWNER=$(echo "$ACCOUNT_INFO" | jq -r '.result.value.owner')
ACCOUNT_EXECUTABLE=$(echo "$ACCOUNT_INFO" | jq -r '.result.value.executable')
ACCOUNT_RENT_EPOCH=$(echo "$ACCOUNT_INFO" | jq -r '.result.value.rentEpoch')

echo "Account owner: $ACCOUNT_OWNER"
echo "Data length: ${#ACCOUNT_DATA_BASE64} bytes (base64)"
echo ""

# Decode base64 to raw bytes array
# Convert base64 string to byte array for JSON
ACCOUNT_DATA_BYTES=$(echo "$ACCOUNT_DATA_BASE64" | base64 -d | od -An -td1 | tr -d '\n' | sed 's/^ *//' | sed 's/  */,/g')

# Build JSON with raw byte array for data
JSON_PAYLOAD=$(cat <<EOF
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "surfnet_setAccount",
  "params": [
    "$ADDRESS",
    {
      "lamports": $LAMPORTS,
      "data": [$ACCOUNT_DATA_BYTES],
      "owner": "$ACCOUNT_OWNER",
      "executable": $ACCOUNT_EXECUTABLE,
      "rentEpoch": $ACCOUNT_RENT_EPOCH
    }
  ]
}
EOF
)

echo "Calling surfnet_setAccount cheatcode..."
RESULT=$(curl -s -X POST "$SURFPOOL_URL" \
  -H "Content-Type: application/json" \
  -d "$JSON_PAYLOAD")

# Check for errors
ERROR=$(echo "$RESULT" | jq -r '.error // empty')
if [ -n "$ERROR" ]; then
    echo "❌ Error: $(echo "$RESULT" | jq -r '.error.message')"
    echo ""
    echo "Full result:"
    echo "$RESULT" | jq '.'
    exit 1
fi

echo "✅ Balance set successfully!"
echo ""

# Verify
sleep 0.5
NEW_BALANCE=$(curl -s -X POST "$SURFPOOL_URL" \
  -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getBalance\",\"params\":[\"$ADDRESS\"]}" \
  | jq -r '.result.value')

NEW_SOL=$(echo "scale=9; $NEW_BALANCE / 1000000000" | bc)
DIFF=$(echo "scale=9; ($NEW_BALANCE - $CURRENT) / 1000000000" | bc)

echo "✓ Old balance: $CURRENT lamports ($CURRENT_SOL SOL)"
echo "✓ New balance: $NEW_BALANCE lamports ($NEW_SOL SOL)"
echo "✓ Changed by: $(echo "$NEW_BALANCE - $CURRENT" | bc) lamports ($DIFF SOL)"
