import { disciplines } from "../data/disciplines";
import { materials } from "../data/materials";
import { teachers } from "../data/teachers";
import { courseMaterialsFor } from "../data/courseMaterials";
import type { DisciplineSummary } from "../types/discipline";
const find = (id: string | number) =>
  disciplines.find(
    (item) => item.slug === String(id) || item.id === Number(id),
  );
export async function getDisciplines(
  signal?: AbortSignal,
): Promise<DisciplineSummary[]> {
  await Promise.resolve();
  signal?.throwIfAborted();
  const order = [1, 5, 2, 4, 7, 3, 6];
  return order.flatMap((id) => {
    const discipline = find(id);
    return discipline
      ? [
          {
            ...discipline,
            materialCount: materials.filter((item) => item.disciplineId === id)
              .length,
            teacher:
              teachers.find((item) => item.id === discipline.teacherId) ?? null,
          },
        ]
      : [];
  });
}
export async function getDiscipline(id: string | number, signal?: AbortSignal) {
  const list = await getDisciplines(signal);
  return (
    list.find((item) => item.slug === String(id) || item.id === Number(id)) ??
    null
  );
}
export async function getDisciplineMaterials(
  id: string | number,
  signal?: AbortSignal,
) {
  await Promise.resolve();
  signal?.throwIfAborted();
  const discipline = find(id);
  return discipline ? courseMaterialsFor(discipline.id) : [];
}
