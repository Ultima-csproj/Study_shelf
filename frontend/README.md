# Учебная полка — frontend

React/TypeScript/Vite, React Router, CSS Modules. Каталог, дисциплины и публичные подборки читают PostgreSQL через Axum. Палитра и существующие компоненты переиспользованы.

## Запуск

Полный порядок запуска PostgreSQL, backend и frontend, настройка `.env` и устранение частых ошибок описаны в [корневом README](../README.md).

Если backend уже запущен, откройте PowerShell в каталоге `frontend`. При первой установке или после изменения `package-lock.json` выполните:

~~~powershell
npm.cmd ci
~~~

Для обычного запуска frontend:

~~~powershell
npm.cmd run dev
~~~

Vite: http://127.0.0.1:5173. `/api` proxy → `http://127.0.0.1:3000`. Production тоже должен маршрутизировать `/api` к backend.

## Маршруты

| Путь | Содержание |
| --- | --- |
| /catalog | Публичные материалы, поиск, filters/sort/page |
| /disciplines | Список дисциплин с pagination |
| /disciplines/{id} | Учреждение, описание, разделы и учебные контексты |
| /collections | Поиск/filter/sort/pagination подборок |
| /collections/{id} | Цель, аудитория, составители и упорядоченный маршрут |
| /materials/{id} | Текущая опубликованная редакция |
| /login | Серверная регистрация и вход |
| /profile, /library, /settings | Авторизованная личная зона |
| /admin | Панель управления согласно capabilities |

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
- Summary материала использует текущую Published revision. История Published подключена отдельными страницами; pinning пока отсутствует.
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

7 октября, этап 13: TypeScript/Vite build и lint успешны. Административные страницы и форма регистрации учреждения загружаются лениво; основной JS chunk около 489 КБ, предупреждения >500 КБ нет.

Браузерные/автоматические тесты нового этапа не выполнялись. Предыдущие визуальные проверки сохранены в design-qa.md и относятся к прежним этапам.

## Дальше

Этап 13 реализован: аккаунты/права, учреждения/участники, регистрационные и авторские заявки, причины решений и журнал. Ссылка «Администрирование» есть в меню аккаунта и мобильном меню при нужных полномочиях. Управление существующим учреждением требует scoped ManageMembers; платформа может назначить первого управляющего при отсутствии действующих.

Основной срез 14 подключён: `/author/materials`, `/author/materials/new`, `/author/materials/:id`, `/admin/materials/pending`, `/admin/materials/restricted`, `/materials/:materialId/history` и detail конкретной Published. В меню есть «Мои публикации». Формы, редактор, права приглашённых, состояние, ограничения, учебные отсоединения и журнал разделены на компоненты в `components/materialWorkspace`.

Новый Draft не меняет опубликованный текст. Pending редактируется после Withdraw; Rejected/Discarded копируется в новый Draft. Все изменения имеют reason/expectedVersion и окончательную проверку на сервере. Состояния Material и restrictions показаны отдельно; каталог поддерживает includeArchived.

Файловой загрузки, встроенного редактора и видео-примера нет. Персональные hide material/author остаются отдельным срезом 14.7; далее proposals/editor. Ручные сценарии: `.codex/docs/material-checklist.md`. Lint/build не заменяют проверку прав, browser flows и concurrency.

Подробности локально: .codex/docs/backend-learning-stage.md и backend-plan.md.


### Проверка контекстной навигации

В каталоге frontend выполните `npm.cmd run test:navigation`. Тесты запускают реальные TypeScript утилиты навигации без дополнительного тестового фреймворка: исходные вкладки/фильтры/страницы, вложенные возвраты, создание/отмена, безопасные fallback и переходы меню. Внешний вид и реальные операции с API эта команда не проверяет.
