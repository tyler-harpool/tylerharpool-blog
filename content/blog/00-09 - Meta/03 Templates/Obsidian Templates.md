# Obsidian Templates

## Daily Check-in Template

```markdown
---
type: daily-checkin
date: {{date:YYYY-MM-DD}}
tags: [daily, checkin]
---

# Daily Check-in: {{date:dddd, MMMM D, YYYY}}

## Morning Planning
- [ ] Review calendar for the day
- [ ] Check priority tasks from yesterday
- [ ] Set top 3 goals for today

### Today's Focus Areas
1.
2.
3.

### Meetings Today
-

## Active Projects
- **Work**: [[21.01]] - Status:
- **Side Project**: [[31.02]] - Status:
- **Personal**: [[13.04]] - Status:

## Tasks
### Carry-over from yesterday
- [ ]

### New tasks
- [ ]

## Evening Review
### Completed Today
-

### Progress on Key Projects
-

### Carry Forward
- [ ]

### Reflections
-

### Tomorrow's Priorities
1.
2.
3.
```

## Weekly Review Template

```markdown
---
type: weekly-review
week: {{date:YYYY-[W]ww}}
date: {{date:YYYY-MM-DD}}
tags: [weekly, review]
---

# Weekly Review: {{date:YYYY-[W]ww}}

## Areas Review

### 10-19 - Personal
- Key activities:
- Items requiring attention:

### 20-29 - Work
- Key activities:
- Items requiring attention:

### 30-39 - Digital Side Projects
- Key activities:
- Items requiring attention:

### Other Areas
- Key activities:
- Items requiring attention:

## Project Updates
- [[21.01]]:
- [[31.02]]:
- [[13.04]]:

## Achievements
-

## Challenges
-

## Next Week Priorities
1.
2.
3.

## New Items to File
- [ ]

## Notes and Ideas
-
```

## Meeting Notes Template

```markdown
---
type: meeting
date: {{date:YYYY-MM-DD}}
time:
attendees:
project:
tags: [meeting]
related: [[25.01]]
---

# Meeting: {{title}}

## Agenda
1.
2.
3.

## Discussion Points
-

## Action Items
- [ ]
- [ ]
- [ ]

## Decisions Made
-

## Follow-up Required
- [ ] Schedule next meeting
- [ ] Distribute notes to team
- [ ] Update project plan [[21.01]]
```

## Project Template

```markdown
---
type: project
id: {{project_id}}
status: active
start_date: {{date:YYYY-MM-DD}}
target_end_date:
tags: [project]
---

# Project: {{title}}

## Overview
- **Goal**:
- **Description**:
- **Success Metrics**:

## Key Stakeholders
-

## Timeline
- Start: {{date:YYYY-MM-DD}}
- Milestone 1:
- Milestone 2:
- Target Completion:

## Tasks
- [ ]
- [ ]
- [ ]

## Resources
-

## Related Items
- Meetings: [[25.01]]
- Documentation:
- References:
```

## New Item Template

```markdown
---
type: item
id: {{decimal_id}}
created: {{date:YYYY-MM-DD}}
tags: []
---

# {{title}}

## Overview
-

## Details
-

## Related Items
-
```

## How to Use These Templates in Obsidian

1. Create a Templates folder at `00-09 - Meta/03 Templates`
2. Create separate files for each template
3. Install the Obsidian Templates plugin if not already installed
4. Configure the Templates plugin to use this folder path
5. Use the template hotkey (default: Ctrl/Cmd+T) to insert a template in a new note

You can customize these templates to better suit your specific workflow needs.
