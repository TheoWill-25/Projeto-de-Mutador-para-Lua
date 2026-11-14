# Mutador de Código Lua em Rust

Projeto desenvolvido em **Rust** para realizar **testes de mutação** em
programas escritos em **Lua**. A ferramenta utiliza a biblioteca
`full_moon` para analisar sintaticamente o código Lua, construir sua
Árvore de Sintaxe Abstrata (AST) e identificar operadores candidatos à
mutação.

A partir dos operadores encontrados, o mutador gera versões modificadas
do código-fonte, executa uma suíte de testes com o interpretador Lua e
registra os resultados em um relatório JSON. Tudo isso pode ser feito
por meio de uma **interface gráfica** (construída com `eframe`/`egui`),
na qual o usuário escolhe o código e a suíte de testes e acompanha o
resultado na própria janela.

O objetivo é avaliar a capacidade de uma suíte de testes de detectar
pequenas alterações no comportamento de um programa e, assim, ajudar a
revelar possíveis lacunas nos casos de teste.

## 1. Arquitetura e arquivos do projeto

### `main.rs`

É o ponto de entrada. Além de configurar e exibir a interface gráfica
(ver seção 3), coordena o fluxo principal na função `executar`:

1.  Lê o arquivo Lua selecionado pelo usuário.
2.  Envia o código para o parser Lua.
3.  Obtém a AST.
4.  Executa o scanner sobre a AST para localizar os operadores
    candidatos.
5.  Encaminha os candidatos e o código original ao mutador.
6.  Filtra as mutações sobreviventes e gera `relatorio.json`.
7.  Devolve à interface a quantidade total de mutações, de sobreviventes
    e de mutações com erro.

Esse fluxo roda em uma **thread separada**, para que a janela não
congele enquanto a suíte de testes é executada para cada mutante.

### `parser.rs`

Responsável por realizar o parsing do código Lua por meio da biblioteca
`full_moon`.

A função `parse_lua(code)` chama `full_moon::parse` e retorna uma
estrutura `full_moon::ast::Ast`. Caso o código não possa ser
interpretado, a implementação atual interrompe a execução com uma
mensagem de erro (a interface informa que a execução falhou e os
detalhes aparecem no terminal).

### `scanner.rs`

Contém a implementação do scanner baseada na AST.

O módulo utiliza o trait `Visitor` de `full_moon` para percorrer a
árvore sintática. Ao visitar expressões, verifica se elas são expressões
binárias (`Expression::BinaryOperator`) ou unárias
(`Expression::UnaryOperator`) e identifica os operadores suportados.

Cada ocorrência encontrada é armazenada em uma estrutura `Candidate`,
que registra:

-   `line`: linha em que o operador aparece;
-   `start`: posição inicial do token em bytes;
-   `end`: posição final do token em bytes;
-   `original`: representação textual do operador;
-   `kind`: categoria do operador (`ComparisonOperator`,
    `LogicOperator` ou `ArithmeticOperator`).

Os operadores identificados atualmente são:

-   **Relacionais e de igualdade:** `<`, `>`, `<=`, `>=`, `==`, `~=`;
-   **Lógicos:** `and`, `or` e `not` (este último como operador unário);
-   **Aritméticos:** `+`, `-`, `*`, `/`.

**Diferentemente de uma busca textual**, a identificação ocorre sobre
nós sintáticos da AST. Dessa forma, o scanner considera operadores
reconhecidos como parte de expressões, em vez de procurar
indiscriminadamente sequências de caracteres no arquivo.

### `mutador.rs`

Responsável por gerar, executar e registrar os mutantes.

Para cada candidato recebido, o módulo:

1.  Consulta `symbol_list` para obter as substituições previstas para o
    operador original.
2.  Cria uma cópia do código-fonte original.
3.  Substitui o intervalo de bytes correspondente ao token identificado
    na AST.
4.  Escreve temporariamente o código modificado no arquivo Lua
    selecionado.
