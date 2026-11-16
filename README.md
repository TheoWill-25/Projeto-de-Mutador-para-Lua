# Mutador de Código Lua em Rust

Projeto desenvolvido em **Rust** para realizar **testes de mutação** em programas escritos em **Lua**. A ferramenta utiliza a biblioteca `full_moon` para analisar sintaticamente o código Lua, construir sua Árvore de Sintaxe Abstrata (AST) e identificar operadores e literais candidatos à mutação.

A partir dos candidatos encontrados, o mutador gera versões modificadas do código-fonte, executa uma suíte de testes com o interpretador Lua e registra os resultados em um relatório JSON. Tudo isso pode ser feito por meio de uma **interface gráfica** (construída com `eframe`/`egui`), na qual o usuário escolhe o código e a suíte de testes e acompanha o resultado na própria janela.

O objetivo é avaliar a capacidade de uma suíte de testes de detectar pequenas alterações no comportamento de um programa e, assim, ajudar a revelar possíveis lacunas nos casos de teste.

## 1. Arquitetura e arquivos do projeto

### `main.rs`

É o ponto de entrada. Além de configurar e exibir a interface gráfica (ver seção 3), coordena o fluxo principal na função `executar`:

1. Lê o arquivo Lua selecionado pelo usuário.
2. Envia o código para o parser Lua e obtém a AST.
3. Serializa a AST em `AST.json`.
4. Executa o scanner sobre a AST para localizar os candidatos à mutação.
5. Encaminha os candidatos e o código original ao mutador.
6. Filtra as mutações sobreviventes e gera `relatorio.json`.
7. Devolve à interface a quantidade total de mutações, de sobreviventes e de mutações com erro.

Esse fluxo roda em uma **thread separada**, para que a janela não congele enquanto a suíte de testes é executada para cada mutante.

### `parser.rs`

Responsável por realizar o parsing do código Lua por meio da biblioteca `full_moon`.

A função `parse_lua(code)` chama `full_moon::parse` e retorna uma estrutura `full_moon::ast::Ast`. Caso o código não possa ser interpretado, a implementação atual interrompe a execução com uma mensagem de erro (a interface informa que a execução falhou e os detalhes aparecem no terminal).

### `scanner.rs`

Contém a implementação do scanner baseada na AST.

O módulo utiliza o trait `Visitor` de `full_moon` para percorrer a árvore sintática. Ao visitar expressões, verifica se elas são expressões binárias (`Expression::BinaryOperator`), unárias (`Expression::UnaryOperator`) ou números (`Expression::Number`) e identifica os candidatos suportados.

Cada ocorrência encontrada é armazenada em uma estrutura `Candidate`, que registra:

- `line`: linha em que o candidato aparece;
- `start`: posição inicial do token em bytes;
- `end`: posição final do token em bytes;
- `original`: representação textual do operador ou literal;
- `kind`: categoria do candidato (`ComparisonOperator`, `LogicOperator`, `ArithmeticOperator` ou `NumberValue`).

Os candidatos identificados atualmente são:

| Categoria | Candidatos |
| --- | --- |
| Relacionais e de igualdade | `<`, `>`, `<=`, `>=`, `==`, `~=` |
| Lógicos | `and`, `or` e `not` (este último como operador unário) |
| Aritméticos binários | `+`, `-`, `*`, `/`, `%`, `^` |
| Literais numéricos | números reconhecidos como `Expression::Number` |

O `-` só é coletado quando aparece como operador **binário** (por exemplo, `a - b`). O sinal de um número negativo escrito diretamente (por exemplo, `-5`) não é tratado como parte do literal numérico pela regra atual.

**Diferentemente de uma busca textual**, a identificação ocorre sobre nós sintáticos da AST. Dessa forma, o scanner considera apenas o que é reconhecido como parte de expressões, em vez de procurar indiscriminadamente sequências de caracteres no arquivo.

### `mutador.rs`

Responsável por gerar, executar e registrar os mutantes.

Para cada candidato recebido, o módulo:

1. Consulta `symbol_list` para obter as substituições previstas para o candidato.
2. Cria uma cópia do código-fonte original.
3. Substitui o intervalo de bytes correspondente ao token identificado na AST.
4. Escreve temporariamente o código modificado no arquivo Lua selecionado.
5. Executa a suíte de testes por meio do processo externo `lua`.
6. Analisa a saída padrão em busca das marcações `[PASS]` e `[FAIL]` e do resumo final (`Tests passed:` / `Tests failed:`).
7. Armazena os dados da mutação em uma estrutura `Mutation`.

