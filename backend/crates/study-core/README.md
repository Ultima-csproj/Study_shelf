# study-core

Core содержит типы и правила без SQLx, HTTP и TypeScript. Catalog читает проекцию MaterialSummary; доменные Material/MaterialRevision сохраняются через study-storage-postgres с этапа 10. Checked snapshots восстанавливают данные без повторения исторических переходов.

## Порядок чтения

1. `identity/user.rs`, `identity/contributor.rs`: аккаунт, глобальная модерация и авторская идентичность.
2. `institution/membership.rs`: учебные статусы, состояние членства и отдельные полномочия учреждения.
3. `material/owner.rs`, `record.rs`, `credit.rs`, `content.rs`: единственный владелец, запись материала и снимок метаданных редакции.
4. `material/revision.rs`, `draft.rs`: состояние, база, изменения черновика и отправка на проверку.
5. `material/actor.rs`, `policy.rs`: серверный контекст пользователя и проверки доступа.
6. `material/moderation.rs`, `publication.rs`: решение модерации, self-review и переключение публикации.

catalog::MaterialSummary — публичная проекция чтения; material::Material — стабильная запись. Содержание находится в MaterialRevision. Summary включает credits, независимые placements, metadata и время текущей/первой публикации.

## Основные правила

- Владелец — User или Institution. Создатель хранится отдельно. В collaborators нет роли Owner.
- Учебный статус не предоставляет administrative permissions. `CreateMaterials` разрешает создание записи от имени учреждения, `ManageMaterials` — управление его материалами; создание не выдаёт автоматически дополнительные права. Для подготовки редакций требуется управление или явно предоставленный EditRevision.
- `Actor` строится приложением из установленной личности и актуальных серверных полномочий. Здесь нет входа, назначения прав и чтения ролей из браузера.
- Публичная карточка получает метаданные из текущей опубликованной редакции. Гостевое чтение конкретной редакции проверяется `can_read_revision`, а не только `can_read_material`.
- Draft можно менять; PendingModeration, Published и Rejected фиксированы. После отказа создаётся новый Draft с новым ID на актуальной базе. Проверка уникальности ID всех редакций будет обеспечиваться хранением.
- Отправка и одобрение проверяют базовую редакцию. Нет автоматического rebase/merge и промежуточных снимков содержимого.
- ModerationAction содержит сохранённый was_self_review. Создание, фактическое изменение метаданных или отправка этой редакции учитываются; одно лишь владение — нет.
- Время — SystemTime; timestamp codec приводит к микросекундной точности и проверяемо преобразует TIMESTAMPTZ/RFC3339 UTC. public_updated_at получается из текущей ссылки публикации и меняется вместе с ней.

## Пример использования домена

Пример показывает переходы в памяти. Данные User и Actor в приложении будут загружаться сервером.

```rust
use std::time::SystemTime;
use study_core::catalog::MaterialType;
use study_core::identity::{PlatformPermission, User, UserState};
use study_core::ids::{MaterialId, MaterialRevisionId, UserId};
use study_core::material::{
    Actor, Material, MaterialError, MaterialOwner, MaterialRevision,
    MaterialVisibility, RevisionContent,
};

fn example() -> Result<(), MaterialError> {
    let user = User {
        id: UserId(1),
        display_name: "Автор".to_owned(),
        state: UserState::Active,
        permissions: vec![PlatformPermission::ModeratePublicMaterials],
    };
    let actor = Actor {
        user: &user,
        memberships: &[],
        collaborators: &[],
    };
    let at = SystemTime::now();
    let mut material = Material::new(
        &actor, MaterialId(42), MaterialOwner::User(user.id),
        MaterialVisibility::Public, at,
    )?;
    let content = RevisionContent {
        title: "Введение в базы данных".to_owned(),
        description: "Первый материал".to_owned(),
        material_type: MaterialType::Lecture,
        tags: vec!["SQL".to_owned()],
        credits: Vec::new(),
        metadata: Default::default(),
    };
    let mut revision = MaterialRevision::new_draft(
        &actor, &material, MaterialRevisionId(7), content, at,
    )?;
    revision.submit_for_moderation(&actor, &material, at)?;
    let action = material.approve_revision(&actor, &mut revision, at)?;
    // action.was_self_review() == true: проверяющий создал и отправил редакцию.
    // PostgreSQL repository сохраняет action в транзакции с публикацией.
    let _ = action;
    Ok(())
}
```

approve_revision меняет запись и редакцию в памяти и возвращает audit-снимок. PostgresMaterialRepository::approve сохраняет pointer/state/search/audit одной транзакцией с проверкой базы и CAS. ChangeProposal, auth, назначение collaborator-прав, assets и заявки учреждений подключаются отдельно.

Подробное объяснение семантики функций находится в .codex/docs/backend-function-semantics.md.
