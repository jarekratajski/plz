# Language 
rust

# General code style
Keep naming verbose
Avoid redundant comments - only comment when names do not suggest what is happenning (prefer naming to commenting)
Prefer immutability
Prefer functional code
Try to write tests for each feature
Avoid mocks in tests, mock external services however (tests should only depend on the state of the code - not on external service working)
If external service is to be used try to use docker containers (for instance for postgres database)
If you need to mock external https service - try to write mock on a http level (so that call is made to http)
Keep code compiling if possible during implementation steps
If in doubt stop and ask human developer (unless feature instructions say - 'yolo it')
Keep code compiling without any warnings

# Rust specific
Avoid any unsafe constructs
Use Result for errors