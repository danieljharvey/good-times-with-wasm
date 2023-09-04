
# run tests etc then build binary so we can try it in neovim etc
dev:
  cargo watch --ignore 'lsp-input' \
    -x test \
    -x 'build --bin smol-lsp'
