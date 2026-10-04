CREATE TABLE teachers (
    id         INTEGER PRIMARY KEY CHECK (id > 0),
    name       TEXT NOT NULL CHECK (length(trim(name)) > 0),
    short_name TEXT NOT NULL CHECK (length(trim(short_name)) > 0)
);

CREATE TABLE disciplines (
    id          INTEGER PRIMARY KEY CHECK (id > 0),
    slug        TEXT NOT NULL UNIQUE CHECK (length(trim(slug)) > 0),
    name        TEXT NOT NULL CHECK (length(trim(name)) > 0),
    description TEXT NOT NULL DEFAULT '',
    summary     TEXT,
    course      INTEGER NOT NULL CHECK (course BETWEEN 1 AND 255),
    semester    INTEGER NOT NULL CHECK (semester BETWEEN 1 AND 255),
    teacher_id  INTEGER NOT NULL
        REFERENCES teachers(id) ON DELETE RESTRICT
);

CREATE TABLE materials (
    id               INTEGER PRIMARY KEY CHECK (id > 0),
    title            TEXT NOT NULL CHECK (length(trim(title)) > 0),
    description      TEXT NOT NULL DEFAULT '',
    material_type    TEXT NOT NULL CHECK (
        material_type IN ('lecture', 'book', 'article', 'video', 'lab')
    ),
    discipline_id    INTEGER NOT NULL
        REFERENCES disciplines(id) ON DELETE RESTRICT,
    teacher_id       INTEGER NOT NULL
        REFERENCES teachers(id) ON DELETE RESTRICT,
    course           INTEGER NOT NULL CHECK (course BETWEEN 1 AND 255),
    year             INTEGER NOT NULL,
    created_at       TEXT NOT NULL CHECK (length(created_at) > 0),
    updated_at       TEXT,
    author_name      TEXT,
    preview_url      TEXT,
    duration_seconds INTEGER CHECK (
        duration_seconds BETWEEN 0 AND 4294967295
    ),
    file_size_bytes  INTEGER CHECK (file_size_bytes >= 0)
);

CREATE TABLE material_tags (
    material_id INTEGER NOT NULL
        REFERENCES materials(id) ON DELETE CASCADE,
    tag         TEXT NOT NULL CHECK (length(trim(tag)) > 0),
    PRIMARY KEY (material_id, tag)
);

CREATE INDEX idx_disciplines_teacher
    ON disciplines(teacher_id);

CREATE INDEX idx_materials_discipline
    ON materials(discipline_id);

CREATE INDEX idx_materials_teacher
    ON materials(teacher_id);

CREATE INDEX idx_materials_created
    ON materials(created_at DESC, id DESC);

CREATE INDEX idx_material_tags_tag
    ON material_tags(tag, material_id);
