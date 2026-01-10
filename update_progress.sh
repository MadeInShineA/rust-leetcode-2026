#!/bin/bash

# Script to update progress tracking in README.md
# Automatically detects all month directories and generates progress bars

# Get current year, month, and day
current_date=$(date +"%Y-%m-%d")
current_year=$(date +"%Y")
current_month=$(date +"%m" | sed 's/^0//')
current_day=$(date +"%d" | sed 's/^0//')

# Month name to number mapping
case $(date +%B | tr '[:upper:]' '[:lower:]') in
  january) current_month_num=1 ;;
  february) current_month_num=2 ;;
  march) current_month_num=3 ;;
  april) current_month_num=4 ;;
  may) current_month_num=5 ;;
  june) current_month_num=6 ;;
  july) current_month_num=7 ;;
  august) current_month_num=8 ;;
  september) current_month_num=9 ;;
  october) current_month_num=10 ;;
  november) current_month_num=11 ;;
  december) current_month_num=12 ;;
esac

# Find all month directories in src/
months=$(find src -maxdepth 1 -type d -name "*" | sed 's|src/||' | grep -v "^src$" | sort)

# Initialize difficulty counters
total_easy=0
total_medium=0
total_hard=0
declare -A month_easy month_medium month_hard

# First pass: count difficulties from all files
for month in $months; do
  if [ -d "src/$month" ]; then
    month_cap=$(echo "$month" | sed 's/^\(.\)/\u\1/')
    month_easy["$month_cap"]=0
    month_medium["$month_cap"]=0
    month_hard["$month_cap"]=0
    
    for day_file in src/$month/day_*.rs; do
      if [ -f "$day_file" ]; then
        difficulty=$(head -3 "$day_file" | tail -1 | sed 's|^// ||')
        case $difficulty in
          Easy) 
            ((total_easy++))
            ((month_easy["$month_cap"]++))
            ;;
          Medium) 
            ((total_medium++))
            ((month_medium["$month_cap"]++))
            ;;
          Hard) 
            ((total_hard++))
            ((month_hard["$month_cap"]++))
            ;;
        esac
      fi
    done
  fi
done

# Create progress section for README
progress_section="## Progress

Updated: $current_date

### Monthly Difficulty Breakdown
| Month | Easy | Medium | Hard | Total |
|-------|------|--------|------|-------|
"

# Add monthly breakdown rows
for month in $months; do
  if [ -d "src/$month" ]; then
    month_cap=$(echo "$month" | sed 's/^\(.\)/\u\1/')
    month_total=$((month_easy["$month_cap"] + month_medium["$month_cap"] + month_hard["$month_cap"]))
    if [ $month_total -gt 0 ]; then
      progress_section+="| $month_cap | ${month_easy["$month_cap"]} | ${month_medium["$month_cap"]} | ${month_hard["$month_cap"]} | $month_total |
"
    fi
  fi
done

# Add total row at the end
grand_total=$((total_easy + total_medium + total_hard))
progress_section+="| **Total** | **$total_easy** | **$total_medium** | **$total_hard** | **$grand_total** |
"
progress_section+="
"