Ao final, o código original é restaurado no arquivo Lua.

A estrutura `Mutation` registra o candidato original, o valor substituto, a linha original, a linha modificada, a quantidade de testes contabilizados, os nomes dos testes que falharam e um indicador de erro (`error`), ativado quando a saída da suíte não contém o resumo esperado.

### Arquivos Lua (código e suíte)

O código a ser mutado e a suíte de testes são **escolhidos pelo usuário na interface**; não é necessário que tenham nomes fixos. No exemplo utilizado durante o desenvolvimento, `codigo.lua` contém a implementação de `ScholarshipEligibilityEvaluator.evaluate_scholarship`, responsável por avaliar a elegibilidade de uma pessoa para uma bolsa, e `suit.lua` contém os testes.

**Formato esperado da suíte:** ela deve utilizar `pcall` para capturar falhas de cada caso e imprimir, na saída padrão:

- uma linha `[PASS] nome_do_teste` para cada teste aprovado;
- uma linha `[FAIL] nome_do_teste` para cada teste reprovado;
- as linhas de resumo `Tests passed:` e `Tests failed:` ao final.

Se o resumo não aparecer (por exemplo, por erro de execução), a mutação é marcada como erro.

## 2. Análise sintática com AST

A integração com AST substitui a identificação baseada exclusivamente em varredura textual. O processo é composto por duas etapas:

1. **Parsing:** `parser.rs` utiliza `full_moon::parse` para converter o código Lua em uma estrutura AST.
2. **Visitação:** `scanner.rs` implementa `full_moon::visitors::Visitor` e percorre a árvore, examinando expressões binárias, unárias e literais numéricos.

Quando uma expressão contém um operador contemplado pela implementação ou um literal numérico, o scanner registra o token e suas posições no código-fonte. Essas posições são utilizadas posteriormente pelo mutador para substituir somente o trecho selecionado.

Essa abordagem permite que a identificação seja orientada pela estrutura sintática do programa, reduzindo falsos positivos que poderiam ocorrer em uma busca textual, por exemplo, ao encontrar símbolos semelhantes dentro de comentários ou strings.

**Observação sobre o escopo:** na implementação atual, o scanner percorre a AST inteira. A delimitação por marcadores `--start_mut();` e `--end_mut();` não está integrada ao fluxo. Portanto, não se deve considerar que somente uma região delimitada é analisada.

## 3. Interface gráfica

A interface foi construída com `eframe`/`egui` e segue uma paleta de cores aproximadamente proporcional a **60% branco, 25% cinza/prata e 15% azul**. A janela é dividida em três regiões.

### 3.1. Faixa superior (azul)

Barra com o título **Lua Mutation Tester**. É o principal elemento de identidade visual da aplicação.

### 3.2. Barra lateral (cinza, cerca de 25% da largura)

Concentra os controles de entrada:

- **Código Lua:** botão *Selecionar código*, que abre o seletor de arquivos (filtrado para `.lua`). Depois da escolha, o nome do arquivo é exibido em azul; antes disso, aparece "Nenhum selecionado".
- **Suíte de testes:** botão *Selecionar suíte*, com o mesmo comportamento, para o arquivo que contém os testes.
- **Executar mutador:** botão azul de destaque. Só fica habilitado quando o código e a suíte foram selecionados e nenhuma execução está em andamento. Durante a execução, o texto muda para "Executando..." e um indicador de carregamento (*spinner*) é exibido; os botões de seleção ficam desabilitados.

### 3.3. Área de resultados (branca)

Ocupa o restante da janela e muda conforme o estado da aplicação:

- **Estado inicial:** uma mensagem orienta o usuário a selecionar os arquivos e clicar em *Executar mutador*.
- **Cartões de resumo:** após a execução, três cartões mostram as **mutações totais**, as **sobreviventes** (em destaque azul) e as **mutações com erro**.
- **Lista de sobreviventes:** cada mutação que sobreviveu aparece em um bloco com a linha, a categoria do candidato (`kind`) e a comparação entre a linha original (`-`) e a linha mutada (`+`). A lista possui rolagem. Se nenhuma mutação sobreviveu, a interface informa que a suíte detectou todas.
- **Mensagens de erro:** falhas de leitura, de escrita do relatório ou da própria execução são exibidas em vermelho nesta área.

### 3.4. Arquivos gerados

