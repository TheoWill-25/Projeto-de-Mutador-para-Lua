# Mutador de Código Lua em Rust

Projeto desenvolvido em **Rust** para realizar **testes de mutação** em
programas escritos em **Lua**. A ferramenta utiliza a biblioteca
`full_moon` para analisar sintaticamente o código Lua, construir sua
Árvore de Sintaxe Abstrata (AST) e identificar operadores candidatos à
mutação.

A partir dos operadores encontrados, o mutador gera versões modificadas
do código-fonte, executa uma suíte de testes com o interpretador Lua e
registra os resultados em um relatório JSON.

O objetivo é avaliar a capacidade de uma suíte de testes de detectar
pequenas alterações no comportamento de um programa e, assim, ajudar a
revelar possíveis lacunas nos casos de teste.

## 1. Arquitetura e arquivos do projeto

### `main.rs`

É o ponto de entrada e coordena o fluxo principal:

1.  Lê o arquivo `codigo.lua`.
2.  Envia o código para o parser Lua.
3.  Obtém a AST.
4.  Executa o scanner sobre a AST para localizar os operadores
    candidatos.
5.  Encaminha os candidatos e o código original ao mutador.
6.  Filtra as mutações sobreviventes e gera `relatorio.json`.
7.  Apresenta a quantidade total de mutações e de sobreviventes.

### `parser.rs`

Responsável por realizar o parsing do código Lua por meio da biblioteca
`full_moon`.

A função `parse_lua(code)` chama `full_moon::parse` e retorna uma
estrutura `full_moon::ast::Ast`. Caso o código não possa ser
interpretado, a implementação atual interrompe a execução com uma
mensagem de erro.

### `scanner.rs`

Contém a implementação do scanner baseada na AST.

O módulo utiliza o trait `Visitor` de `full_moon` para percorrer a
árvore sintática. Ao visitar expressões, verifica se elas são expressões
binárias (`Expression::BinaryOperator`) e identifica os operadores
suportados.

Cada ocorrência encontrada é armazenada em uma estrutura `Candidate`,
que registra:

-   `line`: linha em que o operador aparece;
-   `start`: posição inicial do token em bytes;
-   `end`: posição final do token em bytes;
-   `original`: representação textual do operador;
-   `kind`: categoria do operador.

O scanner atual identifica operadores relacionais, de igualdade e
lógicos, conforme as variantes tratadas no `match` de
`visit_expression`.

**Diferentemente de uma busca textual**, a identificação ocorre sobre
nós sintáticos da AST. Dessa forma, o scanner considera operadores
reconhecidos como parte de expressões binárias, em vez de procurar
indiscriminadamente sequências de caracteres no arquivo.

### `mutador.rs`

Responsável por gerar, executar e registrar os mutantes.

Para cada candidato recebido, o módulo:

1.  Consulta `symbol_list` para obter as substituições previstas para o
    operador original.
2.  Cria uma cópia do código-fonte original.
3.  Substitui o intervalo de bytes correspondente ao token identificado
    na AST.
4.  Escreve temporariamente o código modificado em `codigo.lua`.
5.  Executa `suit.lua` por meio do processo externo `lua`.
6.  Analisa a saída padrão em busca das marcações `[PASS]` e `[FAIL]`.
7.  Armazena os dados da mutação em uma estrutura `Mutation`.

Ao final, o código original é restaurado em `codigo.lua`.

A estrutura `Mutation` registra o candidato original, o operador
substituto, a linha original, a linha modificada, a quantidade de testes
contabilizados e os nomes dos testes que falharam.

### `codigo.lua`

Arquivo Lua submetido à análise e à mutação.

No exemplo utilizado durante o desenvolvimento, contém a implementação
de `ScholarshipEligibilityEvaluator.evaluate_scholarship`, responsável
por avaliar a elegibilidade de uma pessoa para uma bolsa.

### `suit.lua`

Contém os testes executados contra o código original e cada versão
mutante. A suíte utiliza `pcall` para capturar falhas durante a execução
de cada caso e imprime as marcações `[PASS]` e `[FAIL]`, que são
interpretadas pelo mutador.

## 2. Análise sintática com AST

A integração com AST substitui a identificação dos operadores baseada
exclusivamente em varredura textual.

O processo é composto por duas etapas:

