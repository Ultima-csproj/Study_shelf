import type {
  Collection,
  CollectionContributor,
  CollectionSection,
} from "../types/collection";

const ivanova: CollectionContributor = {
  identity: { kind: "teacher", teacherId: 1, displayName: "Иванова М. С." },
  role: "curator",
  position: 1,
};

const petrov: CollectionContributor = {
  identity: { kind: "teacher", teacherId: 2, displayName: "Петров А. В." },
  role: "author",
  position: 2,
};

const smirnov: CollectionContributor = {
  identity: { kind: "teacher", teacherId: 3, displayName: "Смирнов Д. К." },
  role: "contributor",
  position: 2,
};

const student: CollectionContributor = {
  identity: { kind: "user", userId: 1, displayName: "Алексей К." },
  role: "contributor",
  position: 2,
};

function section(
  id: string,
  title: string,
  position: number,
  materialIds: number[],
): CollectionSection {
  return {
    id,
    title,
    position,
    items: materialIds.map((materialId, index) => ({
      id: materialId,
      materialId,
      position: index + 1,
    })),
  };
}

export const collections: Collection[] = [
  {
    id: 1,
    slug: "database-exam",
    title: "Подготовка к экзамену по базам данных",
    shortDescription:
      "Основные материалы по темам экзамена: нормализация, SQL, индексы и транзакции.",
    description:
      "Нормализация, SQL, индексы и транзакции — от базовых тем к экзаменационным вопросам.",
    purpose:
      "Повторить ключевые темы курса перед экзаменом. Материалы расположены от базовых понятий к более сложным задачам.",
    targetAudience: "Студенты 3 курса, изучающие базы данных.",
    estimatedTime: "2–3 часа",
    difficulty: "intermediate",
    disciplineId: 1,
    course: 3,
    isPublic: true,
    createdAt: "2026-09-26",
    updatedAt: "2026-09-28",
    contributors: [ivanova, petrov],
    sections: [
      section("basics", "Повторить основы", 1, [1001, 1002]),
      section("sql", "SQL", 2, [1003, 1004]),
      section("performance", "Производительность", 3, [1005, 1006]),
      section(
        "exam",
        "Перед экзаменом",
        4,
        [1007, 1008, 1009, 1010, 1011, 1012],
      ),
    ],
  },
  {
    id: 2,
    slug: "sql-before-exam",
    title: "SQL перед экзаменом",
    shortDescription:
      "Быстрое повторение SELECT, JOIN, GROUP BY и подзапросов с примерами.",
    description: "Быстрое повторение основных конструкций SQL.",
    purpose:
      "Повторить SELECT, JOIN, GROUP BY, подзапросы и оконные функции перед экзаменом.",
    targetAudience: "Студенты 3 курса, знакомые с основами SQL.",
    estimatedTime: "2 часа",
    difficulty: "intermediate",
    disciplineId: 1,
    course: 3,
    isPublic: true,
    createdAt: "2026-09-24",
    updatedAt: "2026-09-27",
    contributors: [ivanova, smirnov],
    sections: [
      section("queries", "Основные запросы", 1, [1001, 1003, 1009]),
      section(
        "advanced",
        "Подзапросы и производительность",
        2,
        [1004, 1005, 1006],
      ),
      section("revision", "Повторение", 3, [1007, 1008]),
    ],
  },
  {
    id: 3,
    slug: "database-design",
    title: "Практика проектирования баз данных",
    shortDescription:
      "От модели предметной области до схемы и нормальных форм.",
    description:
      "Практический маршрут по проектированию реляционных баз данных.",
    purpose:
      "Научиться переводить требования предметной области в схему базы данных и проверять её нормализацию.",
    targetAudience:
      "Студенты 3 курса, выполняющие практические работы по базам данных.",
    estimatedTime: "3 часа",
    difficulty: "intermediate",
    disciplineId: 1,
    course: 3,
    isPublic: true,
    createdAt: "2026-09-22",
    updatedAt: "2026-09-26",
    contributors: [{ ...petrov, position: 1 }, student],
    sections: [
      section("model", "Модели и нормализация", 1, [1001, 1002, 1010]),
      section(
        "implementation",
        "Практика и самопроверка",
        2,
        [1003, 1009, 1008],
      ),
    ],
  },
  {
    id: 4,
    slug: "indexes-performance",
    title: "Индексы и производительность",
    shortDescription:
      "Разбор индексов и планов выполнения запросов PostgreSQL.",
    description:
      "Как находить медленные запросы и выбирать подходящие индексы.",
    purpose:
      "Научиться читать план выполнения запроса и понимать, когда индекс помогает ускорить доступ к данным.",
    targetAudience:
      "Студенты 3 курса и начинающие разработчики, знакомые с SQL.",
    estimatedTime: "1–2 часа",
    difficulty: "advanced",
    disciplineId: 1,
    course: 3,
    isPublic: true,
    createdAt: "2026-09-20",
    updatedAt: "2026-09-25",
    contributors: [{ ...smirnov, role: "curator", position: 1 }],
    sections: [
      section("indexes", "Индексы и планы выполнения", 1, [1005, 1006]),
      section("practice", "Практика оптимизации", 2, [1003, 1004, 1011]),
    ],
  },
];
