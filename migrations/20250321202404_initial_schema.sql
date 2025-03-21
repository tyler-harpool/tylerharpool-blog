-- Add migration script here
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    summary TEXT NOT NULL,
    content TEXT NOT NULL,
    tech_stack TEXT NOT NULL,
    repo_url TEXT,
    live_url TEXT,
    thumbnail TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    jd_category_id INTEGER
);

-- Create some indexes to improve query performance
CREATE INDEX idx_projects_slug ON projects(slug);
CREATE INDEX idx_projects_category ON projects(jd_category_id);

-- Create blog posts table
CREATE TABLE IF NOT EXISTS blog_posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    summary TEXT NOT NULL,
    content TEXT NOT NULL,
    tags TEXT NOT NULL, -- JSON array stored as text
    image_url TEXT,
    published_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    status TEXT NOT NULL, -- 'draft', 'published'
    jd_category_id INTEGER
);

-- Create some indexes to improve query performance for blog posts
CREATE INDEX idx_blog_posts_slug ON blog_posts(slug);
CREATE INDEX idx_blog_posts_status ON blog_posts(status);
CREATE INDEX idx_blog_posts_category ON blog_posts(jd_category_id);
CREATE INDEX idx_blog_posts_published_at ON blog_posts(published_at);