Durante a execução, são salvos dois arquivos na pasta do código selecionado:

- `AST.json`: representação serializada da AST;
- `relatorio.json`: mutações sobreviventes.

> **Atenção:** enquanto a execução está em andamento, o arquivo de código é sobrescrito temporariamente com cada mutante. Evite editá-lo ou fechar a janela nesse intervalo.

## 4. Fluxo de funcionamento

O fluxo geral da ferramenta é:

1. **Seleção:** o usuário escolhe, na interface, o código Lua e a suíte de testes.
2. **Leitura:** o conteúdo do código é lido.
3. **Parsing:** o código é convertido em uma AST usando `full_moon`, que é serializada em `AST.json`.
4. **Visitação da AST:** o scanner percorre as expressões e identifica os operadores e literais suportados.
5. **Registro dos candidatos:** armazena o operador ou literal, a categoria, a linha e as posições em bytes.
6. **Geração dos mutantes:** cria uma versão independente do código original para cada substituição.
7. **Execução da suíte:** grava o mutante no arquivo de código e executa `lua` sobre a suíte.
8. **Coleta dos resultados:** contabiliza as linhas `[PASS]` e `[FAIL]` da saída padrão e verifica a presença do resumo.
9. **Restauração:** grava novamente o código original.
10. **Relatório:** salva as mutações sobreviventes em `relatorio.json` e exibe as estatísticas na interface.

## 5. Operadores e substituições

As substituições são definidas pela função `symbol_list` em `mutador.rs`. As regras presentes na implementação atual são as seguintes.

### 5.1. Operadores relacionais

| Operador original | Substituições |
| --- | --- |
| `<` | `>`, `<=`, `==`, `>=`, `~=` |
| `>` | `<`, `>=`, `==`, `<=`, `~=` |
| `~=` | `==`, `<`, `>`, `<=`, `>=` |
| `<=` | `<`, `>`, `==`, `>=`, `~=` |
| `>=` | `<`, `>`, `==`, `<=`, `~=` |
| `==` | `<`, `>`, `<=`, `>=`, `~=` |

### 5.2. Operadores lógicos

| Operador original | Substituição |
| --- | --- |
| `and` | `or` |
| `or` | `and` |
| `not` | remoção do operador (substituído por uma string vazia) |

### 5.3. Operadores aritméticos

| Operador original | Substituições |
| --- | --- |
| `+` | `-`, `*`, `/`, `%`, `^` |
| `-` | `+`, `*`, `/`, `%`, `^` |
| `*` | `+`, `-`, `/`, `%`, `^` |
| `/` | `+`, `-`, `*`, `%`, `^` |
| `%` | `+`, `-`, `*`, `/`, `^` |
| `^` | `+`, `-`, `*`, `/`, `%` |

Os operadores `//` (divisão inteira), concatenação (`..`) e operadores bit a bit ainda não são coletados pelo scanner.

### 5.4. Literais numéricos

O scanner também identifica literais numéricos (`NumberValue`). O mutador tenta interpretar o texto do literal primeiro como `i64` e, se isso falhar, como `f64`. Para inteiros e números de ponto flutuante, as mutações previstas são:

| Substituição | Condição |
| --- | --- |
| `0` | valor original diferente de `0` |
| `-valor` | valor original diferente de `0` |
| `1` | valor original diferente de `1` |
| `-1` | valor original diferente de `-1` |
| `valor + 1` | sempre |
| `valor - 1` | sempre |

As substituições por constantes são omitidas quando o valor já é igual ao de destino, evitando mutantes idênticos ao original.

**Limitações numéricas:** a conversão utiliza `i64` e `f64` do Rust. Formatos numéricos de Lua que não sejam aceitos por essas conversões, como determinados literais hexadecimais, podem não gerar substituições. Além disso, as operações de incremento, decremento e inversão de sinal com inteiros ainda não possuem tratamento explícito de overflow.

## 6. Resultados e classificação dos mutantes

Em testes de mutação, os resultados são interpretados da seguinte maneira:

- **Killed (morto):** pelo menos um teste falha ao executar contra o mutante.
- **Survived (sobrevivente):** os testes executados não detectam a alteração.
- **Inconclusive / erro:** a execução não permite concluir se o mutante foi adequadamente testado, por exemplo, devido a erro de execução ou ausência do resumo da suíte na saída.

