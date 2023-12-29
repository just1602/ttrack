# ttrack

ttrack is a simple CLI that allow you to track your time and that use CSV file to store your data.

## Basic usage

You can use `ttrack help` for help and `ttrack help <command>` for help about a specific subcommand.

To track a new time period:

```
ttrack -f your-data-file.csv track -t <time-in-seconds> -d <description> -p [project-name]
```

To get information about the tracked time period, you can use the `report` command. For now, the default behavior is just printing the total amount of hours, but you can use `--by-project` to get a more detailed report.

## Data file

You can name your data file how you want, it doesn't need the `.csv` extension. Also, if you always use the same file, you're encourage to alias to add an alias in your shell config to something like that:

```
alias ttrack='ttrack -f ~/my-data.csv'
```
