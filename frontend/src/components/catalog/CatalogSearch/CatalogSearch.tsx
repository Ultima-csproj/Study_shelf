import { SearchInput } from "../../common/SearchInput/SearchInput";

export function CatalogSearch({
  value,
  onChange,
}: {
  value: string;
  onChange: (value: string) => void;
}) {
  return (
    <SearchInput
      value={value}
      onChange={onChange}
      label="Поиск материалов"
      placeholder="Поиск по названию, теме или автору"
    />
  );
}