Na implementação atual, são considerados **sobreviventes** os mutantes com `failed_tests` vazio **e** `error` falso. Mutantes com `error` verdadeiro são contabilizados à parte (cartão "Com erro" na interface) e não entram no relatório de sobreviventes. O `stderr` das execuções com erro é impresso no terminal.

**Limitações:**

- o código de saída do processo Lua não é verificado;
- não há validação de que a quantidade esperada de testes foi executada;
- não há limite de tempo (*timeout*) por execução, de modo que um mutante que provoque laço infinito mantém a execução em andamento indefinidamente.

### 6.1. Relatório JSON

O arquivo `relatorio.json` contém as mutações classificadas como sobreviventes pelo critério atual. Cada registro possui:

| Campo | Descrição |
| --- | --- |
| `candidate` | Dados do operador ou literal original identificado pelo scanner. |
| `replacement` | Valor utilizado na mutação. |
| `og_line` | Conteúdo da linha original. |
| `mod_line` | Conteúdo da linha após a substituição. |
| `num_tests` | Quantidade de marcações `[PASS]` e `[FAIL]` contabilizadas. |
| `failed_tests` | Nomes dos testes que falharam com o mutante. |
| `error` | Indica se a execução não produziu o resumo esperado da suíte. |

## 7. Tecnologias e dependências

- **Rust:** implementação do fluxo de análise, scanner, mutador, interface e geração dos relatórios (edição 2024).
- **Lua:** linguagem dos programas analisados e ambiente de execução dos testes.
- **Cargo:** gerenciamento do projeto e de suas dependências Rust.

Bibliotecas Rust utilizadas (declaradas no `Cargo.toml`):

| Biblioteca | Versão | Uso no projeto |
| --- | --- | --- |
| `eframe` | 0.37.0 | Janela e interface gráfica (inclui o `egui`) |
| `full_moon` | 2.2 | Parsing do código Lua e geração da AST (feature `serde` habilitada) |
| `rfd` | 0.18.0 | Janelas de seleção de arquivos |
| `serde` | 1 | Serialização das estruturas de dados (feature `derive`) |
| `serde_json` | 1 | Geração dos relatórios em JSON |

Trecho do `Cargo.toml`:

```toml
[package]
name = "project"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = "0.37.0"
full_moon = { version = "2.2", features = ["serde"] }
rfd = "0.18.0"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## 8. Requisitos e instalação (Windows)

O projeto foi desenvolvido e executado no **Windows**, usando a toolchain **GNU** do Rust. Os comandos abaixo podem ser executados no **Prompt de Comando (cmd)**.

Requisitos:

- Rust e Cargo (toolchain `x86_64-pc-windows-gnu`, com suporte à edição 2024, isto é, Rust 1.85 ou superior);
- MinGW-w64 (`gcc`) disponível no `PATH`, necessário para a toolchain GNU;
- Interpretador Lua instalado e disponível no `PATH`;
- Dependências Rust declaradas no `Cargo.toml` (baixadas e compiladas automaticamente pelo Cargo na primeira execução).

### 8.1. Instalando o Lua pelo cmd

Com o `winget` (incluído nas versões atuais do Windows):

```bat
winget install --id DEVCOM.Lua
```

Se o identificador não for encontrado, procure-o com `winget search lua`. Alternativas com outros gerenciadores de pacotes:

```bat
:: Chocolatey
choco install lua

:: Scoop
scoop install lua
```

### 8.2. Instalando o Rust (toolchain GNU) pelo cmd

**1. Instale o `rustup`:**

```bat
winget install --id Rustlang.Rustup
```

Abra um novo cmd para que o comando `rustup` seja reconhecido.

**2. Instale e selecione a toolchain GNU como padrão:**

```bat
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

**3. Instale o MinGW-w64 (compilador/linker `gcc`) via MSYS2:**

```bat
winget install --id MSYS2.MSYS2
C:\msys64\usr\bin\bash.exe -lc "pacman -S --noconfirm mingw-w64-x86_64-toolchain"
```

**4. Adicione `C:\msys64\mingw64\bin` ao `PATH`** (em *Configurações do sistema → Variáveis de ambiente*).

### 8.3. Aplicando as mudanças e verificando

As variáveis de ambiente, incluindo o `PATH`, são herdadas pelos programas no momento em que eles são iniciados. Por isso, depois de instalar o Lua, o Rust e o MinGW-w64, **feche completamente o VS Code (todas as janelas) e abra-o novamente**. Apenas abrir um novo terminal dentro de uma janela antiga pode não atualizar o `PATH`. Isso vale também para a extensão Rust Analyzer, se instalada.