# Generate progress for each month
for month in $months; do
  if [ -d "src/$month" ]; then
    # Count completed days (remove quotes to allow glob expansion)
    completed=$(ls -1 src/$month/day_*.rs 2>/dev/null | wc -l)

    # Determine days in month
    case $month in
    january | jan) days_in_month=31 ;;
    february | feb)
      # Check for leap year
      if ((current_year % 4 == 0 && (current_year % 100 != 0 || current_year % 400 == 0))); then
        days_in_month=29
      else
        days_in_month=28
      fi
      ;;
    march | mar) days_in_month=31 ;;
    april | apr) days_in_month=30 ;;
    may) days_in_month=31 ;;
    june | jun) days_in_month=30 ;;
    july | jul) days_in_month=31 ;;
    august | aug) days_in_month=31 ;;
    september | sep | sept) days_in_month=30 ;;
    october | oct) days_in_month=31 ;;
    november | nov) days_in_month=30 ;;
    december | dec) days_in_month=31 ;;
    *) days_in_month=31 ;;
    esac

    # Calculate percentage
    if [ $days_in_month -gt 0 ]; then
      percentage=$((completed * 100 / days_in_month))
    else
      percentage=0
    fi

    # Generate progress bar
    filled=$((completed * 20 / days_in_month))
    empty=$((20 - filled))
    progress_bar=""
    for ((i = 0; i < filled; i++)); do progress_bar+="█"; done
    for ((i = 0; i < empty; i++)); do progress_bar+="░"; done

    # Capitalize month name and get month number
    month_cap=$(echo "$month" | sed 's/^\(.\)/\u\1/')
    
    # Get month number for comparison
    case $(echo "$month" | tr '[:upper:]' '[:lower:]') in
      january|jan) month_num=1 ;;
      february|feb) month_num=2 ;;
      march|mar) month_num=3 ;;
      april|apr) month_num=4 ;;
      may) month_num=5 ;;
      june|jun) month_num=6 ;;
      july|jul) month_num=7 ;;
      august|aug) month_num=8 ;;
      september|sep|sept) month_num=9 ;;
      october|oct) month_num=10 ;;
      november|nov) month_num=11 ;;
      december|dec) month_num=12 ;;
      *) month_num=0 ;;
    esac

    progress_section+="### $month_cap
 \`\`\`
Progress: $progress_bar $completed/$days_in_month days ($percentage%)
Remaining: $(($days_in_month - $completed)) problems
\`\`\`

| Day | Problem | Difficulty | Status |
|-----|---------|------------|--------|
"
    # Generate table for each day
    for day in $(seq 1 $days_in_month); do
      if [ -f "src/$month/day_$day.rs" ]; then
        # Extract problem URL (line 1)
        url=$(head -1 "src/$month/day_$day.rs" | sed 's|^// ||')

        # Extract problem number and name (line 2, format: // 66. Plus One)
        problem_line=$(head -2 "src/$month/day_$day.rs" | tail -1 | sed 's|^// ||')
        problem_num=$(echo "$problem_line" | grep -oE "^[0-9]+")
        problem_name=$(echo "$problem_line" | sed 's|^[0-9]*\. ||')

        # Extract difficulty (line 3)
        difficulty=$(head -3 "src/$month/day_$day.rs" | tail -1 | sed 's|^// ||')

        if [ -n "$url" ] && [ -n "$problem_name" ]; then
          progress_section+="| $day | [$problem_num. $problem_name]($url) | $difficulty | ✅ |
"
        else
          progress_section+="| $day | TBD | - | ✅ |
"
        fi
      else
        # Check if this day is overdue (current month, day passed, not done)
        if [ $month_num -eq $current_month_num ] && [ $day -lt $current_day ]; then
          progress_section+="| $day | TBD | - | ❌ |
"
        else
          progress_section+="| $day | TBD | - | ⬜ |
"
        fi
      fi
    done
    progress_section+="
"
  fi
done

# Replace progress section in README.md
# Find the line that starts with "## Progress" and replace everything until "## Running Tests"
awk '
    BEGIN { in_progress = 0; progress_found = 0 }
    /^## Progress$/ { 
        if (!progress_found) {
            in_progress = 1
            progress_found = 1
            next
        }
    }
    in_progress && /^## Running Tests$/ { 
        in_progress = 0
    }
    !in_progress { print }
' README.md > README.md.before.md

# Create the new README by combining parts
head -5 README.md.before.md > README.md
echo "$progress_section" >> README.md
tail -n +6 README.md.before.md | grep -A 999 "## Running Tests" >> README.md

rm README.md.before.md

echo "Progress updated!"
