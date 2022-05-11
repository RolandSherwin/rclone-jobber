### YAML Structure
```
# Name of the job, used in logging to logs/app
New-Folder-2:
    # Source/dest/log_path can be linux/windows/remote
    # if linux/windows, the path must exist (log file need not exist, will be created)
    # remote has no checks.
    source:
        linux: /mnt/local-disk-d/New-Folder-2/
        windows: E:\New-Folder-2
    dest:
        remote: daily-backup-crypt:/new_folder_2/
    # compulsory field, has to be separated by space
    options: --fast-list -v --progress --transfers=30 --checkers=40 --copy-links
    #optional one
    log_path:
        windows: C:\Users\roland\Downloads\gg.txt
        linux: /mnt/local-disk-d/New-Folder-2/Projects/rclone-batcher/logs/New-Folder-2.log
    #optional one; uses "--filter=" internally; need be inside quotes.
    # to
    filters:
        - "- *.log"
        - "- *.logs"

    # this pattern is used to exclude everything from a folder then include some folders/files only
    filters:
        - "+ /espanso/**"
        # exclude all should be the last filter
        - "- **"
```
