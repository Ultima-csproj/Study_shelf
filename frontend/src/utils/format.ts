export function materialCount(count: number) {
  const lastTwo = count % 100
  const last = count % 10
  const noun =
    lastTwo >= 11 && lastTwo <= 14
      ? 'материалов'
      : last === 1
        ? 'материал'
        : last >= 2 && last <= 4
          ? 'материала'
          : 'материалов'
  return `${count} ${noun}`
}

export function foundMaterials(count: number) {
  return `${count % 10 === 1 && count % 100 !== 11 ? 'Найден' : 'Найдено'} ${materialCount(count)}`
}

export function durationMinutes(duration: string) {
  const parts = duration.split(':').map(Number)
  if (parts.length === 3) {
    return `${parts[0] * 60 + parts[1]} мин`
  }
  return `${parts[0]} мин`
}
