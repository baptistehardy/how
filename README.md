# how

Still in its infancy. 

To-do:

- Better and fancier user input/error handling
- Config file for defaults (models, answer formatting) and environment variables
- Downloadable production release

Inspired by [Alex's how](https://github.com/alex-kinokon/how).

## Installation

Make sure [Rust and its toolchain](https://www.rust-lang.org/tools/install) are installed.

```
$ cargo install --git https://github.com/baptistehardy/how --branch main
```

A working OpenAI API key is required for this program to work. 

Set `OPENAI_API_KEY` as an environment variable with:
```
# On Linux/MacOS
export OPENAI_API_KEY='sk-...'

# On Windows Powershell
$Env:OPENAI_API_KEY='sk-...'
# On Windows Command Prompt
set OPENAI_API_KEY=sk-...
```