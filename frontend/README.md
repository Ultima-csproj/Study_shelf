# Учебная полка — frontend

Каталог, дисциплина, профиль и подборки реализованы по утверждённым изображениям. Существующие компоненты, палитра и WebP-превью переиспользованы. Материалы подборок идут в одной вертикальной последовательности. Backend не изменён.

## Запуск

```powershell
cd E:\ProgLanguages\college\practic\frontend
npm install
npm run dev
```

Прямой запуск Vite для PowerShell:

```powershell
node .\node_modules\vite\bin\vite.js --host=127.0.0.1 --port=5173 --strictPort
```

## Маршруты

| Адрес                        | Экран                                                         |
| ---------------------------- | ------------------------------------------------------------- |
| `/`                          | Переход на `/catalog`                                         |
| `/catalog`                   | Поиск, фильтры, сортировка и страницы в URL                   |
| `/disciplines/database`      | Утверждённая дисциплина «Базы данных»                         |
| `/disciplines/:disciplineId` | Поддерживает slug и числовой ID                               |
| `/materials/:materialId`     | Минимальная публичная информация о материале                  |
| `/login?redirect=...`        | Демонстрационный вход с возвратом на исходный URL             |
| `/collections`               | Публичные подборки: поиск, дисциплины, курс, сортировка в URL |
| `/collections/:collectionId` | Учебный маршрут; slug или числовой ID                         |
| `/profile`                   | Свой профиль и редактирование демо-данных                     |
| `/library?tab=saved`         | Сохранённые материалы                                         |
| `/library?tab=collections`   | Сохранённые подборки и подборки с вашим участием              |
| `/settings`                  | Переключение темы; информация о будущем аккаунте              |
| `/references/catalog-d`      | Прежний адрес, перенаправляет в каталог                       |
| Неизвестный адрес            | Простая страница «Страница не найдена»                        |

На production-сервере прямые переходы на frontend-маршруты должны возвращать `dist/index.html`; будущие запросы `/api/*` обрабатываются отдельно.

## Контекстная навигация

Материал открывается на одной общей странице с параметром `from`, сохраняющим источник: каталог, дисциплину, подборку или библиотеку. Возврат сохраняет параметры списка и раздел подборки; контекст восстанавливается после reload и демо-входа. Название дисциплины в подборках фильтрует подборки. Отдельная ссылка «Перейти к дисциплине» на странице материала выполняет явный переход в предмет.

Решения и журнал дальнейшей работы ведутся в [`.codex/docs`](../.codex/docs/README.md). Подробное правило переходов — [контекстная навигация](../.codex/docs/context-navigation.md).

## Что работает

- Общая шапка: навигация, поиск, тема, «Войти» или avatar/dropdown.
- Guest/authenticated через discriminated union `Viewer`.
- Поиск по материалам, описаниям, дисциплинам, преподавателям и тегам.
- Фильтры, сортировка, пагинация. URL сохраняет параметры при reload и входе.
- Дисциплина: sidebar на desktop, селектор на планшете/телефоне, три секции и нумерация 1–5.
- Общий `MaterialPreview`: существующие обложки, thumbnail, центрированный Play и fallback-иконки.
- Guest Save остаётся с замком и отправляет на вход с pathname/query/hash. После входа сохранение переключается локально.
- Native dialog меню/фильтров: Escape, кнопка закрытия, клик по фону, возврат фокуса.
- Две темы через CSS variables, сохранение в localStorage, системная тема при первом открытии.
- Loading/error/empty/success. Повторение запроса сохраняет фильтры.
- Подборки: отдельная сущность с целью, аудиторией, временем, сложностью и несколькими составителями. Авторы отдельных материалов не подменяются составителями подборки.
- Материалы разделов упорядочены по `position`, имеют номера `1.1`, `1.2` и продолжаются одной колонкой. Оглавление использует адреса разделов и историю браузера.
- Профиль: имя, фамилия, курс, группа; редактирование, валидация, отмена и сохранение в текущей демо-сессии.
- Компактное меню: «Профиль», «Моя библиотека», «Настройки», «Выйти». Сохранённые материалы и подборки открываются в библиотеке.

