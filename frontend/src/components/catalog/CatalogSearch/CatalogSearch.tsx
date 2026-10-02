import { SearchInput } from "../../common/SearchInput/SearchInput";

interface CatalogSearchProps {
  value: string;
  onChange: (value: string) => void;
}

export function CatalogSearch({ value, onChange }: CatalogSearchProps) {
  return (
    <SearchInput
      value={value}
      onChange={onChange}
      label="Поиск материалов"
      placeholder="Поиск по названию, теме или автору"
    />
  );
}
