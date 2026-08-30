import { Option } from "fp-ts/Option";

// ---------- basics ----------

export enum Theme {
    Gruvbox = "gruvbox",
    Nordic = "nordic",
}

export enum FileType {
    Pdf = "pdf",
    Epub = "epub",
    Mobi = "mobi",
}

export enum BookType {
    Book = "book",
    Paper = "paper",
    AcademicPaper = "academic-paper",
}

export interface Progress {
    page:       number;
    percentage: number;
    updated_at: number;
}

export enum HighlightColor {
    Yellow = "yellow",
    Green = "green",
    Blue = "blue",
    Pink = "pink",
    Orange = "orange",
    Red = "red",
}

export enum ReadStatus {
    Read = "read",
    Unread = "unread",
    InProgress = "in-progress",
}

// ---------- config ----------

export interface Config {
    theme:             Theme;
    custom_genres:     string[];
    calm_mode_default: boolean;
    data_dir:          string;
}

// ---------- notes ----------

export interface Note {
    id:         string;
    page:       string;
    text:       string;
    color:      HighlightColor;
    created_at: number;
    updated_at: number;
}

// ---------- metadata ----------

export interface AcademicMetadata {
    institution: Option<string>;
    doi:         Option<string>;
    journal:     Option<string>;
    volume:      Option<string>;
    issue:       Option<string>;
}

// ---------- book ----------

export interface Book {
    id:            string;
    title:         string;
    authors:       string;
    genres:        string[];
    language:      string;
    year:          Option<number>;
    pages:         Option<number>;
    cover_path:    Option<string>;
    file_path:     string;
    file_type:     FileType;
    book_type:     BookType;
    read:          boolean;
    progress:      Progress;
    notes:         Note[];
    academic_meta: Option<AcademicMetadata>;
    status:        ReadStatus;
}

// ---------- filters ----------

export interface FilterOptions {
    query:       Option<string>;
    genre:       Option<string>;
    language:    Option<string>;
    book_type:   Option<BookType>;
    read_status: Option<ReadStatus>;
    year_range:  Option<[number, number]>;
    page_range:  Option<[number, number]>;
}

// ---------- api payloads ----------

export interface AddBookPayload {
    file_path:     string;
    title:         Option<string>;
    authors:       Option<string[]>;
    genre:         Option<string>;
    language:      Option<string>;
    year:          Option<number>;
    book_type:     BookType;
    academic_meta: Option<AcademicMetadata>;
}

export interface UpdateBookPayload {
    title:         Option<string>;
    authors:       Option<string[]>;
    genre:         Option<string>;
    language:      Option<string>;
    year:          Option<number>;
    book_type:     Option<BookType>;
    academic_meta: Option<AcademicMetadata>;
    read:          Option<boolean>;
}

// ---------- errors ----------

export type AppError =
    | { kind: "Network"; message: string }
    | { kind: "FileSystem"; message: string }
    | { kind: "Validation"; message: string }
    | { kind: "NotFound"; message: string }
    | { kind: "Unknown"; message: string };
