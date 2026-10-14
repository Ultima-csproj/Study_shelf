import { lazy, Suspense } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import { PencilSimpleIcon } from "@phosphor-icons/react/dist/csr/PencilSimple";
import { useViewer } from "../../hooks/useViewer";
import { useNavigationContext } from "../../hooks/useNavigationContext";
import { ProfileEditor } from "../../components/account/profile-editor/ProfileEditor";
import type { User } from "../../types/viewer";
import { ProfileSummary } from "./ProfileSummary";
import { ProfileLayout } from "./ProfileLayout";
import styles from "./ProfilePage.module.css";

const ProfilePublications = lazy(() => import("./ProfilePublications"));

export function ProfilePage() {
  const { viewer, updateUser } = useViewer();
  const [params] = useSearchParams();
  const navigate = useNavigate();
  const { back, linkTo } = useNavigationContext("/profile");
  const editing = params.get("edit") === "1";
  const publications = params.get("tab") === "publications";

  if (viewer.kind !== "authenticated") {
    return null;
  }
  const startEditing = () => {
    navigate(linkTo("/profile?edit=1"));
  };
  const cancelEditing = () => navigate(back.path, { replace: true });
  const saved = (user: User) => {
    updateUser(user);
    navigate(back.path, {
      replace: true,
      state: { notice: "Изменения профиля сохранены." },
    });
  };
  const editAction = editing ? null : (
    <button type="button" className={styles.secondary} onClick={startEditing}>
      <PencilSimpleIcon size={20} aria-hidden="true" />
      Редактировать профиль
    </button>
  );

  return (
    <ProfileLayout
      user={viewer.user}
      editing={editing}
      editAction={editAction}
      onCancel={cancelEditing}
    >
      {editing ? (
        <>
          <div className={styles.heading}>
            <div>
              <h1>Редактирование профиля</h1>
              <p>Личная информация и данные об обучении.</p>
            </div>
          </div>
          <ProfileEditor
            user={viewer.user}
            onSaved={saved}
            onCancel={cancelEditing}
          />
        </>
      ) : publications ? (
        <Suspense fallback={<p role="status">Загружаем публикации…</p>}>
          <ProfilePublications />
        </Suspense>
      ) : (
        <ProfileSummary user={viewer.user} onEdit={startEditing} />
      )}
    </ProfileLayout>
  );
}
