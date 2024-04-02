start_date=2024-03-01
end_date=$(date +%Y-%m-%d)

d="$start_date"
echo "" > docs/daily_report.txt

while [ "$d" != "$end_date" ]; do
    git_commits=$(git log --pretty=format:'%ad %h %s' --after="$d 00:00" --before="$d 23:59" --date=short --reverse)
    if [ -z "$git_commits" ]; then
        echo -e "Date: $d \n  No commits were made this day." >> docs/daily_report.txt
    else
        echo -e "Date: $d" >> docs/daily_report.txt
        echo "$git_commits" | awk '
            {
                print "  " substr($0, index($0,$2))
            }' >> docs/daily_report.txt
    fi
    d=$(date -I -d "$d + 1 day")
done