# Como publicar uma nova versao do Norton Eventos

## Setup uma unica vez (apos instalar Rust + Tauri CLI)

1. Gerar par de chaves de assinatura:
   ```powershell
   cargo tauri signer generate -w "$env:USERPROFILE\.tauri\norton-eventos.key"
   ```
   - A chave **privada** vai em `~/.tauri/norton-eventos.key` — NUNCA compartilhar, NUNCA comitar.
   - A chave **publica** aparece no terminal. Copiar e colar em `src-tauri/tauri.conf.json` no campo `plugins.updater.pubkey`.

2. Salvar a senha da chave privada em variavel de ambiente (a CLI pede ela em todo build):
   ```powershell
   [Environment]::SetEnvironmentVariable("TAURI_SIGNING_PRIVATE_KEY", (Get-Content "$env:USERPROFILE\.tauri\norton-eventos.key" -Raw), "User")
   [Environment]::SetEnvironmentVariable("TAURI_SIGNING_PRIVATE_KEY_PASSWORD", "SUA_SENHA_AQUI", "User")
   ```
   Fechar e reabrir o terminal depois.

3. Criar o repositorio `eblxd/muoverlay` no GitHub (publico).

## Cada release nova

1. Editar `src-tauri/tauri.conf.json` e `src-tauri/Cargo.toml`: bumpar `version` (ex: `1.0.0` -> `1.0.1`).

2. Buildar:
   ```powershell
   cd c:\Users\Windows\Desktop\NortonEvent\tauri-app\src-tauri
   cargo tauri build
   ```

3. Saida em `target/release/bundle/nsis/`:
   - `Norton Eventos_1.0.1_x64-setup.exe` (o instalador)
   - `Norton Eventos_1.0.1_x64-setup.exe.sig` (assinatura)

4. Criar `latest.json` na raiz do projeto:
   ```json
   {
     "version": "1.0.1",
     "notes": "Descricao do que mudou",
     "pub_date": "2026-05-18T12:00:00Z",
     "platforms": {
       "windows-x86_64": {
         "signature": "COLAR_CONTEUDO_DO_ARQUIVO_.sig_AQUI",
         "url": "https://github.com/eblxd/muoverlay/releases/download/v1.0.1/Norton.Eventos_1.0.1_x64-setup.exe"
       }
     }
   }
   ```

5. No GitHub: criar release com tag `v1.0.1`, anexar o `.exe`, o `.sig` e o `latest.json`.

6. Pronto. Quem ja tem o app, na proxima vez que abrir, baixa a atualizacao em background e na abertura seguinte ja esta na nova versao.

## Distribuicao da primeira versao

O usuario final baixa `Norton Eventos_1.0.0_x64-setup.exe` do GitHub Releases, executa, e o app instala em `%LOCALAPPDATA%\Norton Eventos\`. A partir dai, atualizacoes futuras chegam sozinhas.
