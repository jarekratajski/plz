# Goal

Before executing bash command tool should categorize potential impact into 3 categories:

- safe:
    - the command reads files, creates new ones, does not change much existing
    - the command changes files but those in project that is part of a .git project (we can assume the files are stored)
    - the command creates, stops, removes docker components - as long as we can assume this is developer machine

- moderate:
    - the command modifies few files that are not part of a .git project, files are not important system files or user config files (like in .config folder in linux)
    - the command installs well known popular tools (like npm packages)
    - the command deletes some files - but it is surely the intention, and in worst case it is easy to recreate them,
    - command calls any external apis, 

- dangerous:
    - command modifies system or .config files
    - command install some non-stardard tools,
    - there are bigger changes expected (edits) in many files (over 1kb text difference)
    - there are more files to be deleted
    - there is a need to call sudo


plz command should have a switch (option):
 -f  (force)
 -s  (safe)

Without any option (example):
 `plz list three oldest files in this folder`

 commands categorized as safe should be just executed - print what is to execute and then do it

 command categorized as moderate should still need confirmation (y/n),

 command categorized as dangerous should be rejected (print them and do not execute)

 With a -s switch:
 `plz -s delete recent file`

command categorized as safe are just executed (always print what is to be executed)

commands categorized as moderate are rejected (print them and do not execute)

command categorized as dangerous should be rejected (print them and do not execute)

With a -f switch:

commands categorized as safe are just executed (always print what is to be executed)

commands categorized as moderate are just executed (always print what is to be executed)

commands categorized as dangerous need confirmation (y/n) and if confirmed are executed



