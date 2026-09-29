import type { ProfileInput, User } from '../types/viewer'

export async function updateProfile(
  user: User,
  input: ProfileInput,
  signal?: AbortSignal,
): Promise<User> {
  await Promise.resolve()
  signal?.throwIfAborted()

  const firstName = input.firstName.trim()
  const lastName = input.lastName.trim()
  const group = input.group.trim()

  if (!firstName || !lastName) {
    throw new Error('Укажите имя и фамилию.')
  }

  if (firstName.length > 64 || lastName.length > 64 || group.length > 30) {
    throw new Error('Имя и фамилия — до 64 символов, группа — до 30.')
  }

  if (
    input.course !== null &&
    (!Number.isInteger(input.course) || input.course < 1 || input.course > 6)
  ) {
    throw new Error('Выберите курс от 1 до 6.')
  }

  return {
    ...user,
    firstName,
    lastName,
    course: input.course,
    group,
    name: `${firstName} ${lastName[0]}.`,
    initials: `${firstName[0]}${lastName[0]}`.toLocaleUpperCase('ru'),
  }
}
