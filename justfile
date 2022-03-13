serve:
    trunk serve --open

# Checks with all features enabled
check:
    cargo check --features alert,button,button_group

# Runs the alert example
example-alerts:
    cd examples/alerts && trunk serve --open

# Runs the button example
example-buttons:
    cd examples/buttons && trunk serve --open

# Runs the button-group example
example-button-group:
    cd examples/button_groups && trunk serve --open