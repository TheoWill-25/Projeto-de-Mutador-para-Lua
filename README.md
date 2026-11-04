# Mutador de Código Lua em Rust

Projeto desenvolvido em **Rust** para realizar testes de mutação em programas escritos em **Lua**. A ferramenta identifica operadores em uma região delimitada do código-fonte, gera versões mutantes substituindo esses operadores e executa uma suíte de testes para verificar se as alterações são detectadas.

O objetivo é avaliar a capacidade de uma suíte de testes de identificar pequenas alterações no comportamento de um programa e, assim, ajudar a revelar possíveis lacunas nos casos de teste.

## 1. Arquivos e módulos do projeto

### `main.rs`

É o ponto de entrada do programa. Na organização descrita para o projeto, ele coordena as etapas iniciais:

1. Lê o arquivo `codigo.lua`.
2. Localiza a região delimitada pelos marcadores de mutação.
3. Executa o scanner para encontrar candidatos.
4. Encaminha os candidatos para as etapas seguintes e apresenta os resultados.

### `scanner.rs`

Contém as estruturas e funções responsáveis por localizar a região de interesse e identificar os operadores candidatos.

- **`Candidate`**: estrutura que representa um candidato à mutação, contendo linha, posições inicial e final, texto original e categoria.
- **`SYMBOLS_LIST`**: lista de símbolos que o scanner procura.
- **`start_end_finder(code)`**: procura os marcadores que delimitam a região de mutação.
- **`symbol_scanner(code, start_indx, end_indx)`**: percorre a região identificada e retorna os candidatos encontrados.

*Nota:* O scanner atual trabalha com a região delimitada via varredura de caracteres. A sua migração para utilização direta do **método de AST** é a **prioridade atual do projeto**.

### `ast.rs` (Método AST)

Módulo responsável pelo método de parsing e geração da **AST (Árvore de Sintaxe Abstrata / Abstract Syntax Tree)** a partir do código Lua. Permite mapear e estruturar o código em nós sintáticos, fornecendo uma representação precisa do programa para análises e manipulações sintáticas.

### `mutador.rs`

É responsável pela geração e execução das mutações. A partir dos candidatos identificados pelo scanner, consulta as substituições possíveis para cada operador, produz versões modificadas do código e executa a suíte de testes usando o interpretador Lua.

As informações coletadas durante cada execução são organizadas em estruturas de mutação para posterior geração do relatório.

### `codigo.lua`

É o arquivo Lua submetido à análise e à mutação. No exemplo utilizado durante o desenvolvimento, contém a função `ScholarshipEligibilityEvaluator.evaluate_scholarship`, que avalia a elegibilidade de uma pessoa para uma bolsa.

### `suit.lua`

Contém a suíte de testes executada contra o código original e suas versões mutadas. A saída dos testes é utilizada pelo mutador para identificar os resultados, por meio das marcações `[PASS]` e `[FAIL]`.

## 2. Delimitação da região de mutação

Para evitar alterações em partes indevidas do programa, o mutador utiliza comentários especiais como marcadores de início e fim da região elegível:

```lua
--start_mut();

-- Trecho de código que poderá receber mutações
if gpa < 6 then
    rejected = true
elseif gpa < 7 then
    manual_review = true
end

--end_mut();
```

O scanner procura esses marcadores e considera como região de interesse o trecho delimitado por eles. Assim, a intenção é restringir a identificação de candidatos e a geração de mutantes à parte do código selecionada pelo desenvolvedor.

Essa delimitação permite preservar trechos que não fazem parte do experimento, como declarações, validações iniciais e lógica posterior à região de mutação.

**Importante:** os marcadores devem ser escritos exatamente como `--start_mut();` e `--end_mut();` no arquivo Lua. Ambos precisam estar presentes para que a região seja localizada corretamente.

## 3. Como funciona

O fluxo geral da ferramenta é composto pelas seguintes etapas:

