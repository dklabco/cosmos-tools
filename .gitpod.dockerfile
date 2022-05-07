FROM gitpod/workspace-full

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh \
 && curl -fsSL https://deno.land/x/install/install.sh | sh \
 && export DENO_INSTALL="$HOME/.deno" \
 && export PATH="$DENO_INSTALL/bin:$PATH"