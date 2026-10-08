import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useViewer } from "../../hooks/useViewer";
import { useMaterialAction } from "../../hooks/useMaterialAction";
import { createMaterial, type ContentInput } from "../../api/materialWorkspace";
import { MaterialContentForm } from "../../components/materialWorkspace/MaterialContentForm";
import styles from "../../components/materialWorkspace/Workspace.module.css";

export function CreateMaterialPage() {
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
  const initial: ContentInput = {
    title: "",
    description: "",
    materialType: "article",
    sourceYear: null,
    previewUrl: null,
    durationSeconds: null,
    fileSizeBytes: null,
    tags: [],
    credits: [
      { contributorId: viewer.user.primaryContributorId, role: "author" },
    ],
  };
  const save = async (
    content: ContentInput,
    _submit: boolean,
    reason: string,
  ) => {
    await action.run(async () => {
      const result = await createMaterial(
        content,
        institution === "" ? null : Number(institution),
        reason,
      );
      navigate(
        "/author/materials/" +
          result.materialId +
          "?revision=" +
          result.revisionId,
      );
    });
  };
  return (
    <div className={styles.page}>
      <Link to="/author/materials">← Мои публикации</Link>
      <h1>Создать материал</h1>
      <p>
        Сначала создаётся приватный черновик. Публичная публикация проходит
        модерацию.
      </p>
      <label className={styles.label}>
        Владелец
        <select
          disabled={action.busy}
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
      {action.notice && <p role="status">{action.notice}</p>}
      <MaterialContentForm
        initial={initial}
        busy={action.busy}
        allowSubmit={false}
        onSave={save}
      />
    </div>
  );
}
