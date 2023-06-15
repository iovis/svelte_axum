# set dotenv-load  # Uncomment to load .env

default: all

alias l := list
alias f := frontend
alias b := backend

all:
    tmux new-window -n vite 'just frontend'
    tmux new-window -n cargo 'just backend'

# lists available tasks
list:
    @just --list

# Start the frontend development server
@frontend:
    cd frontend && pnpm run dev --host

# Start the backend development server
@backend:
    cd backend && RUST_LOG=debug cargo watch -x run
