#!/bin/bash
# Simplest approach - use Solana's built-in airdrop
# This adds to existing balance instead of setting exact amount

set -e

ADDRESS="${1:-BgKUXdS29YcHCFrPm5M8oLHiTzZaMDjsebggjoaQ6KFL}"
LAMPORTS="${2:-1000}"
SURFPOOL_URL="http://127.0.0.1:8899"

echo "💰 Airdropping $SOL_AMOUNT SOL to $ADDRESS"

# LAMPORTS=$((SOL_AMOUNT * 1000000000))

# Use the standard requestAirdrop method
RESULT=$(curl -s -X POST "$SURFPOOL_URL" \
  -H "Content-Type: application/json" \
  -d "{
    \"jsonrpc\": \"2.0\",
    \"id\": 1,
    \"method\": \"requestAirdrop\",
    \"params\": [
      \"$ADDRESS\",
      $LAMPORTS
    ]
  }")

# Check for errors
ERROR=$(echo "$RESULT" | jq -r '.error // empty')
if [ -n "$ERROR" ]; then
    echo "❌ Error: $ERROR"
    exit 1
fi

SIGNATURE=$(echo "$RESULT" | jq -r '.result')
echo "✅ Airdrop requested!"
echo "   Signature: $SIGNATURE"

# Wait a moment for confirmation
sleep 1

# Check balance
BALANCE=$(curl -s -X POST "$SURFPOOL_URL" \
  -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getBalance\",\"params\":[\"$ADDRESS\"]}" \
  | jq -r '.result.value')

SOL=$(echo "scale=9; $BALANCE / 1000000000" | bc)
echo "   Current balance: $SOL SOL"
