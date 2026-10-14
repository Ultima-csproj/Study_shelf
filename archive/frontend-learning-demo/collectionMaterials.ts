import videoCover from "../assets/covers/transactions-video.webp";
import type { MaterialType } from "../types/material";
import type { DemoMaterial } from "../types/demoMaterial";

interface RouteMaterialSeed {
  id: number;
  title: string;
  type: MaterialType;
  authorName: string;
  teacherId: number;
  duration?: string;
  source?: boolean;
}

const seeds: RouteMaterialSeed[] = [
  {
    id: 1001,
    title: "Введение в реляционные базы данных",
    type: "video",
    authorName: "Иванова М. С.",
    teacherId: 1,
    duration: "42:00",
  },
  {
    id: 1002,
    title: "Нормальные формы",
    type: "lecture",
    authorName: "Петров А. В.",
    teacherId: 2,
  },
  {
    id: 1003,
    title: "SELECT, JOIN и GROUP BY",
    type: "lab",
    authorName: "Петров А. В.",
    teacherId: 2,
  },
  {
    id: 1004,
    title: "Подзапросы и оконные функции",
    type: "video",
    authorName: "Кузнецов Д. М.",
    teacherId: 0,
    duration: "35:00",
  },
  {
    id: 1005,
    title: "Индексы в PostgreSQL",
    type: "article",
    authorName: "PostgreSQL",
    teacherId: 0,
    source: true,
  },
  {
    id: 1006,
    title: "Планы выполнения: EXPLAIN",
    type: "lecture",
    authorName: "кафедра",
    teacherId: 0,
    source: true,
  },
  {
    id: 1007,
    title: "SQL: краткая шпаргалка",
    type: "lecture",
    authorName: "Смирнов Д. К.",
    teacherId: 3,
  },
  {
    id: 1008,
    title: "Типовые экзаменационные вопросы",
    type: "lecture",
    authorName: "кафедра",
    teacherId: 0,
    source: true,
  },
  {
    id: 1009,
    title: "Типовые задачи на JOIN",
    type: "lab",
    authorName: "Петров А. В.",
    teacherId: 2,
  },
  {
    id: 1010,
    title: "Самопроверка: нормализация схемы",
    type: "lab",
    authorName: "Иванова М. С.",
    teacherId: 1,
  },
  {
    id: 1011,
    title: "Транзакции и уровни изоляции",
    type: "lecture",
    authorName: "Смирнов Д. К.",
    teacherId: 3,
  },
  {
    id: 1012,
    title: "План повторения перед экзаменом",
    type: "lecture",
    authorName: "кафедра",
    teacherId: 0,
    source: true,
  },
];

export const routeMaterialSources = new Set(
  seeds.filter((seed) => seed.source).map((seed) => seed.id),
);

export const collectionMaterials: DemoMaterial[] = seeds.map((seed) => {
  const video = seed.type === "video";

  return {
    id: seed.id,
    title: seed.title,
    description: `Учебный материал по теме «${seed.title}» для повторения курса и подготовки к экзамену по базам данных.`,
    type: seed.type,
    disciplineId: 1,
    teacherId: seed.teacherId,
    authorName: seed.authorName,
    tags: ["Базы данных", "Подготовка к экзамену"],
    year: 2026,
    course: 3,
    createdAt: "2026-09-20",
    dateLabel: "Добавлено",
    preview: video ? videoCover : undefined,
    duration: seed.duration,
    fileSize: video ? undefined : "1,2 МБ",
  };
});
