# ADR-0007: SQLite and SQLx for Local Persistence

- **Status:** Accepted
- **Date:** 2026-11-24

## Context

The initial product is a single-user local application. It needs durable task state, approvals, tool results, and audit metadata without requiring a database server.

## Decision

**Use SQLite accessed through SQLx for local persistent state.**

## Alternatives considered

- PostgreSQL server.
- JSON files as the primary database.
- Remote managed database.

## Consequences

- The application owns schema migrations.
- Use transactions for task state and audit updates that must remain consistent.
- Store large screenshots/artifacts in a controlled local artifact directory and persist their path/hash/metadata in SQLite.
- Keep secrets out of database records.
- PostgreSQL or a remote database is outside the initial scope and requires a new ADR.