1. **Leitura do código:** o programa obtém o conteúdo do arquivo Lua de entrada.
2. **Delimitação:** localiza os marcadores `--start_mut();` e `--end_mut();`.
3. **Análise Sintática via AST:** aplicação do método de geração de AST para estruturação e mapeamento sintático do código.
4. **Identificação dos operadores:** o scanner percorre a região delimitada e registra os candidatos encontrados.
5. **Geração dos mutantes:** para cada candidato, o mutador consulta as regras de substituição e cria versões modificadas do código.
6. **Execução dos testes:** cada versão mutante é escrita no arquivo Lua utilizado pela suíte, e os testes são executados pelo processo externo `lua`.
7. **Coleta dos resultados:** o programa captura a saída da suíte e identifica as linhas marcadas com `[PASS]` e `[FAIL]`.
8. **Relatório:** as informações das mutações são armazenadas e serializadas em JSON para análise posterior.

## 4. Funcionalidades implementadas

Com base nos arquivos e na implementação apresentados durante o desenvolvimento, o projeto contempla:

- Leitura do código Lua de entrada.
- Delimitação da região de mutação por meio dos comentários `--start_mut();` e `--end_mut();`.
- Método de geração e manipulação de **AST (Árvore de Sintaxe Abstrata)** para análise estruturada do código Lua.
- Varredura da região delimitada para identificar operadores candidatos.
- Registro de informações dos candidatos, incluindo linha, posição inicial, posição final, operador original e tipo.
- Geração de substituições para operadores relacionais e de igualdade, conforme as regras definidas no mutador.
- Criação de mutantes independentes a partir do código original.
- Escrita temporária do código mutado no arquivo Lua utilizado pela suíte.
- Execução da suíte de testes por meio do interpretador Lua.
- Captura e interpretação das linhas de saída identificadas com `[PASS]` e `[FAIL]`.
- Armazenamento de informações sobre cada mutação, incluindo operador substituto, linha original, linha modificada, quantidade de testes contabilizados e testes que falharam.
- Serialização dos dados das mutações para geração de relatórios em JSON.

### 4.1. Operadores e substituições

As substituições são definidas pelas regras implementadas na função responsável por mapear o operador original para seus possíveis mutantes. Entre as regras documentadas no código estão:

| Operador original | Substituições previstas |
|---|---|
| `<` | `>`, `<=`, `==` |
| `>` | `<`, `>=`, `==` |
| `=` | `~=` |
| `~` | remoção do operador |
| `<=` | `<`, `>`, `==` |
| `>=` | `<`, `>`, `==` |
| `==` | `<`, `>`, `<=`, `>=`, `~=` |

As substituições efetivamente geradas dependem do operador identificado e das regras presentes na função de geração de símbolos.

## 5. Resultados e classificação dos mutantes

A análise dos resultados utiliza os conceitos de teste de mutação:

- **Killed (morto):** pelo menos um teste falha ao executar contra o mutante. Isso indica que a suíte detectou a alteração.
- **Survived (sobrevivente):** todos os testes esperados foram executados e passaram contra o mutante. A suíte não detectou a alteração.
- **Inconclusive (inconclusivo):** a execução não foi concluída corretamente ou não há evidências suficientes de que os testes foram executados. Esse caso não deve ser confundido com um mutante sobrevivente.

A classificação correta exige verificar não apenas a lista de testes que falharam, mas também se a execução terminou adequadamente e se a quantidade esperada de testes foi executada.

### 5.1. Informações registradas por mutação

Cada mutação armazena informações para identificar e analisar a alteração realizada:

| Campo | Descrição |
|---|---|
| `candidate` | Informações do operador original identificado pelo scanner. |
| `replacement` | Operador utilizado na mutação. |
| `og_line` | Linha original do código Lua. |
| `mod_line` | Linha após a substituição do operador. |
| `num_tests` | Quantidade de testes contabilizados na execução. |
| `failed_tests` | Nomes dos testes que falharam com o mutante. |

