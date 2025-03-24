-- Drop existing tables
DROP TABLE IF EXISTS project_technologies;
DROP TABLE IF EXISTS projects;
DROP TABLE IF EXISTS jd_categories;
DROP TABLE IF EXISTS jd_areas;

-- Create Johnny Decimal areas table
CREATE TABLE jd_areas (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL
);

-- Create Johnny Decimal categories table
CREATE TABLE jd_categories (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    area_id INTEGER NOT NULL,
    FOREIGN KEY (area_id) REFERENCES jd_areas(id)
);

-- Create projects table with proper foreign key to categories
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
    jd_category_id INTEGER,
    FOREIGN KEY (jd_category_id) REFERENCES jd_categories(id)
);

-- Create project technologies junction table
CREATE TABLE project_technologies (
    project_id INTEGER NOT NULL,
    technology TEXT NOT NULL,
    PRIMARY KEY (project_id, technology),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX idx_projects_slug ON projects(slug);
CREATE INDEX idx_projects_category ON projects(jd_category_id);
CREATE INDEX idx_categories_area ON jd_categories(area_id);

-- Insert Johnny Decimal areas
INSERT INTO jd_areas (id, name, description) VALUES
(1, 'Web Development', 'Technologies and frameworks for building web applications'),
(2, 'Infrastructure', 'Cloud, servers, and deployment technologies'),
(3, 'Government & Public Sector', 'Digital solutions for government and public services');

-- Insert Johnny Decimal categories
INSERT INTO jd_categories (id, name, description, area_id) VALUES
(12, 'Web Frameworks', 'Front-end and back-end frameworks for building web applications', 1),
(13, 'Software Architecture', 'Design patterns and architectural approaches for software', 1),
(23, 'Containerization', 'Docker, Kubernetes, and container orchestration', 2),
(31, 'GovTech Initiatives', 'Technology initiatives in the government sector', 3);

-- Insert mock projects
INSERT INTO projects (title, slug, summary, content, repo_url, live_url, thumbnail, created_at, updated_at, jd_category_id)
VALUES
('Building a Modern Web App with Leptos and Rust',
'building-with-leptos',
'An exploration of Rust''s web framework ecosystem and how Leptos is pushing the boundaries.',
'# Project Details\n\nThis project was built using Rust and Leptos...\n\n## Features\n\n* Full-stack Rust development\n* Component-based UI with Leptos\n* Reactive state management\n* SSR with hydration\n* Type-safe server functions',
'https://github.com/tylerharpool/building-with-leptos',
'https://building-with-leptos.tylerharpool.com',
NULL,
'2023-11-01T00:00:00Z',
'2023-11-01T00:00:00Z',
12),

('Implementing Islands Architecture in a Rust Web Framework',
'islands-architecture',
'How partial hydration can improve performance while maintaining interactivity.',
'# Islands Architecture\n\nThis project demonstrates implementing the Islands Architecture pattern...\n\n## Benefits\n\n* Reduced JavaScript payload\n* Faster page loads\n* Better performance on mobile devices\n* Progressive enhancement',
'https://github.com/tylerharpool/islands-architecture',
'https://islands-architecture.tylerharpool.com',
NULL,
'2023-11-20T00:00:00Z',
'2023-11-20T00:00:00Z',
13),

('Server Functions: Bridging the Frontend-Backend Divide',
'server-functions',
'Using Rust on both ends of the stack to create a seamless development experience.',
'# Server Functions\n\nThis project demonstrates how server functions can unify front and back end development...\n\n## Technical Details\n\n* Type-safe RPC between client and server\n* Automatic serialization/deserialization\n* Error handling across the boundary\n* Authentication integration',
'https://github.com/tylerharpool/server-functions',
'https://server-functions.tylerharpool.com',
NULL,
'2023-12-05T00:00:00Z',
'2023-12-05T00:00:00Z',
12),

('Cloud-Native Deployment Strategies',
'cloud-native-deployment',
'Best practices for deploying applications in cloud environments with minimal downtime.',
'# Cloud-Native Deployment\n\nThis article explores various strategies for deploying applications in cloud environments...\n\n## Topics Covered\n\n* Blue-green deployments\n* Canary releases\n* Progressive rollouts\n* Rollback strategies\n* Monitoring during deployment',
NULL,
NULL,
NULL,
'2024-01-10T00:00:00Z',
'2024-01-10T00:00:00Z',
23),

('Government Digital Transformation Initiatives',
'gov-digital-transformation',
'How governments are leveraging technology to improve service delivery and citizen engagement.',
'# Government Digital Transformation\n\nThis article examines recent initiatives in government digital transformation...\n\n## Case Studies\n\n* Digital service standards\n* Citizen-centric design\n* Legacy system modernization\n* Cross-agency collaboration\n* Measuring digital transformation success',
NULL,
NULL,
NULL,
'2024-02-15T00:00:00Z',
'2024-02-15T00:00:00Z',
31);

-- Add technologies for the projects
-- Project 1: Building a Modern Web App with Leptos and Rust
INSERT INTO project_technologies (project_id, technology) VALUES
(1, 'Rust'),
(1, 'Leptos'),
(1, 'WebAssembly');

-- Project 2: Implementing Islands Architecture
INSERT INTO project_technologies (project_id, technology) VALUES
(2, 'Rust'),
(2, 'Leptos'),
(2, 'Islands');

-- Project 3: Server Functions
INSERT INTO project_technologies (project_id, technology) VALUES
(3, 'Rust'),
(3, 'Leptos'),
(3, 'Axum');

-- Project 4: Cloud-Native Deployment Strategies
INSERT INTO project_technologies (project_id, technology) VALUES
(4, 'AWS'),
(4, 'Docker'),
(4, 'Kubernetes');

-- Project 5: Government Digital Transformation Initiatives
INSERT INTO project_technologies (project_id, technology) VALUES
(5, 'GovTech'),
(5, 'Digital Services');
