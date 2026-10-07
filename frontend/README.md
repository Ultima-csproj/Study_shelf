# Учебная полка — frontend

React/TypeScript/Vite, React Router, CSS Modules. Каталог, дисциплины и публичные подборки читают PostgreSQL через Axum. Палитра и существующие компоненты переиспользованы.

## Запуск

1. Из корня: docker compose up -d postgres.
2. Из backend: cargo run --bin backend.
3. Из frontend:

~~~powershell
npm install
npm run dev
~~~

Vite: http://127.0.0.1:5173. /api proxy → http://127.0.0.1:3000. Production тоже должен маршрутизировать /api к backend.

Текущее dev-окружение использует study_dev_stage11, корневой .env уже настроен. Первичный seed загружен. Конфигурация/команды: ../backend/README.md.

## Маршруты

| Путь | Содержание |
| --- | --- |
| /catalog | Публичные материалы, поиск, filters/sort/page |
| /disciplines | Список дисциплин с pagination |
| /disciplines/{id} | Учреждение, описание, разделы и учебные контексты |
| /collections | Поиск/filter/sort/pagination подборок |
| /collections/{id} | Цель, аудитория, составители и упорядоченный маршрут |
| /materials/{id} | Текущая опубликованная редакция |
| /login | Демонстрационный вход |
| /profile, /library, /settings | Локальная личная зона до auth |

Дисциплины/подборки открываются по ID. Slug дисциплины уникален внутри учреждения и не служит глобальным идентификатором маршрута.

## Работа с данными

- api/http.ts: GET JSON, AbortSignal, HTTP/network errors.
- api/catalog.ts: текущий публичный каталог.
- api/materials.ts: текущая публичная MaterialDetail, 404 → null; ошибки API не заменяются mock.
- api/disciplines.ts: список/detail/страницы пунктов раздела.
- api/collections.ts: список/detail/страницы маршрута.
- api/learning.ts: один Map уникальных summaries и преобразование entries в представление.
- hooks/useSectionItems.ts: продолжение раздела, abort, loading/error/retry, устранение повторного entry ID.
- types/learning.ts: shared normalized transport; discipline/collection/material — отдельные типы.

Detail содержит первые 10 пунктов каждого непустого раздела. «Показать ещё» загружает следующую страницу. Materials[] нормализован: одна текущая публикация на Material ID, даже если в маршруте несколько ссылок на неё.

## Поведение

- Материал в нескольких курсах/у разных ответственных представлен отдельными placement IDs.
- Авторство из credits показывается отдельно от ответственного учебного участника.
- У всей дисциплины нет выдуманного обязательного teacher/course/semester.
- Один Material может повторяться в разных разделах подборки.
- Счётчики показывают шаги и уникальные доступные материалы.
- Private/unpublished материалы исключаются сервером; пустые публичные sections скрываются.
- Нумерация строится по видимым пунктам; internal position сохраняет порядок.
- Summary материала использует текущую Published revision, без pinning/history.
- Все активные учебные ссылки используют api-источник.
- Opening context в from и section hash сохраняет возврат к каталогу/дисциплине/подборке/библиотеке.
- Ошибка API остаётся ошибкой, есть повтор загрузки.

## Аккаунты и личная зона — этап 12

ViewerProvider получает /api/viewer: Loading/Guest/Authenticated/Error. Email/password формы работают с PostgreSQL sessions в HttpOnly cookie. SessionStorage не используется для авторизации или сохранений.

Профиль и библиотека серверные; библиотека постраничная, недоступное сохранение остаётся нейтральным пунктом. Статусы сохранений кнопок читаются пакетами. Участие в подборках определяется подтверждёнными Contributor linkage. Выход не удаляет сохранения аккаунта.

Личные подборки: /library/collections и /library/collections/:id. Создание Private, сведения, разделы/пункты и порядок с editVersion. Материалы добавляются по ID из URL карточки; улучшение выбора материала можно сделать позже. Public collection writes пока запрещены.

/settings: email, ID аккаунта для bootstrap CLI, смена пароля/выход на остальных устройствах, заявки membership/linking. Подтверждение email/reset через почту остаются отдельным этапом. После обновления backend перезапустить его и обновить frontend.

Старые data-массивы сохранены в archive/frontend-learning-demo и не входят в активную сборку. Старые source=mock ссылки возвращают отсутствие материала; совпадающий API ID не подставляется.

## Проверки

~~~powershell
npm run build
npm run lint
~~~

6 октября: TypeScript/Vite build и lint успешны. Основной JS chunk около 478 КБ, прежнего предупреждения >500 КБ нет.

Браузерные/автоматические тесты нового этапа не выполнялись. Предыдущие визуальные проверки сохранены в design-qa.md и относятся к прежним этапам.

## Дальше

Следующий этап 13: административная UI-панель. Затем черновики/модерация, история опубликованных редакций, proposals и editor. Панель управления в этом срезе не добавлялась.

Подробности локально: .codex/docs/backend-learning-stage.md и backend-plan.md.