5.  Executa a suíte de testes por meio do processo externo `lua`.
6.  Analisa a saída padrão em busca das marcações `[PASS]` e `[FAIL]` e
    do resumo final (`Tests passed:` / `Tests failed:`).
7.  Armazena os dados da mutação em uma estrutura `Mutation`.

Ao final, o código original é restaurado no arquivo Lua.

A estrutura `Mutation` registra o candidato original, o operador
substituto, a linha original, a linha modificada, a quantidade de testes
contabilizados, os nomes dos testes que falharam e um indicador de erro
(`error`), que é ativado quando a saída da suíte não contém o resumo
esperado.

### Arquivos Lua (código e suíte)

O código a ser mutado e a suíte de testes são **escolhidos pelo usuário
na interface**; não é mais necessário que tenham nomes fixos. No exemplo
utilizado durante o desenvolvimento, `codigo.lua` contém a implementação
de `ScholarshipEligibilityEvaluator.evaluate_scholarship`, responsável
por avaliar a elegibilidade de uma pessoa para uma bolsa, e `suit.lua`
contém os testes.

**Formato esperado da suíte:** ela deve utilizar `pcall` para capturar
falhas de cada caso e imprimir, na saída padrão:

-   uma linha `[PASS] nome_do_teste` para cada teste aprovado;
-   uma linha `[FAIL] nome_do_teste` para cada teste reprovado;
-   as linhas de resumo `Tests passed:` e `Tests failed:` ao final.

Se o resumo não aparecer (por exemplo, por erro de execução), a mutação
é marcada como erro.

## 2. Análise sintática com AST

A integração com AST substitui a identificação dos operadores baseada
exclusivamente em varredura textual.

O processo é composto por duas etapas:

1.  **Parsing:** `parser.rs` utiliza `full_moon::parse` para converter o
    código Lua em uma estrutura AST.
2.  **Visitação:** `scanner.rs` implementa
    `full_moon::visitors::Visitor` e percorre a árvore, examinando
    expressões binárias e unárias.

Quando uma expressão contém um operador contemplado pela implementação,
o scanner registra o token e suas posições no código-fonte. Essas
posições são utilizadas posteriormente pelo mutador para substituir
somente o operador selecionado.

Essa abordagem permite que a identificação seja orientada pela estrutura
sintática do programa, reduzindo falsos positivos que poderiam ocorrer
em uma busca textual, por exemplo, ao encontrar símbolos semelhantes
dentro de comentários ou strings.

**Observação sobre o escopo:** na implementação apresentada, o scanner
percorre a AST inteira. A delimitação por marcadores `--start_mut();` e
`--end_mut();` não está integrada ao fluxo atual. Portanto, não se deve
considerar que somente uma região delimitada é analisada.

## 3. Interface gráfica

A interface foi construída com `eframe`/`egui` e segue uma paleta de
cores aproximadamente proporcional a **60% branco, 25% cinza/prata e 15%
azul**. A janela é dividida em três regiões:

### 3.1. Faixa superior (azul)

Barra com o título **Lua Mutation Tester**. Funciona como o principal
elemento de identidade visual da aplicação.

### 3.2. Barra lateral (cinza, cerca de 25% da largura)

Concentra os controles de entrada:

-   **Código Lua:** botão *Selecionar código*, que abre o seletor de
    arquivos (filtrado para `.lua`). Depois da escolha, o nome do
    arquivo é exibido em azul; antes disso, aparece "Nenhum
    selecionado".
-   **Suíte de testes:** botão *Selecionar suíte*, com o mesmo
    comportamento, para o arquivo que contém os testes.
-   **Executar mutador:** botão azul de destaque. Só fica habilitado
    quando o código e a suíte foram selecionados e nenhuma execução está
    em andamento. Durante a execução, o texto muda para
    "Executando..." e um indicador de carregamento (*spinner*) é
    exibido; os botões de seleção ficam desabilitados.

### 3.3. Área de resultados (branca)

Ocupa o restante da janela e muda conforme o estado da aplicação:

-   **Estado inicial:** uma mensagem orienta o usuário a selecionar os
    arquivos e clicar em *Executar mutador*.
