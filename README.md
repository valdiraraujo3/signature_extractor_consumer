Signature Extractor Consumer 🤖

Este projeto é um serviço que consome mensagens de forma assíncrona publicadas no stream signature_extractor. 
Ele utiliza a arquitetura hexagonal para separar as preocupações e garantir que a lógica de negócio seja independente de tecnologias externas. 
As mensagens processadas são salvas em um banco de dados MongoDB.

Funcionalidades ✨
* Consumo Assíncrono de Mensagens: Processa eventos publicados no signature_extractor utilizando async_nats::jetstream.
* API REST: Fornece endpoints para verificar a saúde do serviço e recuperar relatórios salvos no banco de dados.
* Armazenamento de Dados: Salva os eventos processados em um banco de dados MongoDB.

Tecnologias 🛠️
- Linguagem: Rust
- Arquitetura: Hexagonal
- Streaming de Mensagens: async-nats (JetStream)
- Framework Web: axum

Banco de Dados: MongoDB

Dependências: tokio, serde, anyhow, uuid, entre outras.

Estrutura do Projeto 📂
A estrutura do projeto segue a arquitetura hexagonal, organizando o código em camadas que isolam a lógica de negócio das implementações externas.

<img width="1987" height="310" alt="image" src="https://github.com/user-attachments/assets/d79db95b-c84b-4c14-8d9d-3c88baf2db3b" />

Instalação e Execução 🚀

Este projeto utiliza Docker Compose para gerenciar os serviços de NATS e MongoDB, facilitando a configuração do ambiente.

Pré-requisitos
Docker e Docker Compose instalados.

Rust e Cargo.

Passos
Clone o repositório:

Bash

git clone https://github.com/seu-usuario/signature_extractor_consumer.git

cd signature_extractor_consumer

Inicie os serviços com Docker Compose:

docker-compose.yml
<img width="1730" height="902" alt="image" src="https://github.com/user-attachments/assets/c8487fb8-8bb9-41d9-846c-fe9992f9485f" />

Bash

docker-compose up -d

Isso irá subir os containers para o MongoDB, NATS e NUI (uma interface web para o NATS).

Execute o projeto em Rust:

Bash

cargo run

O servidor NATS estará disponível em nats://localhost:4222. 
A API REST será executada em http://localhost:3000.

Endpoints da API 🌐

GET /health: Retorna o status de saúde do serviço.

GET /reports: Retorna todos os relatórios processados e salvos no MongoDB.
