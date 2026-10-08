# Design QA — управление и личный профиль

8 октября 2026 года.

## Источник и область сравнения

Пользователь дал схематические примеры, затем явно выбрал: сохранить общее верхнее меню, перенести разделы админки в компактную боковую панель, оставить профиль минималистичным с разделителями. Это проверка композиции/состояний по принятому brief, а не заявление о пиксельном копировании моков.

Source visual truth:

- `.codex/tmp/ui-reference-admin.png` — 1042×598, копия приложенного концепта.
- `.codex/tmp/ui-reference-profile.png` — 1167×708, последний концепт профиля пользователя.

Implementation:

- `.codex/tmp/ui-admin-dark.png`, `ui-admin-light.png` — 795×702.
- `.codex/tmp/ui-profile-dark.png` — 795×840 full page; `ui-profile-light.png` — 795×702 viewport.
- `.codex/tmp/ui-account-menu-dark.png` — меню аккаунта.
- `.codex/tmp/ui-profile-mobile-dark.png`, `ui-admin-mobile-dark.png`, `ui-institutions-table-fixed.png` — 390×844.
- `.codex/tmp/ui-institutions-table-fixed-desktop.png` — 795×702.

Reference pixels и CSS viewport не приравнивались: source — иллюстрация в пользовательском сообщении. Contact sheets пропорционально масштабируют изображения для совместного просмотра; изменения существующей палитры, верхнего меню и содержания не требовались. Native screenshots получены при density 1 без растягивания. После responsive проверки viewport override снят.

## Совместное сравнение

- Full view: `.codex/tmp/ui-admin-comparison.png`, `ui-profile-comparison.png`, просмотрены как единые изображения с source и обеими темами.
- Focused region: `.codex/tmp/ui-profile-focused.png` — identity/private fields source и implementation вместе. Проверены размер edit action, группа имени/описания, подпись приватности, читаемость labels.
- Table iteration: `.codex/tmp/ui-table-comparison.png` — before/after в одной светлой теме и одном viewport 795×702.

## Проверенные поверхности

- Typography: сохранён Arimo продукта, читаемая иерархия page/section/card, вторичная кнопка профиля.
- Layout rhythm: компактная sidebar, четыре карточки сводки, одинаковые filters/controls, профиль с разделителями и сгруппированными полями.
- Colors/tokens: существующие theme variables, синий active background отдельно от focus outline, нейтральные поверхности в обеих темах.
- Image/icon quality: существующий логотип и Phosphor icons, новых bitmap/декоративных assets нет. Avatar initials — существующий компонент продукта.
- Copy/content: реальные counts; понятные «На проверке», «Найти учреждение» и empty state; без выдуманных жалоб/показателей. Административный контекст явно подписан.

## История исправлений

1. [P2] Таблица при 795 px слишком сжимала название учреждения в несколько строк. Добавлены минимальная ширина таблицы/первого столбца и локальная focusable scroll region. After capture в том же viewport показывает компактные строки, все данные доступны горизонтальной прокруткой. На телефоне тело страницы не переполняется; основной вид — карточки.
2. [P0 runtime] Модерация возвращала 500 из-за SELECT FOR SHARE в read-only snapshot. Read access отделён от locked write access. Backend пересобран; browser queue показывает корректный empty state, API role checks подтверждают доступ/запрет по permissions.
3. [P0 transient dev state] В процессе изменения frontend/backend были старый overview payload без memberAttention и Vite empty module cache. После перезапуска backend/Vite страницы и role browser checks работали с новым контрактом. Console прочитана: найденные ошибки относятся к этим историческим состояниям; отсутствие ошибок за весь сеанс не заявляется.

## Взаимодействия

Проверены обе темы, account dropdown/current item, Cards/Table, пустой moderation list без pagination, переход к institution membership requests, profile membership anchor, mobile collapse/menu navigation. По отдельному запросу пользователя проверены пять временных ролей через API и browser; студент сохранил описание через UI с сохранением курса/группы; scope управляющего и меню остальных ролей подтверждены. Test fixtures удалены, counts вновь 10/3.

## Follow-up polish и ограничения

- [P3] Модератор без учебного статуса пока подписан «Пользователь», хотя его permissions/menu работают. Записано для будущей корректировки; пользователь запретил дальнейшие code changes.
- Подсказка горизонтальной прокрутки таблицы может быть заметнее; прокрутка и keyboard focus уже доступны.
- Full публикация/решение модератора, notifications и public profiles в эту проверку не входили. Непонятный пользователю путь Draft → Submit записан отдельно, не объявлен исправленным.
- Пароль из сохранённого admin credentials файла не подошёл при восстановлении localhost входа. На localhost пользователь входит текущим паролем; основная сессия 127.0.0.1 сохранилась. Это не изменение credentials.

В принятой композиции нет оставшихся actionable P0/P1/P2 различий. Полные бизнес-сценарии и перечисленные будущие возможности не считаются проверенными этой визуальной QA.

final result: passed
