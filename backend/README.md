# Локальный Rust/Axum backend — PostgreSQL

Workspace: backend, study-core, study-storage-postgres. PostgreSQL 18 запускается Compose, Rust/Vite работают локально.

## Текущее окружение

PostgreSQL слушает 127.0.0.1:5433. Ignored корневой .env задаёт DATABASE_URL и POSTGRES_PASSWORD.

Для этапа 11 создана dev-база study_dev_stage11: в старой study_dev checksum изменённой 0001_identities.sql не совпадал с сохранённым. study_dev сохранена. Корневой .env уже переключён; не заменяйте его примером с другим паролем.

Шесть миграций и первичный seed применены к study_dev_stage11. SQLite-адаптер находится в archive/backend-sqlite и не входит в сборку.

## Запуск

Из корня:

~~~powershell
docker compose up -d postgres
docker compose ps
~~~

Из backend:

~~~powershell
cargo run --bin seed
cargo run --bin backend
~~~

При обычном старте применяются schema migrations, seed автоматически не запускается. .env загружается из текущей папки или родительской; переменные окружения имеют приоритет.

Vite: из frontend выполните npm run dev. Страницы /catalog, /disciplines и /collections читают API. Proxy /api направлен на 127.0.0.1:3000. После изменения кода перезапустите уже работающий backend.

## Новое чистое окружение

Compose создаёт study_dev. Для новой установки задайте собственный пароль в корневом .env:

~~~dotenv
POSTGRES_PASSWORD=PLACEHOLDER
DATABASE_URL=postgres://study_dev:PLACEHOLDER@127.0.0.1:5433/study_dev
~~~

Специальные символы пароля в URL кодируются URL-encoding. Дополнительные настройки — backend/.env.example. В текущем окружении используется уже созданная study_dev_stage11.

Volume study_postgres сохраняет обе базы. Остановка docker compose stop не удаляет данные.

## Dev-команды

~~~powershell
cargo run --bin seed
cargo run --bin catalog_preview -- "транзакции"
cargo run --bin learning_preview
cargo run --bin rebuild_search
~~~

Seed создаёт 840 материалов, 3 учреждения, 8 пользователей, 11 contributors, 7 дисциплин, 21 раздел дисциплин и 48 подборок с 144 разделами. Публичны 729 материалов и 44 подборки. Учебные fixtures этапа 11 используют материалы без video.

Существующие fixture keys пропускаются. Перезапуск не возвращает вручную изменённое содержание и права к исходному seed. Повторный seed в этом запросе не выполнялся.

learning_preview показывает публичные страницы, разделы, два счётчика подборок и следующую страницу пунктов. rebuild_search — отдельная maintenance-команда, блокирующая записи materials на время пересборки.

## Настройки

| Переменная | Значение |
| --- | --- |
| DATABASE_URL | Обязательная PostgreSQL URL |
| BIND_ADDRESS | Default 127.0.0.1:3000 |
| DATABASE_MAX_CONNECTIONS | Default 10, 1..100 |
| DATABASE_ACQUIRE_TIMEOUT_SECONDS | Default 5, 1..60 |
| RUST_LOG | Фильтр логирования |
| AUTH_COOKIE_SECURE | false для loopback HTTP, true для HTTPS |
| AUTH_SESSION_SECONDS | 604800 по умолчанию, 300..2592000 |
| AUTH_ALLOWED_ORIGINS | Точные Origin через запятую; local defaults localhost/127.0.0.1:5173 и :3000 |

UTC, statement_timeout=15s, lock_timeout=5s. SQL statements/credentials не выводятся клиенту. DATABASE_PATH больше не используется.

Write Material wrappers повторяют всю транзакцию только для 40001/40P01: максимум три попытки, 25/50 мс. Business conflicts, сеть и неизвестный COMMIT не повторяются.

## HTTP

- GET /api/health
- GET /api/catalog
- GET /api/materials/{id}
- GET /api/disciplines
- GET /api/disciplines/{id}
- GET /api/disciplines/{id}/sections/{section}/items
- GET /api/collections
- GET /api/collections/{id}
- GET /api/collections/{id}/sections/{section}/items

Health подтверждает ответ процесса, не отдельную DB readiness.

Списки: page/pageSize (1..100); empty pageCount=0. Дисциплины: q, institution, sort=title. Подборки: q, повторяемые discipline/course, institution владельца, newest/oldest/title. Section items принимают только page/pageSize.

Detail содержит summary, sections с первыми 10 пунктами и уникальный materials[]; пункты ссылаются на Material IDs. Collection counts: stepCount и uniqueMaterialCount. Пустые публичные разделы скрываются, связи в БД сохраняются.

400 — invalid query, 404 — нет публичной сущности/раздела, 405 — метод, 503 — unavailable/retryable storage, 500 — invalid data/unexpected. JSON: { "error": { "code": "...", "message": "..." } }.

Auth/server library и авторский Material API/UI реализованы в этапах 12–14. Proposals и assets/editor ещё впереди.

## Проверки сборки

~~~powershell
cargo check --workspace
cargo clippy --workspace --bins --lib -- -D warnings
cargo fmt --all --check
~~~

Автотесты, concurrency/repeat seed и browser flow этапа 11 не запускались. Старые SQLite тесты сохранены в архиве и не проверяют текущую PostgreSQL-модель.

