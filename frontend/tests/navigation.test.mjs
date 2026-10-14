import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
import ts from "typescript";

// Run the actual TypeScript utilities without adding a test runtime dependency.
async function moduleUrl(file, imports = {}) {
  const source = await readFile(new URL(file, import.meta.url), "utf8");
  let { outputText } = ts.transpileModule(source, {
    compilerOptions: {
      module: ts.ModuleKind.ESNext,
      target: ts.ScriptTarget.ES2022,
    },
  });
  for (const [specifier, url] of Object.entries(imports)) {
    outputText = outputText.replaceAll(
      JSON.stringify(specifier),
      JSON.stringify(url),
    );
  }
  return `data:text/javascript;base64,${Buffer.from(outputText).toString("base64")}`;
}

const navigationUrl = await moduleUrl("../src/utils/navigation.ts");
const {
  authorListPath,
  parseNavigationOrigin,
  withNavigationOrigin,
  returnDestination,
} = await import(navigationUrl);
const { materialNavigation } = await import(
  await moduleUrl("../src/utils/materialNavigation.ts", {
    "./navigation": navigationUrl,
  })
);
const search = (path) => new URL(path, "https://study-shelf.local").search;

for (const origin of [
  "/profile",
  "/profile?tab=publications&page=3",
  "/library?tab=collections&page=2",
  "/settings#institution-membership",
  "/author/materials?state=published&q=Rust&order=title&page=2",
  "/library/collections?page=2",
  "/catalog?type=book&page=4",
  "/disciplines?page=2",
  "/",
]) {
  test(`material returns to exact origin: ${origin}`, () => {
    const material = withNavigationOrigin("/materials/42", origin);
    const parsed = parseNavigationOrigin(
      new URLSearchParams(search(material)).get("from"),
    );
    assert.equal(materialNavigation(parsed, null, null).backPath, origin);
  });
}

test("profile publications → author list → workspace → publication, each level returns to its parent", () => {
  const profile = "/profile?tab=publications&page=3";
  const list = withNavigationOrigin(
    "/author/materials?state=draft&q=Rust&page=2",
    profile,
  );
  const workspace = withNavigationOrigin(
    "/author/materials/42?revision=7&page=2",
    list,
  );
  const publication = withNavigationOrigin("/materials/42", workspace);
  assert.equal(
    returnDestination(search(publication), "/catalog").path,
    workspace,
  );
  assert.equal(
    returnDestination(search(workspace), "/author/materials").path,
    list,
  );
  assert.equal(returnDestination(search(list), "/profile").path, profile);
});

test("create cancellation and created workspace return to entry, not a fresh creation form", () => {
  for (const parent of [
    "/profile",
    "/profile?tab=publications",
    "/author/materials?state=draft&page=2",
  ]) {
    const create = withNavigationOrigin("/author/materials/new", parent);
    const back = returnDestination(search(create), "/author/materials");
    const workspace = withNavigationOrigin(
      "/author/materials/43?revision=1",
      back.path,
    );
    assert.equal(back.path, parent);
    assert.equal(
      returnDestination(search(workspace), "/author/materials").path,
      parent,
    );
  }
});

test("profile editor preserves settings anchor and publication page", () => {
  for (const origin of [
    "/settings#institution-membership",
    "/profile?tab=publications&page=2",
  ]) {
    const edit = withNavigationOrigin("/profile?edit=1", origin);
    assert.equal(returnDestination(search(edit), "/profile").path, origin);
  }
});

test("direct links and rejected origins have deterministic safe fallbacks", () => {
  for (const value of [
    null,
    "",
    "https://evil.test/profile",
    "//evil.test/profile",
    "/\\evil.test/profile",
    "/login",
    "/author/materials/new",
    "/profile\n",
  ]) {
    assert.equal(parseNavigationOrigin(value), null);
  }
  assert.deepEqual(returnDestination("", "/author/materials"), {
    path: "/author/materials",
    label: "Мои материалы",
  });
  assert.equal(
    returnDestination("?from=https://evil.test", "/profile").path,
    "/profile",
  );
});

test("target query and anchor survive linking; labels distinguish overview and publications", () => {
  const linked = withNavigationOrigin(
    "/materials/42?source=mock#content",
    "/profile?tab=publications",
  );
  const url = new URL(linked, "https://study-shelf.local");
  assert.equal(url.searchParams.get("source"), "mock");
  assert.equal(url.hash, "#content");
  assert.equal(returnDestination(url.search, "/profile").label, "Публикации");
  assert.equal(returnDestination("", "/profile").label, "Обзор профиля");
});

test("account menu keeps author list context without nesting a list inside itself", () => {
  const path = withNavigationOrigin(
    "/author/materials?state=draft&page=2",
    "/profile?tab=publications",
  );
  const location = new URL(path, "https://study-shelf.local");
  assert.equal(authorListPath(location), path);
  const workspace = new URL(
    withNavigationOrigin("/author/materials/42", path),
    location,
  );
  assert.equal(authorListPath(workspace), path);
  const material = new URL("/materials/42?from=%2Flibrary", location);
  const list = authorListPath(material);
  assert.equal(
    returnDestination(search(list), "/profile").path,
    "/materials/42?from=%2Flibrary",
  );
});
