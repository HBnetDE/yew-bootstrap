serve:
    trunk serve --open

# Checks with all features enabled
check:
    cargo check --features alert,button

# Runs the alert example
example-alerts:
    cd examples/alerts && trunk serve --open