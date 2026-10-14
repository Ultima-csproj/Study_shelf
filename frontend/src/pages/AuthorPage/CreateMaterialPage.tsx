import { withNavigationOrigin } from "../../utils/navigation";
import { useNavigationContext } from "../../hooks/useNavigationContext";
import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useViewer } from "../../hooks/useViewer";
import { useMaterialAction } from "../../hooks/useMaterialAction";
import { createMaterial, type ContentInput } from "../../api/materialWorkspace";
import { MaterialContentForm } from "../../components/materials/editor/MaterialContentForm";
import { AuthorHelp } from "../../components/materials/help/AuthorHelp";
import styles from "./CreateMaterialPage.module.css";

export function CreateMaterialPage() {
  const { back } = useNavigationContext("/author/materials");
  const { viewer } = useViewer();
  const navigate = useNavigate();
  const [institution, setInstitution] = useState("");
  const action = useMaterialAction(() => {});
  if (viewer.kind !== "authenticated") {
    return null;
  }
  const institutions = viewer.user.memberships.filter(
    (item) =>
      item.state === "active" && item.permissions.includes("create_materials"),
  );
  const ownCredit = {
    contributorId: viewer.user.primaryContributorId,
    role: "author" as const,
    displayName: viewer.user.name,
  };
  const initial: ContentInput = {
    title: "",
    description: "",
    materialType: "article",
    sourceYear: null,
    previewUrl: null,
    durationSeconds: null,
    fileSizeBytes: null,
    tags: [],
    credits: [ownCredit],
  };
  const save = async (content: ContentInput) => {
    await action.run(async () => {
      const result = await createMaterial(
        content,
        institution === "" ? null : Number(institution),
      );
      navigate(
        withNavigationOrigin(
          `/author/materials/${result.materialId}?revision=${result.revisionId}`,
          back.path,
        ),
        {
          replace: true,
          state: {
            notice:
              "Черновик создан. Продолжайте редактирование или отправьте материал на модерацию.",
          },
        },
      );
    }, "Черновик создан.");
  };
  const ownerField = (
    <label>
      Владелец
      <select
        value={institution}
        onChange={(event) => setInstitution(event.target.value)}
      >
        <option value="">Мой аккаунт</option>
        {institutions.map((item) => (
          <option key={item.institutionId} value={item.institutionId}>
            {item.institutionName}
          </option>
        ))}
      </select>
    </label>
  );
  return (
    <div className={styles.page}>
      <Link className={styles.back} to={back.path}>
        ← {back.label}
      </Link>
      <header className={styles.header}>
        <div>
          <h1>Создать материал</h1>
          <p>
            Заполните основные сведения. Материал сначала сохранится как
            черновик.
          </p>
        </div>
      </header>
      <div className={styles.layout}>
        <div>
          {action.notice && (
            <p className={styles.notice} role="status">
              {action.notice}
            </p>
          )}
          <MaterialContentForm
            initial={initial}
            mode="create"
            busy={action.busy}
            allowSubmit={false}
            ownerField={ownerField}
            onSave={save}
          />
        </div>
        <AuthorHelp creating />
      </div>
    </div>
  );
}
