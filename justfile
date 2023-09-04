
dev:
  rm -f lsp-input
  mkfifo lsp-input
  cargo watch --ignore 'lsp-input' \
    -x test \
    -x 'run --bin smol-lsp -- <lsp-input'
