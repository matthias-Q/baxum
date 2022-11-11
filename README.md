# What is this?

This is just a test to check the simple performance of axum.
The backend exposes a REST endpoint for POST on `localhost:300/user`.
The response is equal to the input.

The client creates a `NUM_USERS` or number of CPU cores detected, random usernames in a
vector and sends them to the backend.

