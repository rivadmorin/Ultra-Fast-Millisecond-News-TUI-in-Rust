# Real-Time News Aggregator Architecture

## Overview
A high-performance, resilient, and resource-efficient news ingestion system built in Rust. It leverages asynchronous I/O to handle hundreds of sources simultaneously with minimal overhead.

## Core Components

### 1. Asynchronous Ingestor (Fetcher)
- **Engine**: Powered by `tokio` and `reqwest`.
- **Concurrency**: Uses `Semaphore` to control the number of simultaneous workers, preventing OS resource exhaustion.
- **Efficiency**: Implements HTTP Conditional GET (`ETag`, `If-Modified-Since`) to skip redundant downloads.
- **Resilience**:
    - **Dynamic Header Rotation**: Cycles through modern `User-Agent` strings to mimic browser behavior.
    - **Exponential Backoff**: Automatically slows down when encountering HTTP 429 or 5xx errors.

### 2. Data Cleansing Pipeline
- **Module**: `src/cleanser.rs`
- **Parsing**: Standardizes diverse RSS/Atom formats into a unified internal schema.
- **Sanitization**:
    - Strips empty/redundant HTML tags.
    - Normalizes whitespace and character encoding.
    - Converts HTML summaries to clean Markdown or plain text.

### 3. Storage Layer (SQLite)
- **Performance**: Uses WAL (Write-Ahead Logging) and prepared transactions for fast inserts.
- **Optimization**: Indexes on `url`, `timestamp`, and `category` for sub-millisecond UI queries.
- **Maintenance**: Automated `VACUUM` and retention policy enforcement (e.g., auto-deleting news older than 24h).

## Data Schema (Standardized JSON-Compatible)
```json
{
  "title": "String",
  "source": "String",
  "url": "String",
  "published_at": "UnixTimestamp",
  "content_summary": "String (Markdown/Plain)",
  "category": "String"
}
```

## Scalability
The system is designed to handle thousands of RSS feeds. By using a worker-pool pattern and efficient database transactions, it maintains a low memory footprint (< 50MB) even under load.
