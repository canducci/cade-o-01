# Cadê o 01?

Jogo satírico de perseguição geográfica, em pt-BR, rodando no browser via WebAssembly (Rust puro, sem dependências).

**Jogar:** https://canducci.github.io/cade-o-01/

O branch `gh-pages` é gerado pelo CI (`.github/workflows/pages.yml`) a cada push no `main`.

> Paródia. Personagem inspirado em figura pública real. As acusações seguem reportagens e aparecem com o status de cada caso (seção *Dossiês* no jogo). Não há condenação nos casos citados e vale a presunção de inocência.

## Rodar

```sh
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/cade_o_01.wasm game.wasm
python3 -m http.server   # abra http://localhost:8000 (file:// não carrega .wasm)
```

Testes: `cargo test` (lógica) e `node check.mjs` (ponte wasm/UTF-8).

## Fontes (consultadas em 28/10/2026)

- Agência Brasil, "Entenda: investigações contra Flávio continuam no STF após fuga" (27/10/2026): https://agenciabrasil.ebc.com.br/justica/noticia/2026-10/entenda-investigacoes-contra-flavio-continuam-no-stf-apos-fuga
- Brasil de Fato (03/10/2026): https://www.brasildefato.com.br/2026/10/03/flavio-bolsonaro-chega-ao-primeiro-turno-investigado-pela-pf-com-relacoes-com-vorcaro-expostas-e-vice-suspeito-de-estupro/
- Nexo Jornal (27/10/2026): https://www.nexojornal.com.br/expresso/2026/10/27/flavio-bolsonaro-fugiu-argentina-pf-passaporte
- O Povo (2020), franquia de chocolates: https://www.opovo.com.br/noticias/politica/2020/08/20/investigacao-apura-se-loja-de-flavio-bolsonaro--lavou--r--2-1-milhoes.html
- Folha de Pernambuco, entrega da loja: https://www.folhape.com.br/politica/flavio-bolsonaro-entrega-loja-de-chocolates-investigada-no-caso-da/171501/
- InfoMoney, imóveis: https://www.infomoney.com.br/?p=1377445
- Poder360, mansão: https://www.poder360.com.br/congresso/flavio-bolsonaro-nega-irregularidades-em-compra-de-mansao-por-r-6-milhoes/
- Poder360 (25/03/2024), pista de Budapeste (Jair Bolsonaro dormiu duas noites na embaixada da Hungria): https://www.poder360.com.br/justica/bolsonaro-dormiu-duas-noites-na-embaixada-da-hungria-diz-nytimes/
