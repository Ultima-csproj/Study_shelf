import { useEffect, useRef } from 'react'
import { Link, useLocation } from 'react-router-dom'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { UserIcon } from '@phosphor-icons/react/dist/csr/User'
import { BookOpenIcon } from '@phosphor-icons/react/dist/csr/BookOpen'
import { GearIcon } from '@phosphor-icons/react/dist/csr/Gear'
import { SignOutIcon } from '@phosphor-icons/react/dist/csr/SignOut'
import { useViewer } from '../../../hooks/useViewer'
import { roleLabels } from '../../../utils/viewerLabels'
import styles from './AccountMenu.module.css'

export function AccountMenu() {
  const { viewer, signOut } = useViewer()
  const location = useLocation()
  const menu = useRef<HTMLDetailsElement>(null)
  const trigger = useRef<HTMLElement>(null)

  useEffect(() => {
    if (menu.current) {
      menu.current.open = false
    }
  }, [location.pathname, location.search])

  useEffect(() => {
    const closeOutside = (event: PointerEvent) => {
      if (
        event.target instanceof Node &&
        !menu.current?.contains(event.target)
      ) {
        if (menu.current) {
          menu.current.open = false
        }
      }
    }

    document.addEventListener('pointerdown', closeOutside)

    return () => {
      document.removeEventListener('pointerdown', closeOutside)
    }
  }, [])

  if (viewer.kind !== 'authenticated') {
    return null
  }

  const user = viewer.user
  const role = roleLabels[user.role]

  return (
    <details
      ref={menu}
      className={styles.menu}
      onKeyDown={(event) => {
        if (event.key === 'Escape' && menu.current) {
          menu.current.open = false
          trigger.current?.focus()
        }
      }}
    >
      <summary
        ref={trigger}
        className={styles.trigger}
        aria-label={`Личная зона: ${user.name}`}
      >
        <span className={styles.avatar}>{user.initials}</span>
        <span className={styles.name}>{user.name}</span>
        <CaretDownIcon
          size={14}
          aria-hidden="true"
        />
      </summary>
      <div className={styles.dropdown}>
        <div className={styles.identity}>
          <span className={styles.avatar}>{user.initials}</span>
          <div>
            <strong>{user.name}</strong>
            <p>{role}</p>
          </div>
        </div>
        <nav aria-label="Личная зона аккаунта">
          <Link to="/profile">
            <UserIcon
              size={22}
              aria-hidden="true"
            />
            Профиль
          </Link>
          <Link to="/library">
            <BookOpenIcon
              size={22}
              aria-hidden="true"
            />
            Моя библиотека
          </Link>
          <Link to="/settings">
            <GearIcon
              size={22}
              aria-hidden="true"
            />
            Настройки
          </Link>
        </nav>
        <button
          type="button"
          className={styles.signOut}
          onClick={signOut}
        >
          <SignOutIcon
            size={22}
            aria-hidden="true"
          />
          Выйти
        </button>
      </div>
    </details>
  )
}
