# Cabrital

Uma simulação de rebanho de cabritos

## Características

  * simula a composição de um rebanho através de um determinado número de meses
  * divide os indivíduos em dois grupos: cabritos, que vão para o abate após um certo número de meses, e matrizes, que podem estar prenhas ou amamentando
  * todos os parâmetros são configuráveis

## Configurações
|   Parâmetro                 |  Descrição                                                                                                                                                                                  |
|-----------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    `filhos_por_100_partos`  |      A quantidade de cabritos por 100 partos. Ex: se 10 cabritos a cada sem são natimortos, esse número deverá der de 90                                                                    |
|   `init_matrizes`           |   Quantidade inicial de matrizes, em números absolutos                                                                                                                                      |
|    `init_cabritos`          | Quantidade inicial de cabritos, em números absolutos                                                                                                                                        |
|   `init_matrizes_idade`     |     **OPCIONAL**; idade das matrizes iniciais. Omitir essa informação fará com que suas idade seja inicializada aleatoriamente                                                              |
|   `init_cabritos_idade`     | **OPCIONAL**; idade dos cabritos iniciais. Omitir essa informação fará com que suas idade seja inicializada aleatoriamente                                                                  |
|   `tempo_prenhez_meses`     | Tempo de prenhez, em meses                                                                                                                                                                  |
|   `tempo_amamentando_meses` |   Período de amamentação, em meses                                                                                                                                                          |
|     `idade_abate_cabrito`   | Idade com a qual cabritos vão para o abate, em meses. Machos são imediatamente descontados do rebanho, enquanto fêmeas, caso haja carência de matrizes, podem ir para o rebanho de matrizes |
|   `tempo_vida_matriz`       | Tempo de vida de uma matriz, em meses                                                                                                                                                       |
|   `teto_matriz`             |    Número máximo de matrizes. Equivalente à capacidade de suporte do meio                                                                                                                   |
|    `rt_meses`               | Tempo para ser simulado                                                                                                                                                                     |

### Exemplo de configuração

Consulte o arquivo [config.toml](https://github.com/franpessoa/cabrital/blob/main/config.toml)

A configuração é no formato [TOML](https://pt.wikipedia.org/wiki/TOML). Tudo que está em uma linha após um caractere `#` é um comentário, e não será considerado.

## Uso
### Ferramentas necessárias
A simulação é programada na linguagem Rust. Logo, é necessário ter um compilador disponível, e o acesso à ferramenta Cargo.
Para instalar-se, é recomendado o uso da interface [rustup](https://rustup.rs/)

Também é necessária a disponibilidade do [git](https://git-scm.com/)

### Compilação
```bash
# Clone o repositório
git clone https://github.com/franpessoa/cabrital.git cabrital
cd cabrital

# Compile
cargo build --release

# Rode
cargo run -- <opções de uso>
```

As opções de linha de comando são as seguintes:
```
  -c, --cfg <CFG>  [default: ./config.toml] Arquivo de configuração
  -o, --out <OUT>  Arquivo de saída
  -h, --help       Mensagem de ajuda
```
Case opte-se por rodar usando `cargo run`, é obrigatório os duplos hífens antes das opçẽs.

### Saída
A saída é no formato CSV, um tipo rudimentar de planilha. Geralmente pode ser aberto com qualquer editor de planilhas