-   **Cartões de resumo:** após a execução, três cartões mostram as
    **mutações totais**, as **sobreviventes** (em destaque azul) e as
    **mutações com erro**.
-   **Lista de sobreviventes:** cada mutação que sobreviveu aparece em
    um bloco com a linha, a categoria do operador (`kind`) e a
    comparação entre a linha original (`-`) e a linha mutada (`+`). A
    lista possui rolagem. Se nenhuma mutação sobreviveu, a interface
    informa que a suíte detectou todas.
-   **Mensagens de erro:** falhas de leitura, de escrita do relatório ou
    da própria execução são exibidas em vermelho nesta área.

Ao final de cada execução, o relatório das sobreviventes é salvo em
`relatorio.json`, na mesma pasta do código selecionado.

> Enquanto a execução está em andamento, o arquivo de código é
> sobrescrito temporariamente com cada mutante. Evite editá-lo ou fechar
> a janela nesse intervalo.

## 4. Fluxo de funcionamento

O fluxo geral da ferramenta é:

1.  **Seleção:** o usuário escolhe, na interface, o código Lua e a suíte
    de testes.
2.  **Leitura:** o conteúdo do código é lido.
3.  **Parsing:** o código Lua é convertido em uma AST usando
    `full_moon`.
4.  **Visitação da AST:** percorre as expressões sintáticas e identifica
    os operadores suportados.
5.  **Registro dos candidatos:** armazena operador, categoria, linha e
    posições em bytes.
6.  **Geração dos mutantes:** cria uma versão independente do código
    original para cada substituição.
7.  **Execução da suíte:** grava o mutante no arquivo de código e
    executa `lua` sobre a suíte.
8.  **Coleta dos resultados:** contabiliza as linhas `[PASS]` e `[FAIL]`
    encontradas na saída padrão e verifica a presença do resumo.
9.  **Restauração:** grava novamente o código original.
10. **Relatório:** serializa as mutações sobreviventes em
    `relatorio.json` e exibe as estatísticas na interface.

## 5. Operadores e substituições

As substituições são definidas pela função `symbol_list` em
`mutador.rs`. As regras presentes na implementação são:

  Operador original   Substituições previstas
  ------------------- ----------------------------
  `<`                 `>`, `<=`, `==`
  `>`                 `<`, `>=`, `==`
  `~=`                `==`
  `<=`                `<`, `>`, `==`
  `>=`                `<`, `>`, `==`
  `==`                `<`, `>`, `<=`, `>=`, `~=`
  `and`               `or`
  `or`                `and`
  `not`               remoção da negação (substituído por espaço)
  `~`                 remoção do operador (regra existente, mas o scanner atualmente não gera esse candidato)

**Operadores aritméticos (`+`, `-`, `*`, `/`):** já são identificados
pelo scanner, mas ainda **não possuem substituições definidas** em
`symbol_list`. Nesses casos, a regra padrão mantém o operador original,
gerando um mutante idêntico ao código original, que tende a ser
classificado como sobrevivente. A definição de substituições para esses
operadores é um aprimoramento previsto.

## 6. Resultados e classificação dos mutantes

Em testes de mutação, os resultados são interpretados da seguinte
maneira:

-   **Killed (morto):** pelo menos um teste falha ao executar contra o
    mutante.
-   **Survived (sobrevivente):** os testes executados não detectam a
    alteração.
-   **Inconclusive / erro:** a execução não permite concluir se o
    mutante foi adequadamente testado, por exemplo, devido a erro de
    execução ou ausência do resumo da suíte na saída.

Na implementação atual, são considerados **sobreviventes** os mutantes
com `failed_tests` vazio **e** `error` falso. Mutantes com `error`
verdadeiro são contabilizados à parte (cartão "Com erro" na interface) e
não entram no relatório de sobreviventes. O `stderr` das execuções com
erro é impresso no terminal.

