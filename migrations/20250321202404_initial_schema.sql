DROP TABLE IF EXISTS project_technologies;
DROP TABLE IF EXISTS projects;

-- Basic projects table
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    summary TEXT NOT NULL,
    content TEXT NOT NULL,
    repo_url TEXT,
    live_url TEXT,
    thumbnail TEXT,
    created_at TEXT NOT NULL,  -- ISO8601 format
    updated_at TEXT NOT NULL,  -- ISO8601 format
    jd_category_id INTEGER
);

-- Simple junction table for project technologies
CREATE TABLE project_technologies (
    project_id INTEGER NOT NULL,
    technology TEXT NOT NULL,
    PRIMARY KEY (project_id, technology),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Add some basic indexes
CREATE INDEX idx_projects_slug ON projects(slug);
CREATE INDEX idx_projects_category ON projects(jd_category_id);
