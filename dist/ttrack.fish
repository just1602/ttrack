complete -c ttrack -n "__fish_use_subcommand" -s f -d 'File where to store tracking data.' -r
complete -c ttrack -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_use_subcommand" -s V -l version -d 'Print version'
complete -c ttrack -n "__fish_use_subcommand" -f -a "track" -d 'Track a new time record.'
complete -c ttrack -n "__fish_use_subcommand" -f -a "report" -d 'Generate report from the records.'
complete -c ttrack -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c ttrack -n "__fish_seen_subcommand_from track" -s t -d 'The time duration of the record in seconds.' -r
complete -c ttrack -n "__fish_seen_subcommand_from track" -s d -d 'The description of the time record (what has been done).' -r
complete -c ttrack -n "__fish_seen_subcommand_from track" -s p -d 'The project with which this record is associated with.' -r
complete -c ttrack -n "__fish_seen_subcommand_from track" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_seen_subcommand_from report" -l since -d 'The date since when we want the report to start.' -r
complete -c ttrack -n "__fish_seen_subcommand_from report" -l until -d 'The date until when we want the report to end.' -r
complete -c ttrack -n "__fish_seen_subcommand_from report" -l by-project -d 'Report time by project.'
complete -c ttrack -n "__fish_seen_subcommand_from report" -l today -d 'Only report data from today'
complete -c ttrack -n "__fish_seen_subcommand_from report" -l yesterday -d 'Only report data from yesterday'
complete -c ttrack -n "__fish_seen_subcommand_from report" -l this-week -d 'Only report data from the current week (monday to sunday).'
complete -c ttrack -n "__fish_seen_subcommand_from report" -l last-week -d 'Only report data from the previous week (monday to sunday).'
complete -c ttrack -n "__fish_seen_subcommand_from report" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from track; and not __fish_seen_subcommand_from report; and not __fish_seen_subcommand_from help" -f -a "track" -d 'Track a new time record.'
complete -c ttrack -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from track; and not __fish_seen_subcommand_from report; and not __fish_seen_subcommand_from help" -f -a "report" -d 'Generate report from the records.'
complete -c ttrack -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from track; and not __fish_seen_subcommand_from report; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
