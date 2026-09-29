import { useId } from 'react'
import { MagnifyingGlassIcon } from '@phosphor-icons/react/dist/csr/MagnifyingGlass'
import { XIcon } from '@phosphor-icons/react/dist/csr/X'
import styles from './SearchInput.module.css'

interface SearchInputProps {
  value: string
  onChange: (value: string) => void
  label: string
  placeholder: string
  compact?: boolean
}
export function SearchInput({
  value,
  onChange,
  label,
  placeholder,
  compact = false,
}: SearchInputProps) {
  const id = useId()
  const className = `${styles.field} ${compact ? styles.compact : ''}`
  return (
    <div className={className}>
      <label
        className="visually-hidden"
        htmlFor={id}
      >
        {label}
      </label>
      <MagnifyingGlassIcon
        size={19}
        aria-hidden="true"
        className={styles.icon}
      />
      <input
        id={id}
        type="search"
        value={value}
        onChange={(event) => onChange(event.target.value)}
        placeholder={placeholder}
        autoComplete="off"
      />
      {value && (
        <button
          type="button"
          className={styles.clear}
          onClick={() => onChange('')}
          aria-label={`Очистить: ${label}`}
        >
          <XIcon
            size={16}
            aria-hidden="true"
          />
        </button>
      )}
    </div>
  )
}