## Документы

Локальная документация: .codex/docs/backend-learning-stage.md — этап 11 и семантика reader; backend-function-semantics.md — Material/storage; backend-postgres-contract.md — каталог; backend-plan.md — roadmap.

## Аккаунты и первоначальные права — этап 12

После обновления перезапустить backend. При открытии хранилища автоматически применяются migrations 0007/0008. Можно отдельно выполнить:

~~~powershell
cargo run --bin study-admin -- migrate
~~~

Регистрация — /login?mode=register на frontend. У seed User нет настоящих credentials; создайте собственный аккаунт. Его ID показывается в /settings.

~~~powershell
# 123 заменить ID своего зарегистрированного аккаунта.
cargo run --bin study-admin -- grant-platform-admin --user-id 123
cargo run --bin study-admin -- grant-institution-admin --user-id 123 --institution-id 1
~~~

После назначения обновить страницу. CLI также поддерживает revoke-platform-admin, revoke-institution-admin, accept-membership/reject-membership (--role student|teacher), accept-contributor/reject-contributor и cleanup-sessions. Выполнить --help для аргументов. После этапа 13 назначения/решения используют общий Administration service и administration_audit; старый identity_audit перенесён в историю.

Основные endpoints: GET /api/auth/csrf, /api/viewer; POST /api/auth/register/login/logout/logout-others; GET/PATCH /api/me/profile; POST /api/me/password; GET /api/me/library/materials/collections/contributions; PUT/DELETE /api/me/library/materials/{id}/collections/{id}; GET/POST /api/me/collections; GET/PATCH /api/me/collections/{id}; GET /api/me/collections/{id}/sections/{section}/items. Каждый перечисленный суффикс — отдельный route, полный контракт в локальном документе ниже.

Все unsafe запросы требуют разрешённый Origin, matching CSRF cookie/X-CSRF-Token; authenticated mutation также проверяет связанную с сессией digest. Клиент работает через Vite /api proxy. POST /api/me/library/flags — пакетный запрос максимум 100 ID каждого вида.

Пароли Argon2id, максимум две blocking CPU-задачи; PostgreSQL хранит digest сессии, expiry/revocation и password_version. Profile DTO не принимает state/permissions/чужого owner. Private material reader возвращает только текущую Published revision согласно owner/membership/collaborator policy. Auth/me responses private/no-store.

Email verification/mail reset, Organization representation, Public collection publication и files/editor ещё не реализованы. Material drafts UI подключён в этапе 14. Подробная семантика аккаунтов: .codex/docs/backend-auth-implementation.md.

## Административная панель — этап 13

После перезапуска backend `/admin` на frontend доступен согласно permissions/membership. Миграции 0009/0010 добавляют версии административных записей, institution registration requests и administration_audit. Аккаунты, права, отзыв сессий, учреждения, membership/Contributor requests и журнал читаются через `/api/admin/*`. Подача/собственные заявки учреждения: `/api/me/institution-requests`.

HTTP mutations требуют прежнюю session/CSRF проверку, причину и expectedVersion для изменяемых сущностей. ManageAccounts и ViewAccountPrivateData разделены; последнее нужно для email и Contributor evidence. Последнего активного platform administrator/institution manager нельзя отключить или лишить управляющего права; CLI использует тот же guard. Назначить замену до отзыва.

Admin responses private/no-store. UI произвольного SQL не выполняет. Material lifecycle/модерация подключены основным срезом этапа 14; общего staff bypass Private материалов нет. Реализация и ограничения: `.codex/docs/backend-admin-implementation.md`, `.codex/docs/backend-material-implementation.md`.

## Материалы и модерация

Migration 0011 добавляет независимые lifecycle/publication, restrictions, материаловый audit и один рабочий Draft/Pending. Авторские routes: `/api/me/author-materials`, `/api/me/materials/{id}/workspace`, drafts/revisions/transition/state/collaborators/events. Moderation queue: `/api/admin/materials/pending`; ограничения: `/api/admin/materials/restricted` и restriction-target. Published history: `/api/materials/{id}/history`, signed counterpart `/api/me/materials/{id}/history`.

Owner/invited Private access и конкретный Pending для модератора проверяются отдельно. Approve атомарно публикует новую редакцию; старые Published immutable. Archive исключает обычные списки, сохраняя прямое чтение; Unpublish сохраняет pointer; Restore/Republish не снимают restrictions. Context discipline/collection/restriction выводит учреждение на сервере.

SQL нового workflow многострочный, небольшие файлы в `material/workspace/sql`. Семантика функций и ручные сценарии: `.codex/docs/backend-material-implementation.md`, `.codex/docs/material-checklist.md`. Персональные hide material/author остаются срезом 14.7; далее proposals и files/editor. Migrate/build не подтверждают сценарии прав и конкуренции.

Применённые SQLx migrations не изменять даже ради форматирования: checksum зависит от байтов файла. При обнаружении старого рассогласования не отключать проверку и не подменять checksum без подтверждения точного исторического текста и отсутствия семантических изменений.

Для SQL migrations в `.gitattributes` закреплён LF, чтобы Git на Windows не менял checksum преобразованием строк при следующем checkout.