Os dados são serializados em JSON para facilitar a consulta e a análise posterior.

## 6. Tecnologias utilizadas

- **Rust:** implementação do scanner, geração de mutantes, execução dos processos e organização dos resultados.
- **Lua:** linguagem dos programas submetidos à mutação e ambiente de execução da suíte de testes.
- **AST (Abstract Syntax Tree):** método de parsing para representação sintática estruturada do código Lua.
- **Serde / Serde JSON:** serialização das estruturas de dados para relatórios JSON.

## 7. Requisitos

Para executar o projeto, é necessário ter instalado:

- [Rust](https://www.rust-lang.org/tools/install), incluindo Cargo.
- [Lua](https://www.lua.org/download.html), com o executável `lua` disponível no `PATH`.

As dependências Rust utilizadas pelo projeto são gerenciadas pelo Cargo e declaradas no arquivo `Cargo.toml`.

## 8. Como executar

Clone o repositório:

```bash
git clone https://github.com/TheoWill-25/Projeto-de-Mutador-para-Lua.git
cd Projeto-de-Mutador-para-Lua
```

Compile e execute o projeto:

```bash
cargo run
```

A execução depende dos arquivos Lua utilizados pelo projeto, incluindo o código submetido à mutação e a suíte de testes. Verifique se os nomes e caminhos desses arquivos correspondem aos utilizados na configuração atual do programa.

O arquivo `codigo.lua` deve conter os marcadores de início e fim da região de mutação. A suíte `suit.lua` deve estar acessível no caminho esperado pelo comando de execução.

## 9. Estrutura do projeto

A estrutura abaixo é uma referência baseada nos módulos e arquivos descritos durante o desenvolvimento. Ajuste os caminhos caso a organização atual do repositório seja diferente:

```text
Projeto-de-Mutador-para-Lua/
├── src/
│   ├── main.rs       # Inicialização e coordenação do processo
│   ├── ast.rs        # Módulo com o método de AST para parsing sintático do código Lua
│   ├── scanner.rs    # Delimitação da região e identificação dos candidatos
│   └── mutador.rs    # Geração e execução das mutações
├── codigo.lua        # Código Lua submetido à mutação
├── suit.lua          # Suíte de testes
├── Cargo.toml        # Configuração do projeto e dependências Rust
└── README.md
```

## 10. Limitações e pontos em desenvolvimento

O projeto está em desenvolvimento. Alguns pontos devem ser considerados ao interpretar os resultados:

### 🚨 PRIORIDADE ALTA / PRÓXIMO PASSO
- **Reformulação do `scanner.rs` utilizando AST:** A principal prioridade no planejamento do projeto é a refatoração completa do `scanner.rs` para utilizar a AST gerada pelo método em `ast.rs`. Atualmente a varredura é baseada em string/caracteres; ao migrá-la para a AST, o scanner passará a navegar diretamente pelos nós de expressão da árvore sintática, eliminando falso-positivos em comentários ou cadeias de caracteres (strings).

### Outros pontos e limitações
- A identificação dos operadores no scanner atual depende de regras de busca textual e não equivale a uma análise sintática completa até que a transição do `scanner.rs` para a AST seja finalizada.
- A cobertura de operadores e os casos de teste ainda podem ser ampliados.
- A suíte precisa ser executada corretamente para que um mutante seja classificado como morto ou sobrevivente.
- Erros de sintaxe ou falhas na execução do interpretador devem ser diferenciados de resultados de testes.
- O tratamento de marcadores ausentes ou inválidos e de posições inexistentes deve ser robusto para evitar mutações em locais incorretos.

## 11. Objetivo acadêmico

Este projeto explora a aplicação de **teste de mutação** como técnica de avaliação de suítes de testes de software. Ao introduzir pequenas alterações no código-fonte e observar se os testes conseguem detectá-las, é possível identificar possíveis lacunas na estratégia de testes e avaliar a eficácia dos casos de teste desenvolvidos.