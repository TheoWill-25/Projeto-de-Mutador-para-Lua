
# Mutação de Código Lua em Rust

Projeto inicial de uma ferramenta em Rust para localizar trechos de código Lua delimitados por marcadores e identificar candidatos a mutação, como operadores relacionais. A proposta é evoluir o projeto para gerar versões modificadas do código e executar testes para verificar se as alterações são detectadas.

## 1. Arquivos do projeto

### `main.rs`

É o ponto de entrada do programa. Atualmente, ele:

1. Lê o arquivo `codigo.lua`.
2. Localiza a região delimitada pelos marcadores de mutação.
3. Executa o scanner para encontrar candidatos.
4. Imprime os candidatos encontrados.

### `scanner.rs`

Contém as estruturas e funções responsáveis pela análise do código.

- **`Candidate`**: estrutura que representa um candidato, contendo a linha, as posições inicial e final, o texto original e a categoria.
- **`SYMBOLS_LIST`**: lista dos caracteres que o scanner procura atualmente: `<`, `>`, `=` e `~`.
- **`start_end_finder(code)`**: procura os marcadores de início e fim da região de mutação.
- **`symbol_scanner(code, start_indx, end_indx)`**: percorre a região identificada e retorna os candidatos encontrados em um vetor.

O scanner ainda está em desenvolvimento e não realiza uma análise completa da sintaxe de Lua.

### `codigo.lua`

É o arquivo Lua utilizado como entrada para a análise. No exemplo atual, contém a função `ScholarshipEligibilityEvaluator.evaluate_scholarship`, responsável por avaliar a elegibilidade de uma pessoa para uma bolsa.

A região a ser analisada é delimitada pelos comentários:

```lua
--start mut

-- Trecho de código que será analisado

--end mut
```

Os marcadores devem permanecer no arquivo para que o programa possa localizar a região de interesse.

### `test.lua`

Este arquivo é reservado **exclusivamente para testes**. Ele não é a entrada principal do scanner: atualmente, o `main.rs` lê `codigo.lua`.

O objetivo de `test.lua` é concentrar os casos de teste usados para verificar o comportamento do código Lua. No contexto futuro da ferramenta de mutação, ele poderá ser executado contra o código original e contra cada versão modificada.

Exemplos de casos que podem ser testados:

- Valores que devem resultar em aprovação ou rejeição.
- Valores próximos aos limites das condições, como `gpa = 6` ou `attendance_rate = 75`.
- Situações em que uma alteração de operador muda o resultado da função.
- Casos que devem continuar produzindo o mesmo resultado mesmo após uma mutação.

**Importante:** o código Rust atual ainda não executa `test.lua` nem gera mutantes automaticamente. Essa integração será implementada em etapas posteriores.

## 2. Como executar

O projeto utiliza Rust. Para organizar os arquivos com Cargo, a estrutura pode ser:

```text
projeto/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── scanner.rs
├── codigo.lua
├── test.lua
└── README.md
```

Com os arquivos organizados dessa maneira, execute na pasta do projeto:

```bash
cargo run
```

O programa espera encontrar `codigo.lua` no diretório de trabalho atual.

## 3. Funcionamento atual

O fluxo de execução é:

1. Ler o código Lua.
2. Identificar os limites da região marcada para mutação.
3. Examinar os símbolos presentes nessa região.
4. Registrar os candidatos encontrados.
5. Imprimir os resultados no terminal.

## 4. Próximas etapas

- [ ] Melhorar o tratamento de marcadores ausentes ou inválidos.
- [ ] Evitar acessos a posições inexistentes durante a análise.
- [ ] Reconhecer operadores compostos, como `<=`, `>=`, `==` e `~=`.
- [ ] Evitar falsos positivos em comentários e identificadores.
- [ ] Definir posições consistentes para localizar candidatos no arquivo original.
- [ ] Gerar mutantes sem modificar permanentemente o código original.
- [ ] Executar `test.lua` contra o código original e os mutantes.
- [ ] Registrar mutantes mortos, sobreviventes e inválidos.
