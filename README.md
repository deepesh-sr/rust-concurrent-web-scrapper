# Rust Web Scraper

A concurrent web scraper built in Rust using async/await. Scrapes multiple URLs simultaneously and extracts all hyperlinks from each page.

## Features

- Concurrent scraping using `tokio::spawn`
- HTTP requests with browser-like User-Agent header to bypass basic bot detection
- HTML parsing with CSS selectors
- Graceful error handling per URL (one failure doesn't stop others)

## Tech Stack

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.50 | Async runtime (multi-thread) |
| `reqwest` | 0.13 | HTTP client |
| `scraper` | 0.25 | HTML parsing via CSS selectors |

## Project Structure

```
src/
└── main.rs
    ├── main()        — spawns concurrent tasks for each URL
    ├── scrape()      — combines fetch + parse
    ├── fetch_page()  — HTTP GET with User-Agent header
    └── parse_links() — extracts all <a href> from HTML
```

## Getting Started

```bash
git clone <repo-url>
cd rust-web-scrapper
cargo run
```

## How It Works

1. A list of URLs is defined in `main()`
2. Each URL gets its own `tokio::spawn` task — all run concurrently
3. `fetch_page` makes an HTTP request and returns raw HTML
4. `parse_links` uses CSS selector `"a"` to extract all `href` attributes
5. Results are collected and printed; errors are reported per URL

## Example Output

```
["/", "/events", "https://lu.ma/solana", "/news", ...]
[]   <- bot-blocked site returns empty
```

## Known Limitations

- Sites with advanced bot detection (Amazon, Myntra) may return empty results
- No recursive scraping — only scrapes the given URL, not discovered links
- Results printed to stdout only — no file export

## Future Scope

- [ ] Recursive scraping — follow discovered links up to configurable depth
- [ ] CSV/JSON export — save results to file instead of stdout
- [ ] CLI interface — accept URLs as arguments using `clap`
- [ ] Rate limiting — add delay between requests to avoid getting blocked
- [ ] Proxy support — rotate proxies to bypass bot detection
- [ ] Specific data extraction — scrape titles, prices, images (not just links)
- [ ] robots.txt respect — check crawl permissions before scraping
- [ ] Retry logic — retry failed requests with exponential backoff
- [ ] Headless browser — use `chromiumoxide` for JavaScript-rendered pages

## Learnings

- Async/await with Tokio
- Concurrent task spawning with `tokio::spawn`
- `Send + Sync` error types for multi-threaded contexts
- `if let Some()` pattern matching
- HTML parsing with CSS selectors
- User-Agent spoofing for basic bot bypass