1.  **Parsing:** `parser.rs` utiliza `full_moon::parse` para converter o
    código Lua em uma estrutura AST.
2.  **Visitação:** `scanner.rs` implementa
    `full_moon::visitors::Visitor` e percorre a árvore, examinando
    expressões binárias.

Quando uma expressão binária contém um operador contemplado pela
implementação, o scanner registra o token e suas posições no
código-fonte. Essas posições são utilizadas posteriormente pelo mutador
para substituir somente o operador selecionado.

Essa abordagem permite que a identificação seja orientada pela estrutura
sintática do programa, reduzindo falsos positivos que poderiam ocorrer
em uma busca textual, por exemplo, ao encontrar símbolos semelhantes
dentro de comentários ou strings.

**Observação sobre o escopo:** na implementação apresentada, o scanner
percorre a AST inteira. A delimitação por marcadores `--start_mut();` e
`--end_mut();` não está integrada ao fluxo atual. Portanto, não se deve
considerar que somente uma região delimitada é analisada.

## 3. Fluxo de funcionamento

O fluxo geral da ferramenta é:

1.  **Leitura:** obtém o conteúdo de `codigo.lua`.
2.  **Parsing:** converte o código Lua em uma AST usando `full_moon`.
3.  **Visitação da AST:** percorre as expressões sintáticas e identifica
    os operadores suportados.
4.  **Registro dos candidatos:** armazena operador, categoria, linha e
    posições em bytes.
5.  **Geração dos mutantes:** cria uma versão independente do código
    original para cada substituição.
6.  **Execução da suíte:** grava o mutante em `codigo.lua` e executa
    `lua suit.lua`.
7.  **Coleta dos resultados:** contabiliza as linhas `[PASS]` e `[FAIL]`
    encontradas na saída padrão.
8.  **Restauração:** grava novamente o código original em `codigo.lua`.
9.  **Relatório:** serializa as mutações sobreviventes em
    `relatorio.json` e imprime as estatísticas no terminal.

## 4. Operadores e substituições

As substituições são definidas pela função `symbol_list` em
`mutador.rs`. As regras presentes na implementação enviada são:

  Operador original   Substituições previstas
  ------------------- ----------------------------
  `<`                 `>`, `<=`, `==`
  `>`                 `<`, `>=`, `==`
  `~=`                `==`
  `~`                 remoção do operador
  `<=`                `<`, `>`, `==`
  `>=`                `<`, `>`, `==`
  `==`                `<`, `>`, `<=`, `>=`, `~=`
  `and`               `or`
  `or`                `and`

A lista acima representa as regras implementadas no código enviado. O
scanner identifica os operadores lógicos e relacionais tratados
explicitamente em `visit_expression`; a função de mutação define as
substituições disponíveis para cada um.

## 5. Resultados e classificação dos mutantes

Em testes de mutação, os resultados são interpretados da seguinte
maneira:

-   **Killed (morto):** pelo menos um teste falha ao executar contra o
    mutante.
-   **Survived (sobrevivente):** os testes executados não detectam a
    alteração.
-   **Inconclusive (inconclusivo):** a execução não permite concluir se
    o mutante foi adequadamente testado, por exemplo, devido a erro de
    execução ou ausência de evidências de que a suíte foi executada.

Na implementação atual, `main.rs` considera sobreviventes as mutações
cujo vetor `failed_tests` está vazio. O `mutador.rs` contabiliza as
linhas `[PASS]` e `[FAIL]` encontradas em `stdout`.

**Limitação importante:** a implementação enviada ainda não verifica o
código de saída do processo Lua, não interpreta `stderr` e não valida se
a quantidade esperada de testes foi executada. Assim, a classificação
atual por `failed_tests.is_empty()` não distingue completamente uma
execução válida de uma execução que não produziu resultados. A
classificação robusta de inconclusivos é um aprimoramento necessário.

### 5.1. Relatório JSON

O arquivo `relatorio.json` contém as mutações classificadas como
sobreviventes pelo critério atual.