Адаптив дисциплины: desktop от 1200 px; tablet 768–1199; mobile 480–767; narrow mobile до 480. Каталог сохраняет ранее согласованный breakpoint панели фильтров. Мобильная строка дисциплины: номер/preview/тип/save, затем текст. Короткие подписи соответствуют мобильному референсу; «Подробнее» раскрывает полное описание.

## Декомпозиция

| Часть                                              | Ответственность                                     |
| -------------------------------------------------- | --------------------------------------------------- |
| `app/App`, `router`, `AppShell`                    | Провайдеры, маршруты, общая шапка и outlet          |
| `ThemeProvider`                                    | Тема и сохранение                                   |
| `ViewerProvider`                                   | Guest/demo viewer и локальные сохранения            |
| `pages/CatalogPage`                                | Сборка существующего каталога, параметры URL        |
| `pages/DisciplinePage`                             | Загрузка дисциплины и сборка секций                 |
| `pages/CollectionsPage`, `CollectionPage`          | Поиск подборок и упорядоченный маршрут              |
| `pages/ProfilePage`, `LibraryPage`, `SettingsPage` | Личная зона демо-пользователя                       |
| `components/layout`                                | Header, MobileMenu, PageContainer                   |
| `components/catalog`                               | Фильтры, поиск, toolbar, список, строка, пагинация  |
| `components/discipline`                            | Sidebar, Selector, Header, Section, MaterialItem    |
| `components/collections`                           | Фильтры, строка подборки, строка материала маршрута |
| `components/layout/AccountMenu`, `PersonalLayout`  | Меню аккаунта и навигация личной зоны               |
| `components/common`                                | Preview, SaveAction, ThemeToggle и базовые контролы |
| `api`                                              | Простые Promise-функции с AbortSignal               |
| `hooks/useCatalog`, `useDiscipline`, `useMaterial` | Загрузка, отмена устаревших ответов, повторение     |
| `hooks/useApiResource`, `useCollections`           | Загрузка подборок и библиотеки, отмена, retry       |
| `types`, `data`                                    | Domain types и отдельные mock-данные                |
| `styles`                                           | Base/accessibility, размеры и semantic colors       |

## Подключение Axum

Каталог и материалы каталога подключены к локальному backend. Запустите `cargo run` из `../backend` и `npm run dev` из `frontend`. Откройте `http://127.0.0.1:5173/catalog?all=1`, чтобы увидеть все 420 материалов SQLite. Обычный `/catalog` сохраняет начальный фильтр «Базы данных» (60 материалов).

Vite проксирует `/api` на `http://127.0.0.1:3000`. Production-развёртывание тоже должно направлять `/api` в backend: dev proxy действует только при `npm run dev`.

Граница работы с данными — `src/api/`:

- `http.ts`: общий GET JSON, `AbortSignal`, ошибки сети и HTTP с сообщением сервера.
- `catalog.ts`: `getCatalog(request, signal)` читает `/api/catalog`; отправляет повторяемые фильтры и `pageSize`, возвращает список, количество/страницы, справочники и счётчики фильтров. Параметр интерфейса `all` в API не отправляется.
- `disciplines.ts`: `getDisciplines`, `getDiscipline`, `getDisciplineMaterials`.
- `materials.ts`: `getMaterial` читает `/api/materials/{id}` со связанными discipline/teacher; `404` означает отсутствие материала, остальные ошибки дают retry. `getDemoMaterial` обслуживает демонстрационные подборки и дисциплины.
- `collections.ts`: `getCollections` и `getCollection`; публичность, поиск, составители и упорядоченные разделы.
- `profile.ts`: `updateProfile` с проверкой полей.
- `library.ts`: `getLibrary` со связанными материалами, сохранёнными подборками и подборками с участием пользователя.

