# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_ttrack_global_optspecs
	string join \n f/file= h/help V/version
end

function __fish_ttrack_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_ttrack_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_ttrack_using_subcommand
	set -l cmd (__fish_ttrack_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c ttrack -n "__fish_ttrack_needs_command" -s f -l file -d 'File where to store tracking data' -r -F
complete -c ttrack -n "__fish_ttrack_needs_command" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_ttrack_needs_command" -s V -l version -d 'Print version'
complete -c ttrack -n "__fish_ttrack_needs_command" -f -a "track" -d 'Track a new time record'
complete -c ttrack -n "__fish_ttrack_needs_command" -f -a "t" -d 'Track a new time record'
complete -c ttrack -n "__fish_ttrack_needs_command" -f -a "report" -d 'Generate report from the records'
complete -c ttrack -n "__fish_ttrack_needs_command" -f -a "r" -d 'Generate report from the records'
complete -c ttrack -n "__fish_ttrack_needs_command" -f -a "start" -d 'Start to track a task'
complete -c ttrack -n "__fish_ttrack_needs_command" -f -a "s" -d 'Start to track a task'
complete -c ttrack -n "__fish_ttrack_needs_command" -f -a "pause"
complete -c ttrack -n "__fish_ttrack_needs_command" -f -a "resume"
complete -c ttrack -n "__fish_ttrack_needs_command" -f -a "stop"
complete -c ttrack -n "__fish_ttrack_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c ttrack -n "__fish_ttrack_using_subcommand track" -l date -d 'The date the record is created' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand track" -s t -l time -d 'The time duration of the record' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand track" -s d -l description -d 'The description of the time record (what has been done)' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand track" -s p -l project -d 'The project with which this record is associated with' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand track" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_ttrack_using_subcommand t" -l date -d 'The date the record is created' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand t" -s t -l time -d 'The time duration of the record' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand t" -s d -l description -d 'The description of the time record (what has been done)' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand t" -s p -l project -d 'The project with which this record is associated with' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand t" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_ttrack_using_subcommand report" -l since -d 'The date since when we want the report to start' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand report" -l until -d 'The date until when we want the report to end' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand report" -l by-project -d 'Report time by project'
complete -c ttrack -n "__fish_ttrack_using_subcommand report" -l today -d 'Only report data from today'
complete -c ttrack -n "__fish_ttrack_using_subcommand report" -l yesterday -d 'Only report data from yesterday'
complete -c ttrack -n "__fish_ttrack_using_subcommand report" -l this-week -d 'Only report data from the current week (monday to sunday)'
complete -c ttrack -n "__fish_ttrack_using_subcommand report" -l last-week -d 'Only report data from the current week (monday to sunday)'
complete -c ttrack -n "__fish_ttrack_using_subcommand report" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_ttrack_using_subcommand r" -l since -d 'The date since when we want the report to start' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand r" -l until -d 'The date until when we want the report to end' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand r" -l by-project -d 'Report time by project'
complete -c ttrack -n "__fish_ttrack_using_subcommand r" -l today -d 'Only report data from today'
complete -c ttrack -n "__fish_ttrack_using_subcommand r" -l yesterday -d 'Only report data from yesterday'
complete -c ttrack -n "__fish_ttrack_using_subcommand r" -l this-week -d 'Only report data from the current week (monday to sunday)'
complete -c ttrack -n "__fish_ttrack_using_subcommand r" -l last-week -d 'Only report data from the current week (monday to sunday)'
complete -c ttrack -n "__fish_ttrack_using_subcommand r" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_ttrack_using_subcommand start" -s d -l description -d 'The description of the task we start tracking' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand start" -s p -l project -d 'The project with which this task will be associated' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand start" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_ttrack_using_subcommand s" -s d -l description -d 'The description of the task we start tracking' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand s" -s p -l project -d 'The project with which this task will be associated' -r
complete -c ttrack -n "__fish_ttrack_using_subcommand s" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_ttrack_using_subcommand pause" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_ttrack_using_subcommand resume" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_ttrack_using_subcommand stop" -s h -l help -d 'Print help'
complete -c ttrack -n "__fish_ttrack_using_subcommand help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "track" -d 'Track a new time record'
complete -c ttrack -n "__fish_ttrack_using_subcommand help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "report" -d 'Generate report from the records'
complete -c ttrack -n "__fish_ttrack_using_subcommand help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "start" -d 'Start to track a task'
complete -c ttrack -n "__fish_ttrack_using_subcommand help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "pause"
complete -c ttrack -n "__fish_ttrack_using_subcommand help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "resume"
complete -c ttrack -n "__fish_ttrack_using_subcommand help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "stop"
complete -c ttrack -n "__fish_ttrack_using_subcommand help; and not __fish_seen_subcommand_from track report start pause resume stop help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
