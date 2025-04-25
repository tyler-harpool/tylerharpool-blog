#!/bin/bash

# Define base path for the structure
BASE_PATH="content/blog"

# Create the folder structure under each Area and Category
create_folders() {
  mkdir -p "$BASE_PATH/10-19 - Technology & Development/11 Programming Languages"
  mkdir -p "$BASE_PATH/10-19 - Technology & Development/12 Web Frameworks"
  mkdir -p "$BASE_PATH/10-19 - Technology & Development/13 Software Architecture"
  mkdir -p "$BASE_PATH/10-19 - Technology & Development/14 WebAssembly"

  mkdir -p "$BASE_PATH/20-29 - Digital Infrastructure/21 Cloud Platforms"
  mkdir -p "$BASE_PATH/20-29 - Digital Infrastructure/22 DevOps"  # Fixed folder name
  mkdir -p "$BASE_PATH/20-29 - Digital Infrastructure/23 Containerization"

  mkdir -p "$BASE_PATH/30-39 - Government & Policy/31 GovTech Initiatives"
  mkdir -p "$BASE_PATH/30-39 - Government & Policy/32 Digital Policy"
  mkdir -p "$BASE_PATH/30-39 - Government & Policy/33 Open Government"

  mkdir -p "$BASE_PATH/40-49 - Data & Analytics/41 Data Science"
  mkdir -p "$BASE_PATH/40-49 - Data & Analytics/42 Machine Learning"
  mkdir -p "$BASE_PATH/40-49 - Data & Analytics/43 Big Data"

  mkdir -p "$BASE_PATH/50-59 - Industry Insights/51 Case Studies"
  mkdir -p "$BASE_PATH/50-59 - Industry Insights/52 Tech Trends"
  mkdir -p "$BASE_PATH/50-59 - Industry Insights/53 Career Development"
}

# Create markdown files with expanded frontmatter for each item
create_markdown() {
  area_id=$1
  category_id=$2
  item_id=$3
  item_name=$4
  description=$5
  folder=$6

  # Check if the folder exists, and create it if necessary
  mkdir -p "$folder"

  # Create the markdown file with extended frontmatter
  cat > "$folder/$category_id.$item_id $item_name.md" <<EOL
---
id: $category_id.$item_id
area_id: $area_id
category_id: $category_id
title: $item_name
author: Tyler Harpool
date: $(date +'%Y-%m-%d')
summary: $description
tags: $7
thumbnail: /images/thumbnails/$item_id-thumbnail.jpg
draft: false
related_articles: []
---

# $item_name

$description
EOL
}

# Generate folder structure
create_folders

# Generate markdown files for Technology & Development
create_markdown 10 11 "01" "Introduction to Rust" "An introductory guide to the Rust programming language" "$BASE_PATH/10-19 - Technology & Development/11 Programming Languages" "[Rust Programming, Programming Languages]"
create_markdown 10 12 "01" "Building Web Apps with Leptos" "How to build web applications using Leptos" "$BASE_PATH/10-19 - Technology & Development/12 Web Frameworks" "[Web Development, Leptos, Web Frameworks]"
create_markdown 10 13 "01" "Software Design Principles" "Key principles and patterns in software architecture" "$BASE_PATH/10-19 - Technology & Development/13 Software Architecture" "[Software Architecture, Design Patterns]"
create_markdown 10 14 "01" "Introduction to WebAssembly" "Understanding and using WebAssembly in web apps" "$BASE_PATH/10-19 - Technology & Development/14 WebAssembly" "[WebAssembly, Web Development]"

# Generate markdown files for Digital Infrastructure
create_markdown 20 21 "01" "Google Cloud Platform: Getting Started" "A comprehensive guide to deploying your first application on GCP" "$BASE_PATH/20-29 - Digital Infrastructure/21 Cloud Platforms" "[Cloud Platforms, Google Cloud Platform, GCP]"
create_markdown 20 22 "01" "ArgoCD for Continuous Delivery" "A deep dive into setting up and using ArgoCD for automated continuous delivery" "$BASE_PATH/20-29 - Digital Infrastructure/22 DevOps" "[DevOps, ArgoCD, Continuous Delivery]"
create_markdown 20 23 "01" "Mastering Docker: Beyond the Basics" "An in-depth exploration of Docker's advanced features and optimizations" "$BASE_PATH/20-29 - Digital Infrastructure/23 Containerization" "[Docker, Containerization, DevOps]"

# Generate markdown files for Government & Policy
create_markdown 30 31 "01" "GovTech 101" "An overview of government technology initiatives" "$BASE_PATH/30-39 - Government & Policy/31 GovTech Initiatives" "[GovTech, Government, Technology Initiatives]"
create_markdown 30 32 "01" "Tech Policy & Regulation" "How technology policy is shaped and enforced" "$BASE_PATH/30-39 - Government & Policy/32 Digital Policy" "[Digital Policy, Technology, Regulation]"
create_markdown 30 33 "01" "Open Government & Transparency" "The importance of open data and transparency in government" "$BASE_PATH/30-39 - Government & Policy/33 Open Government" "[Open Government, Transparency, GovTech]"

# Generate markdown files for Data & Analytics
create_markdown 40 41 "01" "Introduction to Data Science" "Getting started with data analysis and visualization" "$BASE_PATH/40-49 - Data & Analytics/41 Data Science" "[Data Science, Analytics, Data Visualization]"
create_markdown 40 42 "01" "Machine Learning Basics" "An introduction to machine learning concepts and techniques" "$BASE_PATH/40-49 - Data & Analytics/42 Machine Learning" "[Machine Learning, Data Science, AI]"
create_markdown 40 43 "01" "Big Data Analytics" "Working with large-scale data and distributed systems" "$BASE_PATH/40-49 - Data & Analytics/43 Big Data" "[Big Data, Analytics, Data Science]"

# Generate markdown files for Industry Insights
create_markdown 50 51 "01" "Case Studies in Tech" "Real-world examples of technology implementations" "$BASE_PATH/50-59 - Industry Insights/51 Case Studies" "[Case Studies, Technology, Real-World]"
create_markdown 50 52 "01" "Tech Trends 2025" "Emerging technology trends and predictions for 2025" "$BASE_PATH/50-59 - Industry Insights/52 Tech Trends" "[Tech Trends, Emerging Technologies, Future Tech]"
create_markdown 50 53 "01" "Building a Tech Career" "Tips and advice for growing your career in technology" "$BASE_PATH/50-59 - Industry Insights/53 Career Development" "[Career Development, Technology, Jobs]"

echo "Folder structure and markdown files created!"