UI не импортирует массивы mock-материалов и справочники. Hooks предусматривают loading, отмену устаревших ответов и retry. Каталог показывает ошибку при недоступном backend; автоматической подмены ответа mock-данными нет.

`CatalogMaterial.source` — локальный признак `api` или `mock`, добавляемый адаптером frontend. В HTTP JSON его нет. Обычный `/materials/:id` читает API; `?source=mock` выбирает демонстрационный материал. Этот параметр сохраняется рядом с `from`, включая возврат к разделу подборки. ID источников пересекаются (например, 1001), поэтому локальное сохранение хранит `{id, source}`. Старые числовые сохранения текущей вкладки читаются как mock-ссылки.

Серверный viewer позже подключается в `ViewerProvider`; настоящее сохранение — вместо `toggleSaved` и `toggleSavedCollection`. `CollectionContributor` отделяет роль (`author`, `curator`, `contributor`) от участника (преподаватель, пользователь, организация). В модель не заложен единственный преподаватель подборки. Реальную авторизацию и проверку прав должен выполнять сервер. Repository/service stack не вводился.

## Что остаётся mock

- Данные страниц дисциплин: 43 демонстрационных материала, 7 дисциплин, 6 преподавателей; 5 выбранных материалов курса «Базы данных». Каталог использует справочники и материалы HTTP API.
- 4 публичные подборки и 12 дополнительных материалов маршрута. Их ссылки явно выбирают источник mock; количества 12/8/6/5 вычисляются по составу подборок.
- Метаданные размещения материала в курсе могут отличаться от общей карточки каталога.
- Демо-аккаунт Алексей К. и сохранённые ссылки `{id, source}` живут в sessionStorage текущей вкладки. Библиотека загружает сохранённые API-материалы через HTTP. Новая сессия начинается гостем; выход очищает сохранения. При недоступном storage просмотр работает.
- «Создать аккаунт» открывает тот же явно обозначенный демонстрационный вход. Пароли и токены не запрашиваются.
- Файлы, playback, настоящая регистрация/auth и серверное сохранение отсутствуют. SQLite обслуживает каталог.
- Профиль, библиотека и сохранение подборок работают локально. Email, пароль и безопасность пока не подключены; это явно указано в настройках. История, создание подборок, управление преподавателя и загрузка файлов не добавлялись.

## Этап профиля и подборок — 29 сентября 2026

Добавлены `types/collection`, `data/collections`, `data/collectionMaterials`, API `collections/profile/library`, hooks `useApiResource/useCollections`, страницы `ProfilePage/CollectionsPage/CollectionPage/LibraryPage/SettingsPage`, компоненты `AccountMenu/PersonalLayout` и `components/collections`. Расширены `Viewer`, `ViewerProvider`, `SaveAction`, общий `MaterialPreview`, маршруты и семантические цвета PDF.

При очистке удалены старые стили аккаунта из `Header.module.css` и недостижимая ветка заглушки подборок из `InformationPage`. Подсветка оглавления определяется URL вместо дополнительного observer/state. Исправлен повторный переход на login: исходный redirect больше не вкладывается в другой login URL.

TypeScript/React оформлен развёрнуто: statements отдельно, `if/else` с блоками, длинные JSX-выражения вынесены в локальные переменные, props на отдельных строках. Зависимости и backend сохранены, коммиты не создавались.

Сборка TypeScript/Vite и Oxlint повторены после очистки. Браузерные проверки и снимки перечислены в [design-qa.md](design-qa.md).

## Файлы предыдущего этапа: каталог и дисциплина

Новые (пары TSX/CSS Modules указаны вместе):

