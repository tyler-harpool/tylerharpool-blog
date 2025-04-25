# Search & Navigation Guide

## Finding Information in Your Johnny Decimal System

This guide will help you effectively search and navigate your Johnny Decimal system in Obsidian.

## Quick Navigation Methods

### Using the Johnny Decimal ID

The most efficient way to navigate is using the Johnny Decimal ID numbers:

1. Open Obsidian's Quick Switcher: `Ctrl/Cmd + O`
2. Type the ID number (e.g., `21.03`)
3. Press Enter to open the file

This method works because the ID numbers are unique throughout your system.

### Using the File Explorer

The hierarchical structure allows for intuitive browsing:

1. Areas (10-19, 20-29, etc.)
2. Categories within areas (11, 12, 21, etc.)
3. Individual items with decimal IDs

### Using the Graph View

The graph view can reveal connections between your notes:

1. Open Graph View: `Ctrl/Cmd + G`
2. Filter by area using tags like `#work` or `#personal`
3. Look for clusters of related notes

## Advanced Search Techniques

### Basic Search

Use Obsidian's search function (`Ctrl/Cmd + F`) with the following patterns:

- `file:11` - Find all files in category 11
- `file:11.01` - Find a specific item
- `path:20-29` - Find all files in the Work area

### Tag-Based Search

Use tags to create cross-cutting views:

- `tag:#active tag:#work` - All active work items
- `tag:#project tag:#high-priority` - High-priority projects
- `tag:#waiting` - Items waiting for action

### Dataview Queries

For Obsidian users with the Dataview plugin, create powerful custom views:

```dataview
TABLE file.ctime as "Created"
FROM "20-29 - Work"
WHERE contains(tags, "active")
SORT file.ctime DESC
```

## Creating Custom Dashboards

### Project Dashboard

Create a note at `21.00 Active Projects.md` with this query:

```dataview
TABLE status, target_end_date as "Deadline"
FROM "20-29 - Work/21 Active Projects"
WHERE status = "active"
SORT target_end_date ASC
```

### Daily Work Dashboard

Create a note at `20.00 Work Dashboard.md`:

```dataview
LIST
FROM "20-29 - Work"
WHERE contains(file.name, "{{date:YYYY-MM-DD}}")
```

## Using an Index Note

Create an index note at the root of your vault:

```markdown
# Johnny Decimal Index

## Personal (10-19)
- [[11 Me]]
- [[12 House]]
- [[13 Money]]
- [[14 Online]]
- [[15 Travel]]

## Work (20-29)
- [[21 Active Projects]]
- [[22 Admin & HR]]
- [[23 Professional Development]]
- [[24 Contacts]]
- [[25 Meetings]]

## Digital Side Projects (30-39)
- [[31 Betting App]]
- [[32 Oscar Competition App]]
- [[33 Bachelor Party App]]
- [[34 Wholesome App]]
- [[35 Project Ideas & Future Dev]]
```

## Setting Up Saved Searches

Save your most common searches:

1. Perform a search
2. Click the "Save" icon next to the search bar
3. Name your search (e.g., "Active Work Projects")
4. Access from the search pane

## Keyboard Shortcuts

Set up custom keyboard shortcuts for your most used areas:

1. Open Settings > Hotkeys
2. Create shortcuts for commands like:
   - Open daily note
   - Open weekly review
   - Navigate to Work area

## Using Aliases for Common Items

For frequently accessed items, create aliases in your Obsidian frontmatter:

```yaml
---
aliases: [Daily Work Log, Today's Tasks]
---
```

Then you can search for these aliases instead of the full Johnny Decimal ID.

## Mobile Navigation Tips

When using Obsidian mobile:

1. Star your most important notes
2. Use the quick switcher with ID numbers
3. Set up saved searches for common queries
4. Create a "Mobile Dashboard" note with links to frequent areas
