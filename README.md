# deskstash

Stash your desktop items.

```sh
ls ~/Desktop
# something-1	something-2	something-3

deskstash

ls ~/Desktop
# Nothing! They have moved ~/.deskstash/YYYY-MM-DD-....

tree ~/.deskstash
# /Users/astk/.deskstash
# └── 2022-01-28-022310
#     ├── something-1
#     ├── something-2
#     └── something-3
#
# 4 directories, 4 files
```
