# Rust-Text-Editor

<p align="center">
If you want to read this file in other language, click on the respective flag <br> <br>
  <a href="https://github.com/Paferreira982/rust-text-editor/blob/main/README.pt-br.md">
    <img src="https://img.shields.io/badge/lang-pt--br-green.svg" alt="pt-br">
  </a>
  <a href="https://github.com/Paferreira982/rust-text-editor/blob/main/README.md">
    <img src="https://img.shields.io/badge/lang-en-red.svg" alt="en">
  </a>
</p>

## Descrição
O *Rust-Text-Editor* é um poderoso e flexível editor de texto desenvolvido em Rust, projetado para oferecer uma experiência de edição eficiente e segura. O *Rust-Text-Editor* combina a eficiência do Rust com a familiaridade do modo de comando para criar um ambiente de edição robusto e confiável.

### Porque um Editor de Texto?
Um editor de texto é uma ferramenta essencial para qualquer desenvolvedor. Como utilizo ferramentas do tipo diariamente, decidi desenvolver meu próprio editor de texto para aprender e aplicar conceitos de baixo nível, e principalmente, entender como um editor de texto funciona, como todas as suas funcionalidades são implementadas e como elas se integram para criar uma experiência de edição eficiente e confiável.

### Porque Rust?
Rust é uma linguagem de programação de sistemas que prioriza a segurança e desempenho. O principal motivo da minha escolha por Rust é o aprendizado em linguagens de baixo nível. Rust é uma linguagem de programação de sistemas que oferece um equilíbrio perfeito entre segurança e desempenho, tornando-a uma linguagem ideal para aprender e aplicar conceitos de baixo nível.

Além disso, Rust possui algumas características que me chamaram a atenção:

>**Segurança de Memória** <br>
>  O sistema de propriedade de Rust evita erros comuns de alocação de memória, proporcionando um código mais seguro e robusto.

>**Concorrência sem Erros** <br>
>  Rust facilita a programação concorrente sem a preocupação de condições de corrida, garantindo que o Rust-Text-Editor seja confiável em ambientes multi-thread.

>**Desempenho** <br>
>  A proximidade com o hardware e a ausência de "garbage collection" permitem que o Rust-Text-Editor alcance desempenho excepcional.

Apesar de não utilizar todas as características apresentadas devido ao escopo do projeto, Rust me parecia ser a escolha perfeita para desenvolver um editor de texto eficiente e confiável.

---
## Funcionalidades
[x] **Visualização de Texto:** <small><i>Capacidade de especificar um caminho de um arquivo de texto para visualização de seu conteúdo.</i></small>

[x] **Navegação:** <small><i>Capacidade de scrollar o conteúdo do arquivo de texto.</i></small>

[X] **Barra de Status:** <small><i>Capacidade de exibir informações sobre o arquivo de texto atual, como o nome do arquivo, o número da linha atual, número total de linhas e o tipo do arquivo.</i></small>

[X] **Prompt e Mensagens:** <small><i>Capacidade de exibir mensagens e capturar comandos do usuário via prompt.</i></small>

[x] **Edição de Texto:** <small><i>Capacidade de editar o conteúdo de um arquivo de texto, editando linhas já existentes ou adicionando novas linhas.</i></small>

[x] **Inicialização:** <small><i>Capacidade de inicializar o editor de texto sem especificar um arquivo de texto.</i></small>

[x] **Salvar Arquivo:** <small><i>Capacidade de salvar o conteúdo de um arquivo de texto em seu caminho original.</i></small>

[x] **Salvar Como:** <small><i>Capacidade de salvar o conteúdo de um arquivo de texto em um novo caminho.</i></small>

[X] **Detectar Alterações:** <small><i>Capacidade de detectar se o conteúdo de um arquivo de texto foi alterado desde a última vez que foi salvo.</i></small>

[x] **Confirmar Sair Sem Salvar:** <small><i>Capacidade de confirmar a saída do editor de texto sem salvar o conteúdo do arquivo de texto.</i></small>

[x] **Localizar Texto:** <small><i>Capacidade de localizar uma string no conteúdo de um arquivo de texto.</i></small>

[x] **Navegar no modo Localizar Texto:** <small><i>Capacidade de navegar entre as ocorrências de uma string no conteúdo de um arquivo de texto, tanto para frente quanto para trás.</i></small>

[x] **Highlighting de Texto:** <small><i>Capacidade de colorir o texto de acordo com a sintaxe do arquivo de texto.</i></small>

[x] **Suporte para Rust:** <small><i>Capacidade de colorir o texto de acordo com a sintaxe da linguagem Rust.</i></small>

[ ] **Auto Indentação:** <small><i>Capacidade de indentar automaticamente o texto de acordo com a sintaxe do arquivo de texto.</i></small>

[ ] **Copiar e Colar:** <small><i>Capacidade de copiar e colar texto com comandos Ctrl+C e Ctrl+V.</i></small>

[ ] **Suporte para a Typescript/Javascript:** <small><i>Capacidade de colorir o texto de acordo com a sintaxe da linguagem Typescript.</i></small>

[ ] **Suporte para a Python:** <small><i>Capacidade de colorir o texto de acordo com a sintaxe da linguagem Python.</i></small>

---

## Instalação
Para executar o projeto, é necessário ter o Rust instalado em sua máquina. Para instalar o Rust, siga as instruções do site oficial: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Preparar o ambiente
```bash
git clone https://github.com/Paferreira982/rust-text-editor.git

cd rust-text-editor
```

### Inicializar sem especificar um arquivo de texto
```bash
cargo run
```

### Inicializar a edição de um arquivo existente
```bash
cargo run <caminho-do-arquivo>
```

## Créditos
Este editor de texto foi baseado em um tutorial do [Build Your Own Text Editor in Rust](https://www.flenker.blog/hecto/) desenvolvido por [pflenker](https://github.com/pflenker).