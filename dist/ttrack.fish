complete -c ttrack -n "__fish_use_subcommand" -s f -l file -d 'File where to store tracking data' -r -F
complete -c ttrack -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_use_subcommand" -s V -l version -d 'Print version'
complete -c ttrack -n "__fish_use_subcommand" -f -a "track" -d 'Track a new time record'
complete -c ttrack -n "__fish_use_subcommand" -f -a "t" -d 'Track a new time record'
complete -c ttrack -n "__fish_use_subcommand" -f -a "report" -d 'Generate report from the records'
complete -c ttrack -n "__fish_use_subcommand" -f -a "r" -d 'Generate report from the records'
complete -c ttrack -n "__fish_use_subcommand" -f -a "start" -d 'Start to track a task'
complete -c ttrack -n "__fish_use_subcommand" -f -a "s" -d 'Start to track a task'
complete -c ttrack -n "__fish_use_subcommand" -f -a "pause"
complete -c ttrack -n "__fish_use_subcommand" -f -a "resume"
complete -c ttrack -n "__fish_use_subcommand" -f -a "stop"
complete -c ttrack -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c ttrack -n "__fish_seen_subcommand_from track" -s t -l time -d 'The time duration of the record' -r
complete -c ttrack -n "__fish_seen_subcommand_from track" -s d -l description -d 'The description of the time record (what has been done)' -r
complete -c ttrack -n "__fish_seen_subcommand_from track" -s p -l project -d 'The project with which this record is associated with' -r
complete -c ttrack -n "__fish_seen_subcommand_from track" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_seen_subcommand_from t" -s t -l time -d 'The time duration of the record' -r
complete -c ttrack -n "__fish_seen_subcommand_from t" -s d -l description -d 'The description of the time record (what has been done)' -r
complete -c ttrack -n "__fish_seen_subcommand_from t" -s p -l project -d 'The project with which this record is associated with' -r
complete -c ttrack -n "__fish_seen_subcommand_from t" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_seen_subcommand_from report" -l since -d 'The date since when we want the report to start' -r
complete -c ttrack -n "__fish_seen_subcommand_from report" -l until -d 'The date until when we want the report to end' -r
complete -c ttrack -n "__fish_seen_subcommand_from report" -l by-project -d 'Report time by project'
complete -c ttrack -n "__fish_seen_subcommand_from report" -l today -d 'Only report data from today'
complete -c ttrack -n "__fish_seen_subcommand_from report" -l yesterday -d 'Only report data from yesterday'
complete -c ttrack -n "__fish_seen_subcommand_from report" -l this-week -d 'Only report data from the current week (monday to sunday)'
complete -c ttrack -n "__fish_seen_subcommand_from report" -l last-week -d 'Only report data from the current week (monday to sunday)'
complete -c ttrack -n "__fish_seen_subcommand_from report" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_seen_subcommand_from r" -l since -d 'The date since when we want the report to start' -r
complete -c ttrack -n "__fish_seen_subcommand_from r" -l until -d 'The date until when we want the report to end' -r
complete -c ttrack -n "__fish_seen_subcommand_from r" -l by-project -d 'Report time by project'
complete -c ttrack -n "__fish_seen_subcommand_from r" -l today -d 'Only report data from today'
complete -c ttrack -n "__fish_seen_subcommand_from r" -l yesterday -d 'Only report data from yesterday'
complete -c ttrack -n "__fish_seen_subcommand_from r" -l this-week -d 'Only report data from the current week (monday to sunday)'
complete -c ttrack -n "__fish_seen_subcommand_from r" -l last-week -d 'Only report data from the current week (monday to sunday)'
complete -c ttrack -n "__fish_seen_subcommand_from r" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_seen_subcommand_from start" -s d -l description -d 'The description of the task we start tracking' -r
complete -c ttrack -n "__fish_seen_subcommand_from start" -s p -l project -d 'The project with which this task will be associated' -r
complete -c ttrack -n "__fish_seen_subcommand_from start" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_seen_subcommand_from s" -s d -l description -d 'The description of the task we start tracking' -r
complete -c ttrack -n "__fish_seen_subcommand_from s" -s p -l project -d 'The project with which this task will be associated' -r
complete -c ttrack -n "__fish_seen_subcommand_from s" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_seen_subcommand_from pause" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_seen_subcommand_from resume" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_seen_subcommand_from stop" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "track" -d 'Track a new time record'
complete -c ttrack -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "report" -d 'Generate report from the records'
complete -c ttrack -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "start" -d 'Start to track a task'
complete -c ttrack -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "pause"
complete -c ttrack -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "resume"
complete -c ttrack -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "stop"
complete -c ttrack -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
