export type HighlightColor = "yellow" | "green" | "blue" | "pink" | "orange";

export type Theme = "gruvbox" | "nordic" | string;

export type FileType = "pdf" | "epub" | "mobi" | string;

export type BookType = "book" | "paper" | "academic-paper" | string;

export enum ReadStatus {
    Read,
    Unread,
    InProgress,
}

export interface Progress {
    page:       number;
    percentage: number;
    updated_at: number;
}

// interfaces

export interface AcademicMetadata {
    university?: string;
    doi?:        string;
    journal?:    string;
    volume?:     string;
    issue?:      string;
}

// Dependant:

export interface Config {
    theme:             Theme;
    custom_genres:     string[];
    calm_mode_default: boolean;
    data_dir:          string;
}

export interface Note {
    id:         string;
    page:       number;
    text:       string;
    color:      HighlightColor;
    created_at: number;
    updated_at: number;
}

export interface Book {
    id:            string;
    title:         string;
    authors:       string[];
    genre:         string;
    language:      string;
    year:          number | null;
    pages:         number | null;
    cover_path:    string | null;
    file_path:     string;
    file_type:     FileType;
    book_type:     BookType;
    read:          boolean;
    progress:      Progress;
    notes:         Note[];
    academic_meta: AcademicMetadata | null;
}

export interface FilterOptions {
    query?:       string;
    genre?:       string;
    language?:    string;
    book_type?:   BookType;
    read_status?: ReadStatus;
    year_range?:  [number, number];
    page_range?:  [number, number];
}