Em seguida, confirme as instalações:

```bat
lua -v
rustc -vV
cargo -V
gcc --version
```

Na saída de `rustc -vV`, a linha `host` deve indicar `x86_64-pc-windows-gnu`. Se algum comando não for reconhecido, verifique se a pasta do respectivo executável está no `PATH`.

## 9. Como executar

Clone o repositório:

```bat
git clone https://github.com/TheoWill-25/Projeto-de-Mutador-para-Lua.git
cd Projeto-de-Mutador-para-Lua
```

Compile e execute:

```bat
cargo run
```

Na primeira execução, o Cargo lê o `Cargo.toml`, baixa as bibliotecas declaradas em `[dependencies]` e as compila junto com o projeto. **Não é necessário instalar essas bibliotecas manualmente**, apenas ter conexão com a internet nessa primeira compilação. As dependências ficam em cache, então as execuções seguintes são mais rápidas. Se o `Cargo.toml` mudar, o Cargo baixa e compila automaticamente as dependências novas ou atualizadas.

Ao abrir a janela:

1. Clique em **Selecionar código** e escolha o arquivo Lua a ser mutado.
2. Clique em **Selecionar suíte** e escolha o arquivo com os testes.
3. Clique em **Executar mutador** e aguarde o fim da execução.
4. Consulte os resultados na própria janela e nos arquivos `relatorio.json` e `AST.json`, gerados na pasta do código selecionado.

O comando externo `lua` precisa estar disponível no `PATH`.

## 10. Estrutura do projeto

A estrutura correspondente aos arquivos apresentados é:

```text
Projeto-de-Mutador-para-Lua/
├── src/
│   ├── main.rs       # Interface gráfica e coordenação do fluxo
│   ├── parser.rs     # Parsing Lua e geração da AST
│   ├── scanner.rs    # Visitação da AST e identificação dos candidatos
│   └── mutador.rs    # Geração, execução e registro dos mutantes
├── codigo.lua        # Exemplo de código Lua submetido à mutação
├── suit.lua          # Exemplo de suíte de testes
├── Cargo.toml        # Dependências e configuração Rust
├── AST.json          # AST serializada do código analisado (gerado)
└── relatorio.json    # Relatório das mutações sobreviventes (gerado)
```

Os arquivos `AST.json` e `relatorio.json` são gerados durante a execução, na pasta do código Lua selecionado.

## 11. Limitações e próximos passos

Embora a identificação dos candidatos já utilize a AST, há pontos que podem ser aprimorados:

- **Delimitação da região de mutação:** integrar os marcadores `--start_mut();` e `--end_mut();` à análise da AST, para restringir a visita aos nós localizados na região selecionada.
- **Validação da execução:** verificar o status de saída do processo Lua.
- **Tempo limite:** interromper execuções que excedam um tempo máximo, evitando travamentos causados por laços infinitos em mutantes.
- **Validação da suíte:** conferir se a quantidade de testes executados corresponde à quantidade esperada e se a suíte passa no código original antes da mutação.
- **Cobertura de operadores:** ampliar a cobertura para outros operadores e construções da linguagem, como `//`, concatenação (`..`), operadores unários e bit a bit.
- **Cobertura numérica:** ampliar o reconhecimento de formatos numéricos aceitos por Lua e tratar overflow nas operações com inteiros.
- **Mutação de operandos:** avaliar a inclusão de mutações para outros tipos de valores, como strings, booleanos, `nil` e identificadores.
- **Tratamento de erros:** tornar mais robustos os casos de falhas de parsing e problemas ao executar o interpretador, evitando interrupções abruptas (`panic`).
- **Isolamento das execuções:** evitar que a escrita temporária no arquivo de código cause interferência entre execuções simultâneas ou deixe o arquivo alterado caso ocorra uma falha inesperada (por exemplo, executando os mutantes sobre uma cópia).
- **Interface:** exibir também os mutantes mortos e com erro, e permitir exportar o relatório pela janela.

## 12. Objetivo acadêmico

O projeto explora a aplicação de **teste de mutação** como técnica de avaliação de suítes de testes de software.

A utilização da AST permite identificar operadores a partir da estrutura sintática do programa, tornando a seleção dos pontos de mutação mais precisa do que uma busca textual simples. A geração de versões mutantes e a execução automatizada dos testes permitem observar se as alterações são detectadas pela suíte e identificar possíveis lacunas na cobertura dos testes.