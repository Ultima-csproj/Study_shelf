# Локальный backend

Запуск из папки `backend`:

```powershell
cargo run
```

Сервер слушает `http://127.0.0.1:3000`, открывает `backend/data/study.sqlite3` и применяет миграции. Данные автоматически не добавляются.

## Данные и просмотр

```powershell
cargo run --bin seed
cargo run --bin catalog_preview -- транзакции
```

## Настройки

Все три команды используют одинаковую конфигурацию. Переменные задаются в PowerShell перед запуском:

```powershell
$env:BIND_ADDRESS = "127.0.0.1:3000"
$env:DATABASE_PATH = "data/study.sqlite3"
$env:RUST_LOG = "backend=info,tower_http=debug"
cargo run
```

- `BIND_ADDRESS`: адрес TCP listener, по умолчанию `127.0.0.1:3000`.
- `DATABASE_PATH`: абсолютный путь или путь относительно папки `backend`; по умолчанию `data/study.sqlite3`.
- `RUST_LOG`: фильтр логирования сервера.

`.env` автоматически не загружается. Относительный путь базы не зависит от папки запуска команды.

## HTTP

- `GET /api/health`: проверка ответа HTTP-процесса.
- `GET /api/catalog`: каталог; параметры `q`, повторяемые `type`, `discipline`, `teacher`, `course`, `year`, `tag`, одиночные `sort`, `page`, `pageSize`.
- `GET /api/materials/{id}`: материал со связанными преподавателем и дисциплиной.

Ошибки имеют форму `{ "error": { "code": "...", "message": "..." } }`. Некорректный запрос возвращает `400`, отсутствующий материал — `404`, неподдерживаемый метод — `405`. Недоступность хранилища возвращает `503` с кодом `service_unavailable`; повреждённые данные и неожиданные внутренние ошибки — `500` с кодом `internal_error`. Подробная причина сохраняется в серверном логе.

```powershell
Invoke-RestMethod "http://127.0.0.1:3000/api/catalog?type=book&type=video&pageSize=5"
Invoke-RestMethod "http://127.0.0.1:3000/api/materials/1001"
```

Vite запускается из `frontend` командой `npm run dev` на `http://127.0.0.1:5173` и проксирует `/api` на backend. При изменении порта backend нужно согласованно изменить `target` в `frontend/vite.config.ts`.

Каталог и страницы материалов каталога используют `fetch` через этот proxy. Для проверки откройте `http://127.0.0.1:5173/catalog?all=1`: текущий seed содержит 420 материалов. Поиск, фильтры, сортировка и страницы выполняются в SQLite.

Дисциплины и подборки пока используют демонстрационные данные. Их ссылки содержат `source=mock`, чтобы совпадающие ID не открывали другой материал из SQLite. Локальные сохранения учитывают ID и источник; серверная авторизация и сохранение ещё не реализованы.

## Проверки

```powershell
cargo check --workspace
cargo test --workspace
cargo fmt --all --check
```

HTTP-тесты применяют реальные миграции к отдельной SQLite в памяти и не изменяют локальную базу.
