# Session Handoff - 2026-03-02

> ⚠️ **已过时**: 本文档记录了 2026-03-02 的 MCP endpoint session。所有相关信息已合并到 [Production Plan](./production-plan.md)。后续开发跟踪请查看 production-plan.md 的 "Phase 0: 当前任务跟踪" 节。

## What Was Done

### 1. MCP Endpoint Implementation
- Implemented unified `/mcp` endpoint for external AI agents
- Added API Key authentication supporting both `Authorization: Bearer <key>` and `X-API-Key: <key>` headers
- Implemented tenant-based tool filtering
- Public tools (is_public=true) accessible without authentication
- Private tools require valid API Key with matching tenant_id

### 2. Bug Fixes
- Fixed duplicate `ToolStore` struct definition in `tool_handlers.rs`
- Fixed MCP route double-nesting issue in `routes/mcp.rs`
- Fixed `main.rs` syntax errors (duplicate code blocks)
- Fixed tool lookup to use name instead of UUID (MCP spec requirement)

### 3. Files Created/Modified
- `backend/crates/api/src/handlers/mcp_handlers.rs` - New MCP handler
- `backend/crates/api/src/routes/mcp.rs` - MCP route configuration
- `backend/crates/api/src/handlers/tool_handlers.rs` - Added `get_all()` method
- `backend/src/main.rs` - Added McpState initialization

## Current State

### Services
- Backend: Running on port 8080 (release build)
- Frontend: Running on port 3000

### Build Status
- `cargo build --release` succeeds with only warnings
- All MCP endpoints verified working

### Verified MCP Endpoints

| Endpoint | Method | Status |
|----------|--------|--------|
| `/health` | GET | ✅ Working |
| `/mcp` (initialize) | POST | ✅ Working |
| `/mcp` (tools/list) | POST | ✅ Working (returns 4 public tools) |
| `/mcp` (tools/call) | POST | ✅ Working |
| `/mcp` (private tool) | POST | ✅ Returns auth required |

## Next Steps

1. Create an API Key via `POST /api/v1/tenant/{tenant_id}/api-keys`
2. Test MCP endpoint with API Key to verify tenant-specific tool filtering
3. Implement actual tool execution (currently returns mock response)
4. Continue Phase 6 tasks: User registration, RBAC, Member invitation

## Technical Decisions

1. MCP uses tool **name** for lookup, not UUID (per MCP specification)
2. API Key supports dual header formats for flexibility
3. ToolStore uses in-memory storage for development (SQLite default)

## Restart Commands

```bash
# Kill and restart backend
lsof -ti:8080 | xargs kill -9
cd backend && nohup ./target/release/evolith > /tmp/evolith-backend.log 2>&1 &

# Check health
curl -s http://localhost:8080/health
```

## Test Commands

```bash
# MCP initialize
curl -s -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | jq .

# MCP tools/list
curl -s -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | jq .

# MCP tools/call
curl -s -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"filesystem_read","arguments":{"path":"/tmp/test.txt"}}}' | jq .

# Test private tool (should fail without auth)
curl -s -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"web_scraper","arguments":{"url":"https://example.com"}}}' | jq .
```

## Warnings/Gotchas

1. Several handler files had duplicate struct definitions - be careful with edits
2. The `main.rs` file was corrupted with duplicate code - clean version now in place
3. MCP endpoint returns 404 if route is not properly configured (check `routes/mcp.rs`)

## Related Documentation

- [Implementation Plan](./implementation-plan.md)
- [Multi-Tenant Design](./multi-tenant.md)
- [API Design](./api-design.md)