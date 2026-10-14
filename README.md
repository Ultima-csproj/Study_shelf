# Учебная полка

Веб-приложение для поиска учебных материалов и подборок. Frontend работает на React, TypeScript и Vite; backend — на Rust/Axum; данные хранятся в PostgreSQL. При локальной разработке запускаются три части: PostgreSQL в Docker и два процесса в отдельных терминалах — backend и frontend.

## Что установить

- Docker Desktop с работающим Docker Engine.
- Rust stable и Cargo. Backend использует Rust edition 2024.
- Node.js, который поддерживает установленную версию Vite: Node `^20.19.0` или `>=22.12.0`. Подойдёт Node 22.12 или новее.
- PowerShell. В примерах используется `npm.cmd`, чтобы запуск npm не зависел от PowerShell execution policy.

Проверить инструменты можно из PowerShell:

```powershell
docker --version
docker compose version
rustc --version
cargo --version
node --version
npm.cmd --version
```

Все команды ниже запускаются из корня репозитория, если явно не указан другой каталог.

## 1. Подготовить настройки базы

Compose читает корневой `.env`. Для запуска нужны `POSTGRES_PASSWORD` и `DATABASE_URL`. В этом проекте `.env` исключён из Git, поэтому он не попадёт в коммит.

Если корневой `.env` уже настроен, сохраните его значения. Не заменяйте его примером: пароль должен совпадать с паролем, с которым был создан текущий Docker volume, а `DATABASE_URL` должен указывать на нужную базу.

Для совершенно новой локальной установки создайте `.env` в корне репозитория и укажите один и тот же пароль в обеих переменных:

```dotenv
POSTGRES_PASSWORD=choose_a_local_password
DATABASE_URL=postgres://study_dev:choose_a_local_password@127.0.0.1:5433/study_dev
```

Если пароль содержит зарезервированные символы URL, закодируйте их в `DATABASE_URL`. Значение `POSTGRES_PASSWORD` при этом остаётся обычным паролем.

Дополнительные настройки backend перечислены в [`backend/.env.example`](backend/.env.example). Backend загружает `.env` из текущего каталога или родительского; команда запуска ниже выполняется из корня, поэтому используется корневой `.env`.

## 2. Запустить PostgreSQL

В PowerShell из корня репозитория:

```powershell
docker compose up -d postgres
docker compose ps
docker compose exec postgres pg_isready -U study_dev -d study_dev
```

В `docker compose ps` дождитесь состояния `Up (healthy)`. PostgreSQL слушает порт `5432` внутри контейнера и `127.0.0.1:5433` на Windows-хосте. Поэтому `DATABASE_URL` для локального backend использует порт `5433`.

Если Docker Desktop уже открыт, но `docker compose ps` не показывает работающий `postgres`, выполните `docker compose up -d postgres`: запуск Docker Desktop сам по себе не запускает сервис этого проекта.

## 3. Запустить backend

Откройте отдельный PowerShell в корне репозитория и выполните:

```powershell
cargo run --manifest-path backend/Cargo.toml --bin backend
```

Backend слушает `http://127.0.0.1:3000`. При открытии PostgreSQL-хранилища приложение автоматически применяет новые SQLx-миграции. Оставьте это окно открытым, пока работает приложение.

### Демонстрационные данные

На уже подготовленной базе повторно запускать seed для обычного старта не нужно. Для новой пустой базы, если в каталоге нужны демонстрационные материалы, выполните команду один раз из корня репозитория:

```powershell
cargo run --manifest-path backend/Cargo.toml --bin seed
```

Seed использует базу из `DATABASE_URL`; миграции также применяются при открытии хранилища. Seed не запускается автоматически вместе с сервером. Пользователи из демонстрационных данных не имеют паролей для входа — зарегистрируйте собственный аккаунт через frontend.

## 4. Запустить frontend

Откройте ещё один PowerShell. При первой установке зависимостей выполните:

```powershell
Set-Location frontend
npm.cmd ci
```

Затем запустите Vite:

```powershell
npm.cmd run dev
```

В следующие разы достаточно открыть PowerShell в `frontend` и выполнить `npm.cmd run dev`. Vite доступен по адресу [http://127.0.0.1:5173](http://127.0.0.1:5173), а запросы `/api` проксируются на backend `http://127.0.0.1:3000`.

Открывайте приложение по адресу `http://127.0.0.1:5173`. Адрес браузера должен точно совпадать с одним из `AUTH_ALLOWED_ORIGINS`; настройки по умолчанию уже включают этот адрес и `http://localhost:5173`.

## 5. Проверить запуск

Из любого PowerShell:

```powershell
Invoke-RestMethod -Uri 'http://127.0.0.1:3000/api/health'
```

Успешный ответ содержит `status: ok`. Этот endpoint проверяет, что отвечает HTTP-процесс, но сам по себе не проверяет доступ к PostgreSQL. Проверить API через frontend и Vite proxy можно запросом каталога:

```powershell
Invoke-RestMethod -Uri 'http://127.0.0.1:5173/api/catalog?page=1&pageSize=5'
```

Откройте `http://127.0.0.1:5173` и перейдите в каталог. Если новая база пустая, сначала загрузите демонстрационные данные командой seed выше.

## Частые проблемы

| Симптом | Что проверить |
| --- | --- |
| `docker compose ps` не показывает работающий PostgreSQL | Убедитесь, что запущен Docker Engine, затем выполните `docker compose up -d postgres` и дождитесь `healthy`. |
| Backend сообщает, что не может подключиться к PostgreSQL | Проверьте корневой `.env`, адрес `127.0.0.1:5433`, имя базы в `DATABASE_URL` и соответствие пароля запущенному volume. В контейнере порт `5432`, с Windows-хоста — `5433`. |
| `password authentication failed` после изменения `.env` | Переменные `POSTGRES_PASSWORD` применяются при первоначальном создании PostgreSQL volume. Изменение `.env` не меняет пароль уже созданной базы; укажите действующий пароль в `DATABASE_URL`. |
| Vite показывает `ECONNREFUSED 127.0.0.1:3000` | Запустите backend в отдельном PowerShell и дождитесь сообщения о старте HTTP-сервера. После этого обновите страницу. |
| `http://127.0.0.1:3000/api/health` работает, но каталог возвращает ошибку | Проверьте `DATABASE_URL` и логи backend. Health endpoint не проверяет подключение к базе. |
| Порт 3000, 5173 или 5433 занят | Остановите другой процесс, использующий порт. Vite настроен с `strictPort`, а адреса и Origin должны оставаться согласованными. |
| SQLx сообщает о несовпадении checksum миграции | Не редактируйте применённую миграцию и не удаляйте volume. Сохраните текст миграции и разберите ошибку; изменения схемы оформляются новой миграцией. |

Не используйте `docker compose down -v` для обычной остановки: параметр `-v` удаляет volume с базой данных.

## Остановить и возобновить

Остановите frontend и backend сочетанием `Ctrl+C` в их терминалах. PostgreSQL можно остановить, сохранив данные:

```powershell
docker compose stop postgres
```

При следующем запуске повторите `docker compose up -d postgres`, затем запустите backend и frontend в отдельных терминалах.

## Команды для проверки изменений

Backend:

```powershell
cargo check --workspace --manifest-path backend/Cargo.toml
```

Frontend — из каталога `frontend`:

```powershell
npm.cmd run build
npm.cmd run lint
```

Подробности по частям проекта: [`backend/README.md`](backend/README.md) и [`frontend/README.md`](frontend/README.md).
