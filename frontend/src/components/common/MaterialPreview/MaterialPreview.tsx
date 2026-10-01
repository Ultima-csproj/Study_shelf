import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { FilePptIcon } from "@phosphor-icons/react/dist/csr/FilePpt";
import { FilePdfIcon } from "@phosphor-icons/react/dist/csr/FilePdf";
import { FlaskIcon } from "@phosphor-icons/react/dist/csr/Flask";
import { BookOpenIcon } from "@phosphor-icons/react/dist/csr/BookOpen";
import { VideoCameraIcon } from "@phosphor-icons/react/dist/csr/VideoCamera";
import { PlayIcon } from "@phosphor-icons/react/dist/csr/Play";
import type { Material, PreviewKind } from "../../../types/material";
import styles from "./MaterialPreview.module.css";
const materialIcons = {
  lecture: FileTextIcon,
  article: FileTextIcon,
  lab: FlaskIcon,
  book: BookOpenIcon,
  video: VideoCameraIcon,
};
const previewIcons = {
  presentation: FilePptIcon,
  document: FileTextIcon,
  pdf: FilePdfIcon,
};
export function MaterialPreview({
  material,
  variant = "catalog",
  kind,
}: {
  material: Material;
  variant?: "catalog" | "course" | "collection";
  kind?: PreviewKind;
}) {
  const Icon = kind ? previewIcons[kind] : materialIcons[material.type];
  const hasImage = material.preview && !kind;
  const imageClassName = [
    styles.image,
    material.type === "video" ? styles.video : "",
    material.type === "book" ? styles.book : "",
  ].join(" ");
  const iconClassName = [
    styles.icon,
    kind === "document" ? styles.document : "",
    kind === "pdf" ? styles.pdf : "",
  ].join(" ");
  const imageWidth = material.type === "video" ? 600 : 240;
  const imageHeight = material.type === "video" ? 400 : 252;
  const iconWeight = kind === "document" ? "fill" : "regular";
  return (
    <div className={`${styles.slot} ${styles[variant]}`} aria-hidden="true">
      {hasImage ? (
        <div className={imageClassName}>
          <img
            src={material.preview}
            alt=""
            width={imageWidth}
            height={imageHeight}
          />
          {material.type === "video" && (
            <>
              <span className={styles.play}>
                <PlayIcon size={20} weight="fill" />
              </span>
              {material.duration && (
                <span className={styles.duration}>{material.duration}</span>
              )}
            </>
          )}
        </div>
      ) : (
        <div className={iconClassName}>
          <Icon size={38} weight={iconWeight} />
        </div>
      )}
    </div>
  );
}
