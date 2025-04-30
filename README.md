# Eos: A Johnny Decimal Blog System 🌅

A modern, organized personal blog system built with Leptos, Rust, and the Johnny Decimal organizational framework. Named after Eos, the Greek goddess of dawn, bringing light to your thoughts and ideas.

![Blog Screenshot](https://example.com/blog-screenshot.png)

## 🌟 Features

- **Blazing Fast Performance**: Server-side rendering with client-side hydration for optimal speed
- **Johnny Decimal Organization**: Content structured using the [Johnny Decimal system](https://johnnydecimal.com/) for intuitive navigation
- **Rich Content**: Markdown-based content with YAML frontmatter
- **Interactive UI**: Islands architecture for efficient interactivity
- **Related Articles**: Smart content recommendations based on categories and tags
- **Responsive Design**: Mobile-friendly layout that works on all devices

## 🏗️ Technical Architecture

### Technology Stack

- **Frontend Framework**: [Leptos](https://leptos.dev/) - A Rust-based reactive web framework
- **Language**: [Rust](https://www.rust-lang.org/) for type-safe, high-performance code
- **Content Format**: Markdown with YAML frontmatter
- **Styling**: SCSS for maintainable CSS
- **Deployment**: Containerized with Docker, deployed on [Fly.io](https://fly.io)

### Code Structure

```
eos-blog/
├── Cargo.toml           # Rust dependencies and configuration
├── src/
│   ├── app.rs           # Main application component
│   ├── components/      # Reusable UI components
│   ├── model.rs         # Data structures and models
│   ├── pages/           # Page components
│   └── utils/           # Utility functions and helpers
├── content/
│   └── blog/            # Blog content organized by Johnny Decimal
│       ├── 10-19 - Technology & Development/
│       ├── 20-29 - Digital Infrastructure/
│       ├── 30-39 - Government & Policy/
│       ├── 40-49 - Data & Analytics/
│       └── 50-59 - Industry Insights/
└── style/               # SCSS stylesheets
```

## 📂 Content Organization

The content follows the Johnny Decimal system, organizing articles by area and category:
- **NUMBER-NUMBER AREA**: CATEGORY
- **10-19 Technology & Development**: Programming languages, frameworks, technical concepts
- **20-29 Digital Infrastructure**: Cloud services, DevOps, system architecture
- **30-39 Government & Policy**: GovTech initiatives, digital policy
- **40-49 Data & Analytics**: Data science, machine learning, analytics
- **50-59 Industry Insights**: Trends, case studies, industry analysis


## 📝 Creating Content

### Markdown Format

Articles are written in Markdown with YAML frontmatter:

```markdown
---
id: 11.01
area_id: 10
category_id: 11
title: Article Title
author: Tyler Harpool
date: 2025-04-29
summary: Brief summary of the article.
tags: [Tag1, Tag2, Tag3]
thumbnail: /images/thumbnails/article-thumbnail.jpg
draft: false
related_articles: ["12.01", "13.01"]
---

# Article Title

Article content begins here...
```

### Johnny Decimal IDs

Every article is assigned a Johnny Decimal ID (e.g., `11.01`):
- First two digits (`11`) represent the category
- Digits after the decimal point (`01`) represent the specific article
- Categories are grouped into areas (e.g., categories `10-19` belong to area `10`)

## 🚀 Deployment

The site is containerized using Docker and deployed on Fly.io:

```bash
# Deploy to Fly.io
flyctl deploy
```

## 🛠️ Development

### Prerequisites

- Rust (nightly)
- Node.js (for SASS compilation)
- Cargo

### Setup

```bash
# Clone the repository
git clone https://github.com/tyler-harpool/eos-blog.git
cd eos-blog

# Install dependencies
cargo install cargo-leptos

# Run the development server
cargo leptos watch
```

## 🧩 Architecture Details

### Component Hierarchy

- **App**: Main component that sets up routing and context
- **Pages**: Individual page components (Home, About, Project, Areas, etc.)
- **Components**: Reusable UI elements (Header, ProjectSearch, etc.)

### Data Flow

1. **Content Loading**: Markdown files are parsed and converted to `Project` structs
2. **Context Provision**: Projects, areas, and categories are provided via Leptos context
3. **Rendering**: Components access data through context and render content
4. **Interactivity**: Islands architecture enables client-side interactivity where needed

## 📈 Performance

The site is optimized for performance:
- Server-side rendering for fast initial load
- Selective hydration for interactive components
- Optimized assets (CSS, images)
- Efficient data loading and caching

## 👥 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

Built with ❤️ using Rust and Leptos
