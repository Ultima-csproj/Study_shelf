import type { CourseMaterial, MaterialSectionData } from '../types/material'
import { materials } from './materials'
import { materialTypeLabels } from '../utils/materialLabels'
import { teachers } from './teachers'
const sections: Omit<MaterialSectionData, 'items'>[] = [
  {
    id: 'start',
    title: 'С чего начать',
    description: 'Базовые материалы, которые помогут быстро войти в курс.',
  },
  {
    id: 'lectures',
    title: 'Лекции и конспекты',
    mobileDescription: 'Презентации и дополнительные материалы.',
    description:
      'Полный курс лекций с презентациями и дополнительными материалами.',
  },
  {
    id: 'practice',
    title: 'Практика',
    mobileDescription: 'Лабораторные работы и задания.',
    description: 'Лабораторные работы, задания и методические указания.',
  },
]
const databaseEntries = [
  {
    id: 6,
    section: 'start',
    title: 'Введение в реляционные базы данных',
    summary: 'Основные понятия и архитектура реляционных СУБД.',
    formatLabel: 'Видео',
    contributor: 'Иванова М. С.',
    addedAt: '2026-09-12',
    description:
      'Основные понятия и архитектура реляционных СУБД. Примеры и терминология.',
  },
  {
    id: 4,
    section: 'lectures',
    title: 'Лекция 7. Нормальные формы',
    summary: 'Первая, вторая и третья нормальные формы.',
    formatLabel: 'Презентация (PDF)',
    contributor: 'Иванова М. С.',
    addedAt: '2026-09-21',
    fileSize: '2,4 МБ',
    previewKind: 'presentation',
    description:
      'Первая, вторая и третья нормальные формы. Примеры декомпозиции.',
  },
  {
    id: 7,
    section: 'lectures',
    title: 'Конспект лекций (все темы)',
    formatLabel: 'Документ (PDF)',
    contributor: 'Иванова М. С.',
    addedAt: '2026-09-10',
    fileSize: '6,8 МБ',
    previewKind: 'document',
    description:
      'Полный конспект лекций по дисциплине с примерами и ключевыми определениями.',
  },
  {
    id: 5,
    section: 'practice',
    title: 'Лабораторная работа 4. SQL-запросы',
    formatLabel: 'Методические указания',
    contributor: 'Иванова М. С.',
    addedAt: '2026-09-25',
    fileSize: '1,1 МБ',
    description:
      'Запросы на выборку, фильтрация, объединения таблиц. Задания и критерии оценки.',
  },
  {
    id: 1,
    section: 'practice',
    title: 'Базы данных. Теория и практика',
    formatLabel: 'Учебник',
    contributor: 'А. В. Кузнецов',
    addedAt: '2026-09-01',
    publicationYear: 2022,
    fileSize: '12,3 МБ',
    description:
      'Современный и подробный курс по реляционным и нереляционным базам данных.',
  },
] satisfies ({ id: number; section: MaterialSectionData['id'] } & Omit<
  CourseMaterial,
  'material' | 'number'
>)[]
export function courseMaterialsFor(
  disciplineId: number,
): MaterialSectionData[] {
  if (disciplineId === 1) {
    return sections.map((section) => ({
      ...section,
      items: databaseEntries.flatMap((entry, index) => {
        const material = materials.find((item) => item.id === entry.id)
        return entry.section === section.id && material
          ? [{ ...entry, material, number: index + 1 }]
          : []
      }),
    }))
  }
  const selected = materials.filter(
    (item) => item.disciplineId === disciplineId,
  )
  let number = 0
  return sections.map((section, sectionIndex) => ({
    ...section,
    items: selected
      .filter((material, index) =>
        sectionIndex === 0
          ? index === 0
          : sectionIndex === 1
            ? index > 0 && material.type !== 'lab' && material.type !== 'book'
            : index > 0 &&
              (material.type === 'lab' || material.type === 'book'),
      )
      .map((material): CourseMaterial => ({
        material,
        number: ++number,
        title: material.title,
        description: material.description,
        formatLabel: materialTypeLabels[material.type],
        contributor:
          teachers.find((teacher) => teacher.id === material.teacherId)
            ?.shortName ?? 'Автор не указан',
        addedAt: material.createdAt,
        fileSize: material.fileSize,
      })),
  }))
}
