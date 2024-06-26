name: Video Streamer

on:
  push:
    branches: [ "master", "develop" ]
  pull_request:
    branches: [ "master", "develop" ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4
    - name: Install GStreamer dependencies.
      run: sudo apt-get install -y libgstreamer1.0-dev
      working-directory: ./backend/video_streamer

    - name: Build.
      run: cargo build --verbose
      working-directory: ./backend/video_streamer

    - name: Run tests.
      run: cargo test --verbose
      working-directory: ./backend/video_streamer
