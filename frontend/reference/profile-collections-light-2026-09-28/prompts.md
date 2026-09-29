# Референсы профиля и подборок

Дата: 28 сентября 2026. Способ: встроенный Image Gen. Код приложения не изменялся.

Итоговые изображения: profile-student-desktop.png, collections-guest-desktop.png, collection-route-guest-desktop.png.

Это три экрана одной системы, а не взаимоисключающие варианты. Профиль показан от лица студента; подборки — от лица гостя. Светлая тема, desktop. В открытой подборке показан видимый фрагмент маршрута: 8 из 12 материалов.

Визуальная основа: ../qa/implementation/discipline-desktop-light.png.

## Профиль

Use case: ui-mockup. Generate ONE high fidelity desktop website reference screen for the Russian educational platform "Учебная полка". Target viewport 1487 x 1058 px. Use the attached actual existing product screen as STRICT visual grounding: same shared top header/logo, Arimo/Arial typography, navy #10203d, muted #65728a, blue #1677e8, white background, pale blue active selection #eaf3ff, fine border #dfe5ed. Continue this exact visual system; do not redesign the brand or create a marketing layout. Current date anchor 28 September 2026; no activity dates needed.
This screen is MY PROFILE, signed in as student Алексей К. Primary purpose: view and edit personal identity and academic information. Distinguish profile from account settings and personal library. Top header: brand, Каталог, Дисциплины, Подборки, existing search field, small theme switch, initials avatar АК with name Алексей К. No login button for this authenticated screen.
Left pale sidebar width about 284px: small heading "Личная зона", navigation "Профиль" selected blue, "Моя библиотека", "Настройки". No saved count, no My Collections separate item, no teaching management for student.
Main white surface generous whitespace, breadcrumb "Личная зона / Профиль", page heading "Мой профиль", muted subtitle "Личная информация и данные об обучении". A small blue primary "Редактировать профиль" button top right of main heading. Identity block on base surface: modest circular initials avatar АК, "Алексей К.", subtle "Студент", caption "Информация видна только вам." Thin divider. Under heading "Личная информация" show a clean readable two-column label/value layout, "Имя" -> "Алексей", "Фамилия" -> "Ковалёв". Under "Обучение" show "Курс" -> "3 курс", "Группа" -> "ИС-31". This is view mode, not a form full of inputs. One low emphasis helper link at bottom "Email, пароль и безопасность — в настройках аккаунта".
Also show a small realistic account dropdown open under header avatar over the empty upper right portion; it must not cover the main profile content. Dropdown heading Алексей К. and Студент; compact icon menu only "Профиль", "Моя библиотека", "Настройки", a separator then "Выйти". This demonstrates the agreed compact personal navigation; avoid additional entries.
No metrics dashboard, progress charts, history, saved materials inside profile, large decorative illustration, nested cards, dark background, enormous avatar, device bezel, browser chrome, backend/database jargon. Use simple dividers, restrained library icons and readable 14-16px text. Strong primary heading about32px. Render Cyrillic clearly. Only this single coherent screen, no collage and no numbered variant labels.

## Публичный список подборок

Use case: ui-mockup. Generate ONE high fidelity desktop website reference screen for the Russian educational platform "Учебная полка": PUBLIC COLLECTIONS DIRECTORY seen by a GUEST. Target viewport1487 x1058px. Attached existing discipline screen is strict visual style reference: preserve same header/logo/search/theme/login, Arimo/Arial, white #ffffff, navy #10203d, muted #65728a, blue #1677e8, pale blue #eaf3ff, border #dfe5ed. No new brand direction. Current date 28 September2026; no dates necessary.
Header nav Каталог, Дисциплины, Подборки (active blue underline), search, theme toggle, Войти. No logged in avatar, no personal sidebar, no create/edit collection button for guest.
Left sidebar same visual weight as current catalog/discipline sidebar, about284px. Heading "Фильтры"; sections "Дисциплина" with checkbox Базы данных selected, Компьютерные сети, Операционные системы; "Курс" with checkboxes 1 курс,2 курс,3 курс selected; muted "Сбросить фильтры". Don't filter by a single teacher or suggest a collection has one owner.
Main area heading "Подборки", subtitle "Учебные маршруты для повторения, практики и подготовки к экзаменам". Search field placeholder "Поиск по названию или теме". Selected light chips "Базы данных" and "3 курс"; modest sort "Сначала новые". Render FOUR useful collections as horizontal editorial list rows on one base white surface separated by fine lines. No card grid, nested cards, decorative covers or hero banners. Each row small blue outlined library/list icon, bold navy title, 1–2 lines useful short description, explicit "Составили:" plus multiple names where relevant, compact metadata count/discipline/course/estimated time. Right aligned small blue "Открыть →" rather than prominent competing buttons.
Rows exact Russian:
"Подготовка к экзамену по базам данных"
"Основные материалы по темам экзамена: нормализация, SQL, индексы и транзакции."
"Составили: Иванова М. С., Петров А. В."
"12 материалов · Базы данных · 3 курс · 2–3 часа"
"SQL перед экзаменом"
"Быстрое повторение SELECT, JOIN, GROUP BY и подзапросов с примерами."
"Составили: Иванова М. С., Смирнов Д. К."
"8 материалов · Базы данных · 3 курс · 2 часа"
"Практика проектирования баз данных"
"От модели предметной области до схемы и нормальных форм."
"Составили: Петров А. В., Алексей К."
"6 материалов · Базы данных · 3 курс · 3 часа"
"Индексы и производительность"
"Разбор индексов и планов выполнения запросов PostgreSQL."
"Составили: Смирнов Д. К."
"5 материалов · Базы данных · 3 курс · 1–2 часа"
Small bottom helper "Публичные подборки доступны без входа." Guest browses freely, any save action optional gray lock with caption login needed, do not make whole directory locked.
Purposeful whitespace, readable 14-16px product text,32px heading. One screen only, no mock code/schema, no device/bezel/browserchrome, no variant numbering. Names are COMPILERS of collections, never implied authors of all included materials.

