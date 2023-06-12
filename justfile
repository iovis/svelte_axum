# set dotenv-load  # Uncomment to load .env

default: list

alias l := list
alias s := server

# lists available tasks
list:
    @just --list

# Start the development server
@server:
    cd frontend && pnpm run dev --host
