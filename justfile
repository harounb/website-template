build:
  cd builder && cargo run
serve:
  static-web-server -p 3001 -d dist
