import { Link, useNavigate, useSearchParams } from 'react-router-dom'
import { useViewer } from '../../hooks/useViewer'
import { safeRedirect } from '../../utils/redirect'
import styles from '../InformationPage/InformationPage.module.css'

export function LoginPage() {
  const [params] = useSearchParams()
  const navigate = useNavigate()
  const { viewer, signInDemo } = useViewer()
  const redirect = safeRedirect(params.get('redirect'))
  const redirectPathname = new URL(redirect, 'https://study-shelf.local')
    .pathname
  const personalPage = ['/profile', '/library', '/settings'].includes(
    redirectPathname,
  )
  const backPath = personalPage ? '/catalog' : redirect
  const register = params.get('mode') === 'register'
  const title = register ? 'Создать аккаунт' : 'Войти'
  const buttonLabel =
    viewer.kind === 'authenticated'
      ? 'Продолжить просмотр'
      : 'Войти в демо-аккаунт'
  const enter = () => {
    signInDemo()
    navigate(redirect, { replace: true })
  }
  return (
    <main
      id="main-content"
      className={styles.page}
    >
      <Link
        className={styles.back}
        to={backPath}
      >
        ← Вернуться к просмотру
      </Link>
      <h1>{title}</h1>
      <p>
        Каталог и материалы доступны без аккаунта. Вход позволит сохранять
        материалы и пользоваться личными подборками.
      </p>
      <div className={styles.notice}>
        <h2>Демонстрационный вход</h2>
        <p>
          Сейчас можно посмотреть интерфейс от лица Алексея К. Настоящий вход и
          регистрация появятся после подключения сервера.
        </p>
        <button
          className={styles.primary}
          type="button"
          onClick={enter}
        >
          {buttonLabel}
        </button>
      </div>
    </main>
  )
}
