import type { ButtonHTMLAttributes } from "react";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "text" | "primary" | "secondary" | "ghost" | "danger";
  size?: "regular" | "compact";
  loading?: boolean;
}
export function Button({
  variant = "text",
  size = "regular",
  loading = false,
  className = "",
  type = "button",
  disabled,
  onClick,
  children,
  ...htmlAttributes
}: ButtonProps) {
  const buttonClassName = `ui-button ${className}`.trim();

  return (
    <button
      type={type}
      disabled={disabled || loading}
      aria-busy={loading || undefined}
      onClick={onClick}
      className={buttonClassName}
      {...htmlAttributes}
      data-variant={variant}
      data-size={size}
    >
      {loading && <span className="ui-button-spinner" aria-hidden="true" />}
      {children}
    </button>
  );
}