Cada registro de mutação possui:

  -----------------------------------------------------------------------
  Campo                               Descrição
  ----------------------------------- -----------------------------------
  `candidate`                         Dados do operador original
                                      identificado pelo scanner.

  `replacement`                       Operador utilizado na mutação.

  `og_line`                           Número da linha original,
                                      armazenado como texto.

  `mod_line`                          Conteúdo da linha após a
                                      substituição.

  `num_tests`                         Quantidade de marcações `[PASS]` e
                                      `[FAIL]` contabilizadas.

  `failed_tests`                      Nomes dos testes que falharam com o
                                      mutante.
  -----------------------------------------------------------------------

## 6. Tecnologias e dependências

-   **Rust:** implementação do fluxo de análise, scanner, mutador e
    geração dos relatórios.
-   **Lua:** linguagem dos programas analisados e ambiente de execução
    dos testes.
-   **`full_moon`:** parsing do código Lua e geração da AST.
-   **`serde`:** serialização das estruturas de dados.
-   **`serde_json`:** geração dos relatórios em JSON.
-   **Cargo:** gerenciamento do projeto e de suas dependências Rust.

## 7. Requisitos

Para executar o projeto, é necessário ter:

-   Rust e Cargo instalados;
-   Interpretador Lua instalado e disponível no `PATH`;
-   Dependências Rust declaradas no `Cargo.toml`.

## 8. Como executar

Clone o repositório:

``` bash
git clone https://github.com/TheoWill-25/Projeto-de-Mutador-para-Lua.git
cd Projeto-de-Mutador-para-Lua
```

Compile e execute:

``` bash
cargo run
```

A execução espera encontrar `codigo.lua` e `suit.lua` nos caminhos
relativos utilizados pelo programa. O comando externo `lua suit.lua`
também precisa estar disponível no ambiente.

Ao terminar, o programa apresenta no terminal a quantidade total de
mutações e a quantidade de mutações sobreviventes. O relatório das
sobreviventes é gravado em `relatorio.json`.

## 9. Estrutura do projeto

A estrutura correspondente aos arquivos apresentados é:

``` text
Projeto-de-Mutador-para-Lua/
├── src/
│   ├── main.rs       # Coordenação do fluxo
│   ├── parser.rs     # Parsing Lua e geração da AST
│   ├── scanner.rs    # Visitação da AST e identificação dos operadores
│   └── mutador.rs    # Geração, execução e registro dos mutantes
├── codigo.lua        # Código Lua submetido à mutação
├── suit.lua          # Suíte de testes
├── Cargo.toml        # Dependências e configuração Rust
├── AST.json          # AST serializada pelo main.rs
└── relatorio.json    # Relatório das mutações sobreviventes
```

Os arquivos `AST.json` e `relatorio.json` são gerados durante a
execução. A localização efetiva dos módulos e dos arquivos Lua depende
da configuração do projeto e do diretório de execução.

## 10. Limitações e próximos passos

Embora a identificação dos operadores já utilize a AST, há pontos que
podem ser aprimorados:

-   **Delimitação da região de mutação:** integrar os marcadores
    `--start_mut();` e `--end_mut();` à análise da AST, para restringir
    a visita aos nós localizados na região selecionada.
-   **Validação da execução:** verificar o status de saída do processo
    Lua e capturar também `stderr`.
-   **Classificação de inconclusivos:** impedir que uma execução sem
    testes contabilizados seja considerada sobrevivente.
-   **Validação da suíte:** conferir se a quantidade de testes
    executados corresponde à quantidade esperada.
-   **Cobertura de mutadores:** ampliar e revisar as regras de
    substituição para outros operadores e construções da linguagem.
-   **Tratamento de erros:** tornar mais robustos os casos de marcadores
    ausentes, intervalos inválidos, falhas de parsing e problemas ao
    executar o interpretador.
-   **Isolamento das execuções:** evitar que a escrita temporária em
    `codigo.lua` cause interferência entre execuções simultâneas ou
    deixe o arquivo alterado caso ocorra uma falha inesperada.

## 11. Objetivo acadêmico

O projeto explora a aplicação de **teste de mutação** como técnica de
avaliação de suítes de testes de software.

A utilização da AST permite identificar operadores a partir da estrutura
sintática do programa, tornando a seleção dos pontos de mutação mais
precisa do que uma busca textual simples. A geração de versões mutantes
e a execução automatizada dos testes permitem observar se as alterações
são detectadas pela suíte e identificar possíveis lacunas na cobertura
dos testes.