## Страница подборки

Use case: ui-mockup. Generate ONE high fidelity Russian desktop website reference screen for "Учебная полка": PUBLIC COLLECTION DETAIL as a guest. Target1487x1058px. Attached product screenshot is strict visual grounding: existing shared header/logo/nav/search/theme/Войти; same Arimo/Arial typography, white surface, navy #10203d, muted #65728a, blue #1677e8, fine #dfe5ed dividers and pale blue accents. Current date28September2026; no visible dates necessary. This is an educational route, NOT a flat folder of links.
Header nav Подборки active, guest Войти. Narrow pale left sidebar about284px headed "В этой подборке" with four numbered anchor items: "1. Повторить основы" selected, "2. SQL", "3. Производительность", "4. Перед экзаменом"; lower "Все подборки →". No personal zone or teacher filter.
Main breadcrumb "Подборки / Базы данных". Heading "Подготовка к экзамену по базам данных" readable about30-32px. Short description "Нормализация, SQL, индексы и транзакции — от базовых тем к экзаменационным вопросам." Compact metadata "8 материалов · Базы данных · 3 курс · 2–3 часа · Средняя сложность". Explicit line "Составили: Иванова М. С., Смирнов Д. К." Optionally second smaller role caption curator for one person ONLY if uncluttered; prefer simple "Составили" in UI.
A small right aligned gray outlined lock action "Сохранить подборку"; helper "Сохранение — после входа." Available guest content must remain fully readable. No create/edit buttons for guest.
Two brief editorial information blocks on same white base without boxes: "Зачем эта подборка" -> "Повторить ключевые темы курса перед экзаменом. Материалы расположены от базовых понятий к более сложным задачам." and "Для кого" -> "Студенты 3 курса, изучающие базы данных." Keep descriptions with comfortable line length.
Below thin divider show FOUR clearly grouped sections, each section heading with a small blue step number and two compact material rows, total8 materials. Each row has small actual file icon or source-like video thumbnail, bold clickable title, format and explicitly labelled author/source; unlike compilers, authors vary.
"1. Повторить основы": "Введение в реляционные базы данных" / "Видеолекция · Автор: Иванова М. С. · 42 мин"; "Нормальные формы" / "Лекция · Автор: Петров А. В. · PDF".
"2. SQL": "SELECT, JOIN и GROUP BY" / "Практикум · Автор: Петров А. В. · PDF"; "Подзапросы и оконные функции" / "Видео · Автор: Кузнецов Д. М. · 35 мин".
"3. Производительность": "Индексы в PostgreSQL" / "Внешняя статья · Источник: PostgreSQL"; "Планы выполнения: EXPLAIN" / "Методичка · Источник: кафедра".
"4. Перед экзаменом": "SQL: краткая шпаргалка" / "Конспект · Автор: Смирнов Д. К. · PDF"; "Типовые экзаменационные вопросы" / "Документ · Источник: кафедра".
Use short rows and readable14px metadata,16px titles,20px section headers so full route fits cleanly; no content clipped, no tiny illegible text. No per-row save button needed, avoid repeated locks.
Distinct author identity per MATERIAL vs multi-person compile identity for COLLECTION is essential. No database schema, source code, implementation details, nested cards, progress tracking/checkmarks for guest, marketing hero, dark background, device frame, browser chrome or variant labels. Single coherent screen.

### Итоговая правка страницы подборки

Сохранить всё изображение. В метаданных заменить 8 материалов на 12 материалов. Строка составителей: Составили: Иванова М. С., Петров А. В. Авторы отдельных материалов остаются прежними. Восемь видимых строк — начало маршрута из 12 материалов.

