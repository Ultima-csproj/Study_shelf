import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { FilePptIcon } from "@phosphor-icons/react/dist/csr/FilePpt";
import { FilePdfIcon } from "@phosphor-icons/react/dist/csr/FilePdf";
import { FlaskIcon } from "@phosphor-icons/react/dist/csr/Flask";
import { BookOpenIcon } from "@phosphor-icons/react/dist/csr/BookOpen";
import { VideoCameraIcon } from "@phosphor-icons/react/dist/csr/VideoCamera";
import { PlayIcon } from "@phosphor-icons/react/dist/csr/Play";
import type { ReactNode } from "react";
import type { MaterialType, PreviewKind } from "../../../types/material";
import { formatDuration } from "../../../utils/materialLabels";
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

interface MaterialPreviewProps {
  material: { type: MaterialType; preview?: string | null; duration?: string; durationSeconds?: number | null };
  variant?: "catalog" | "course" | "collection";
  kind?: PreviewKind;
}

export function MaterialPreview({
  material,
  variant = "catalog",
  kind,
}: MaterialPreviewProps) {
  const duration = material.duration ?? formatDuration(material.durationSeconds);
  const imageSource = material.preview ?? undefined;
  let Icon = materialIcons[material.type];

  if (kind) {
    Icon = previewIcons[kind];
  }

  let hasImage = false;

  if (material.preview && !kind) {
    hasImage = true;
  }

  const imageClassNames = [styles.image];

  if (material.type === "video") {
    imageClassNames.push(styles.video);
  }

  if (material.type === "book") {
    imageClassNames.push(styles.book);
  }

  const imageClassName = imageClassNames.join(" ");
  const iconClassNames = [styles.icon];

  if (kind === "document") {
    iconClassNames.push(styles.document);
  }

  if (kind === "pdf") {
    iconClassNames.push(styles.pdf);
  }

  const iconClassName = iconClassNames.join(" ");
  const slotClassName = `${styles.slot} ${styles[variant]}`;
  let imageWidth = 240;
  let imageHeight = 252;

  if (material.type === "video") {
    imageWidth = 600;
    imageHeight = 400;
  }

  let iconWeight: "fill" | "regular" = "regular";

  if (kind === "document") {
    iconWeight = "fill";
  }

  let videoControls: ReactNode = null;

  if (material.type === "video") {
    let durationLabel: ReactNode = null;

    if (duration) {
      durationLabel = (
        <span className={styles.duration}>{duration}</span>
      );
    }

    videoControls = (
      <>
        <span className={styles.play}>
          <PlayIcon size={20} weight="fill" />
        </span>
        {durationLabel}
      </>
    );
  }

  let previewContent: ReactNode;

  if (hasImage) {
    previewContent = (
      <div className={imageClassName}>
        <img
          src={imageSource}
          alt=""
          width={imageWidth}
          height={imageHeight}
        />
        {videoControls}
      </div>
    );
  } else {
    previewContent = (
      <div className={iconClassName}>
        <Icon size={38} weight={iconWeight} />
      </div>
    );
  }

  return (
    <div className={slotClassName} aria-hidden="true">
      {previewContent}
    </div>
  );
}
