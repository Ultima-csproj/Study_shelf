import type { ButtonHTMLAttributes } from "react";
import styles from "./Button.module.css";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "text" | "secondary";
}
export function Button({
  variant = "text",
  className = "",
  type = "button",
  disabled,
  onClick,
  children,
  ...htmlAttributes
}: ButtonProps) {
  const buttonClassName = `${styles.button} ${styles[variant]} ${className}`;

  return (
    <button
      type={type}
      disabled={disabled}
      onClick={onClick}
      className={buttonClassName}
      {...htmlAttributes}
    >
      {children}
    </button>
  );
}