**Limitações:** a implementação ainda não verifica o código de saída do
processo Lua, não valida se a quantidade esperada de testes foi
executada e não possui limite de tempo (*timeout*) por execução, de modo
que um mutante que provoque laço infinito mantém a execução em andamento
indefinidamente.

### 6.1. Relatório JSON

O arquivo `relatorio.json` contém as mutações classificadas como
sobreviventes pelo critério atual.

Cada registro de mutação possui:

  -----------------------------------------------------------------------
  Campo                               Descrição
  ----------------------------------- -----------------------------------
  `candidate`                         Dados do operador original
                                      identificado pelo scanner.

  `replacement`                       Operador utilizado na mutação.

  `og_line`                           Conteúdo da linha original.

  `mod_line`                          Conteúdo da linha após a
                                      substituição.

  `num_tests`                         Quantidade de marcações `[PASS]` e
                                      `[FAIL]` contabilizadas.

  `failed_tests`                      Nomes dos testes que falharam com o
                                      mutante.

  `error`                             Indica se a execução não produziu
                                      o resumo esperado da suíte.
  -----------------------------------------------------------------------

## 7. Tecnologias e dependências

-   **Rust:** implementação do fluxo de análise, scanner, mutador,
    interface e geração dos relatórios (edição 2024).
-   **Lua:** linguagem dos programas analisados e ambiente de execução
    dos testes.
-   **Cargo:** gerenciamento do projeto e de suas dependências Rust.

Bibliotecas Rust utilizadas (declaradas no `Cargo.toml`):

  Biblioteca     Versão    Uso no projeto
  -------------- --------- ---------------------------------------------
  `eframe`       0.37.0    Janela e interface gráfica (inclui o `egui`)
  `full_moon`    2.2       Parsing do código Lua e geração da AST
                           (feature `serde` habilitada)
  `rfd`          0.18.0    Janelas de seleção de arquivos
  `serde`        1         Serialização das estruturas de dados
                           (feature `derive`)
  `serde_json`   1         Geração dos relatórios em JSON

Trecho do `Cargo.toml`:

``` toml
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

O projeto foi desenvolvido e executado no **Windows**, usando a
toolchain **GNU** do Rust. Os comandos abaixo podem ser executados no
**Prompt de Comando (cmd)**.

Requisitos:

-   Rust e Cargo (toolchain `x86_64-pc-windows-gnu`, versão com suporte
    à edição 2024, isto é, Rust 1.85 ou superior);
-   MinGW-w64 (`gcc`) disponível no `PATH`, necessário para a toolchain
    GNU;
-   Interpretador Lua instalado e disponível no `PATH`;
-   Dependências Rust declaradas no `Cargo.toml` (baixadas
    automaticamente pelo Cargo).

### 8.1. Instalando o Lua pelo cmd

Com o `winget` (já incluído nas versões atuais do Windows):

``` bat
winget install --id DEVCOM.Lua
```

Se o identificador não for encontrado, procure-o com
`winget search lua`. Alternativas com outros gerenciadores de pacotes:

``` bat
:: Chocolatey
choco install lua

:: Scoop
scoop install lua
```

Feche e abra o terminal novamente e confirme a instalação:

``` bat
lua -v
```

O comando deve exibir a versão instalada. Se aparecer que o comando não
foi reconhecido, adicione a pasta do executável do Lua ao `PATH`.

### 8.2. Instalando o Rust (toolchain GNU) pelo cmd

**1. Baixe e execute o instalador do `rustup`, já selecionando o host
GNU:**

``` bat
curl -L -o rustup-init.exe https://win.rustup.rs/x86_64
rustup-init.exe -y --default-host x86_64-pc-windows-gnu
```

Caso o Rust já esteja instalado com outra toolchain (por exemplo, MSVC),
instale e defina a GNU como padrão:

``` bat
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

**2. Instale o MinGW-w64 (compilador/linker `gcc`) via MSYS2:**

``` bat
winget install --id MSYS2.MSYS2
C:\msys64\usr\bin\bash.exe -lc "pacman -S --noconfirm mingw-w64-x86_64-toolchain"
```