```text
src/app/AppShell.tsx
src/app/providers/ViewerContext.ts
src/app/providers/ViewerProvider.tsx
src/types/viewer.ts
src/data/viewer.ts
src/data/courseMaterials.ts
src/api/disciplines.ts
src/api/materials.ts
src/hooks/useViewer.ts
src/hooks/useDiscipline.ts
src/hooks/useMaterial.ts
src/utils/redirect.ts
src/utils/catalogSearchParams.ts
src/utils/materialLabels.ts
src/pages/DisciplinePage/DisciplinePage.tsx + DisciplinePage.module.css
src/pages/LoginPage/LoginPage.tsx
src/pages/MaterialPage/MaterialPage.tsx + MaterialPage.module.css
src/pages/InformationPage/InformationPage.tsx + InformationPage.module.css
src/components/layout/MobileMenu/MobileMenu.tsx + MobileMenu.module.css
src/components/common/MaterialPreview/MaterialPreview.tsx + MaterialPreview.module.css
src/components/common/SaveAction/SaveAction.tsx + SaveAction.module.css
src/components/discipline/DisciplineSidebar/DisciplineSidebar.tsx + DisciplineSidebar.module.css
src/components/discipline/DisciplineSelector/DisciplineSelector.tsx + DisciplineSelector.module.css
src/components/discipline/DisciplineHeader/DisciplineHeader.tsx + DisciplineHeader.module.css
src/components/discipline/DisciplineMaterialItem/DisciplineMaterialItem.tsx + DisciplineMaterialItem.module.css
src/components/discipline/MaterialSection/MaterialSection.tsx + MaterialSection.module.css
reference/qa/implementation/*.png
reference/qa/implementation/discipline-widths.json
```

Изменена логика/стили существующих файлов:

```text
src/app/App.tsx
src/app/router.tsx
src/components/layout/Header/Header.tsx + Header.module.css
src/pages/CatalogPage/CatalogPage.tsx
src/components/catalog/CatalogFilters/CatalogFilters.tsx
src/components/catalog/CatalogToolbar/CatalogToolbar.tsx
src/components/catalog/MaterialList/MaterialList.tsx
src/components/catalog/MaterialListItem/MaterialListItem.tsx + MaterialListItem.module.css
src/components/common/Badge/Badge.tsx
src/components/common/SearchInput/SearchInput.tsx
src/components/common/ThemeToggle/ThemeToggle.tsx
src/api/catalog.ts
src/hooks/useCatalog.ts
src/types/material.ts
src/types/discipline.ts
src/data/materials.ts
src/data/disciplines.ts
src/utils/format.ts
src/styles/variables.css
src/styles/themes.css
README.md
design-qa.md
```

Прежний `components/catalog/MaterialListItem/MaterialPreview.tsx` перенесён в `components/common/MaterialPreview`. Его неиспользуемые стили удалены из каталожной строки. Остальные TS/TSX также отформатированы: statements отдельно, `if` развёрнуты, длинные JSX-выражения вынесены в переменные, props на отдельных строках. Зависимости и backend сохранены. Коммиты не создавались.

## Проверки

```powershell
npm run build
npm run lint
```

Обе команды завершились с exit code 0 без ошибок и предупреждений после финальной очистки 29 сентября 2026 года. Build включает `tsc -b`. Для нового этапа проверены профиль, список и детали всех четырёх подборок, обе темы, ширины 1487/768/390/320, гостевые и личные переходы, редактирование/валидация/отмена, save/unsave, URL/reload, оглавление, меню и Escape. Console errors/warnings отсутствуют. Прежние проверки каталога и дисциплины сохранены в истории отчёта.

Снимки и сравнения первоначального дизайна — в [design-qa.md](design-qa.md). 1 октября дополнительно проверены HTTP-каталог из 420 материалов, поиск SQL с двумя типами, сортировка, пагинация, материал и возврат, сохранения разных источников с одинаковым ID. Остановка backend дала ошибку HTTP 502 через Vite; повтор после запуска восстановил каталог. Отсутствующий материал показывает «Материал не найден».