**3. Adicione `C:\msys64\mingw64\bin` ao `PATH`** (em *Configurações do
sistema > Variáveis de ambiente*) e reabra o terminal.

**4. Verifique a instalação:**

``` bat
rustc -vV
cargo -V
gcc --version
```

Na saída de `rustc -vV`, a linha `host` deve indicar
`x86_64-pc-windows-gnu`.

## 9. Como executar

Clone o repositório:

``` bash
git clone https://github.com/TheoWill-25/Projeto-de-Mutador-para-Lua.git
cd Projeto-de-Mutador-para-Lua
```

Compile e execute:

``` bash
cargo run
```

A primeira compilação pode demorar, pois o Cargo baixa e compila todas
as dependências (principalmente `eframe`/`egui`). As compilações
seguintes são mais rápidas.

Ao abrir a janela:

1.  Clique em **Selecionar código** e escolha o arquivo Lua a ser
    mutado.
2.  Clique em **Selecionar suíte** e escolha o arquivo com os testes.
3.  Clique em **Executar mutador** e aguarde o fim da execução.
4.  Consulte os resultados na própria janela e o arquivo
    `relatorio.json`, gerado na pasta do código selecionado.

O comando externo `lua` precisa estar disponível no `PATH`.

## 10. Estrutura do projeto

A estrutura correspondente aos arquivos apresentados é:

``` text
Projeto-de-Mutador-para-Lua/
├── src/
│   ├── main.rs       # Interface gráfica e coordenação do fluxo
│   ├── parser.rs     # Parsing Lua e geração da AST
│   ├── scanner.rs    # Visitação da AST e identificação dos operadores
│   └── mutador.rs    # Geração, execução e registro dos mutantes
├── codigo.lua        # Exemplo de código Lua submetido à mutação
├── suit.lua          # Exemplo de suíte de testes
├── Cargo.toml        # Dependências e configuração Rust
└── relatorio.json    # Relatório das mutações sobreviventes
```

O arquivo `relatorio.json` é gerado durante a execução, na pasta do
código Lua selecionado. A versão em linha de comando, que gerava também
o `AST.json`, foi substituída pela interface gráfica.

## 11. Limitações e próximos passos

Embora a identificação dos operadores já utilize a AST, há pontos que
podem ser aprimorados:

-   **Delimitação da região de mutação:** integrar os marcadores
    `--start_mut();` e `--end_mut();` à análise da AST, para restringir
    a visita aos nós localizados na região selecionada.
-   **Validação da execução:** verificar o status de saída do processo
    Lua.
-   **Tempo limite:** interromper execuções que excedam um tempo máximo,
    evitando travamentos causados por laços infinitos em mutantes.
-   **Validação da suíte:** conferir se a quantidade de testes
    executados corresponde à quantidade esperada, e se a suíte passa no
    código original antes da mutação.
-   **Cobertura de mutadores:** definir substituições para os operadores
    aritméticos já identificados pelo scanner e ampliar as regras para
    outras construções da linguagem.
-   **Tratamento de erros:** tornar mais robustos os casos de falhas de
    parsing e problemas ao executar o interpretador, evitando
    interrupções abruptas (`panic`).
-   **Isolamento das execuções:** evitar que a escrita temporária no
    arquivo de código cause interferência entre execuções simultâneas ou
    deixe o arquivo alterado caso ocorra uma falha inesperada (por
    exemplo, executando os mutantes sobre uma cópia).
-   **Interface:** exibir também os mutantes mortos e com erro, e
    permitir exportar o relatório pela janela.

## 12. Objetivo acadêmico

O projeto explora a aplicação de **teste de mutação** como técnica de
avaliação de suítes de testes de software.

A utilização da AST permite identificar operadores a partir da estrutura
sintática do programa, tornando a seleção dos pontos de mutação mais
precisa do que uma busca textual simples. A geração de versões mutantes
e a execução automatizada dos testes permitem observar se as alterações
são detectadas pela suíte e identificar possíveis lacunas na cobertura
dos testes.